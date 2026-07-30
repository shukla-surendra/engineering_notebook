# Prerequisite Concepts, Part 10: The Physics of Persistence (B-Trees vs. LSM-Trees)

[Part 6](06_mechanical_sympathy_and_physics_of_latency.md) already made the core physical
argument — random disk access costs ~9 ms of pure mechanical waiting, sequential access
doesn't — and used it to sketch, in one paragraph, why Cassandra and RocksDB append writes
instead of updating in place. [Part 2](02_data_and_consistency.md) separately named
**B-tree indexes** as the default structure behind "how does a database find a row without
scanning everything." Neither part actually finished the argument: Part 6 never explains
what a B-tree's write path costs that an LSM-tree's doesn't, and Part 2 never explains what
happens on the *write* side of an index at all. **This part finishes both**, then adds the
two mechanisms every storage engine — B-tree or LSM-tree, relational or "NoSQL" — depends
on to survive a crash: **fsync** (the physical durability boundary) and the **write-ahead
log** (the trick that makes durability affordable). It closes by showing that "SQL vs.
NoSQL" is largely the wrong axis for this discussion — the real axis, the one that actually
determines performance, is B-tree vs. LSM-tree, and databases on both sides of the SQL
label pick from the same two structures.

## Recap: What Parts 2 and 6 Already Own, and What This Part Adds

| Question | Earlier part's answer | This part's answer |
|---|---|---|
| Why do databases use a separate structure instead of scanning every row? | [Part 2](02_data_and_consistency.md#indexing-why-databases-dont-scan-everything): "a B-tree index, O(log n)" | The actual page-based structure, its write path, and why that write path is expensive |
| Why do Cassandra/RocksDB append instead of updating in place? | [Part 6](06_mechanical_sympathy_and_physics_of_latency.md#cassandra-lsm-trees-turning-random-writes-into-sequential-ones): "an LSM-tree, sequential writes" | The full mechanism — memtable, WAL, SSTable, compaction — and what it costs on the *read* side in exchange |
| What guarantees does a committed transaction actually have? | [Part 2](02_data_and_consistency.md#acid-vs-base): "**D**urability — once committed, a write survives a crash" | The physical mechanism that makes that guarantee true: `fsync` and the write-ahead log |
| Is "NoSQL" one kind of database? | Not covered | No — it's a data-model label sitting on top of the same two storage-engine choices this doc covers, chosen independently |

## The Real Question Every Storage Engine Is Answering

Strip away the marketing on any database product and it's making one physical bet: **data
lives on a medium where random writes are expensive and sequential writes are cheap** (Part
6's ~9 ms seek-and-rotate tax, or flash's write-amplification tax), **so every persistent
data structure has to decide where to pay that cost** — on the write path, the read path,
or some blend of both. There is no structure that makes both free simultaneously; this is
the central, unavoidable trade-off the rest of this doc explores from both directions.

## B-Trees, Fully Unpacked: Optimizing for Reads by Paying on Writes

A **B-tree** (the structure behind Postgres and MySQL/InnoDB's default engine, SQLite,
MongoDB's default WiredTiger engine, and most traditional relational databases) is a
balanced, sorted, n-ary tree of fixed-size **pages** — typically 4-16 KB, deliberately sized
to match the OS's own page size so a "read one B-tree page" operation is exactly "read one
disk block," no partial reads or wasted extra I/O.

```mermaid
flowchart TD
    R["Root page\n(keys: 50, 150)"] --> A["Page A\nkeys < 50"]
    R --> B["Page B\n50 <= keys < 150"]
    R --> C["Page C\nkeys >= 150"]
    B --> B1["Leaf page\nkeys 50-99\n+ row data"]
    B --> B2["Leaf page\nkeys 100-149\n+ row data"]
```

**The read path**: start at the root, compare the target key against the page's sorted keys,
follow the matching child pointer, repeat until a leaf page holds the actual row (or a
pointer to it). A tree of fan-out *b* holding *n* keys has depth **log_b(n)** — with a
realistic fan-out in the hundreds (many keys fit in one 4-16 KB page), a billion-row table
is typically only **3-4 levels deep**, so a lookup costs 3-4 page reads, not 3-4 seeks
through a huge structure. This is genuinely fast, and — because rows live sorted in place —
a **range query** (`WHERE created_at BETWEEN X AND Y`, exactly [Part 2's range-query
point](02_data_and_consistency.md#indexing-why-databases-dont-scan-everything)) is just
"find the start, then read the next several leaf pages in sequence," no extra work per row.

**The write path is where the cost Part 6 gestured at actually lives.** Updating a row means:

1. Find the leaf page holding that key (a read, same as above).
2. **Modify that page in place** — and because the row could be *any* key, the page it
   lives on could be anywhere on disk. This is, mechanically, exactly the random-access
   pattern [Part 6](06_mechanical_sympathy_and_physics_of_latency.md#random-vs-sequential-access-on-a-physical-disk)
   describes: an update to key 50 and an update to key 8 million touch two unrelated
   physical locations, each paying its own seek.
3. If the page is now full, it **splits** into two pages, and the split has to be recorded
   in the *parent* page too — occasionally cascading multiple levels up the tree. A single
   logical row insert can therefore turn into several page writes, not one.

**Why this is the "read-optimized" side of the trade-off, precisely stated**: a B-tree keeps
exactly one physical copy of each row, always up to date, always in one findable location —
which is *why* reads are so cheap (nothing to reconcile, nothing to search for across
multiple copies) — but that same property means every write has to find and mutate that one
location immediately, in place, wherever it happens to be. The read path's simplicity and
the write path's random-I/O cost are the same design decision, seen from two sides.

**A mitigation worth naming**: copy-on-write B-trees (e.g., LMDB) never modify a page in
place at all — a write copies the affected page and every ancestor up to a new root,
leaving the old tree fully intact and readable until the new root is atomically swapped in.
This trades some extra write volume (copying ancestor pages) for a genuinely different
durability story (the old tree is always a valid, complete snapshot — no WAL-based crash
recovery needed, a point the WAL section below returns to), but it does not remove the
random-write cost of the standard mutable B-tree this doc otherwise describes.

## LSM-Trees, Fully Unpacked: Optimizing for Writes by Paying on Reads

[Part 6 already gave the one-paragraph version](06_mechanical_sympathy_and_physics_of_latency.md#cassandra-lsm-trees-turning-random-writes-into-sequential-ones)
of the **Log-Structured Merge-tree** — the structure behind Cassandra, RocksDB, LevelDB,
HBase, and most write-heavy stores. Here is the full mechanism it referenced but didn't
walk through:

```mermaid
flowchart TD
    W["Write: SET key=value"] --> WAL["1. Append to WAL (disk, sequential)"]
    WAL --> MT["2. Insert into memtable (RAM, sorted)"]
    MT -->|memtable full| FLUSH["3. Flush memtable to disk as an immutable SSTable"]
    FLUSH --> L0["SSTable (Level 0)"]
    L0 -->|background compaction| L1["Merged, sorted SSTable (Level 1)"]
```

1. **Write-ahead log (WAL) first.** Before anything else, the write is appended to a
   sequential on-disk log — purely for crash recovery, covered in its own section below.
2. **Memtable.** The write is also applied to an in-memory sorted structure (commonly a
   skip list or balanced tree) — this is what makes the write instantly queryable and
   costs no disk I/O at all beyond the WAL append.
3. **Flush to an SSTable.** Once the memtable reaches a size threshold, it's written to
   disk as a **Sorted String Table (SSTable)** — an immutable file, written once,
   start-to-finish, sequentially. Immutable is the operative word: an SSTable is never
   edited again, only read, merged, or eventually deleted.
4. **Compaction.** A background process periodically merges multiple SSTables into fewer,
   larger ones — dropping overwritten and deleted (**tombstoned**) values in the process —
   using a sequential merge-sort-style read-and-rewrite, never random access.

**The insight [Part 6](06_mechanical_sympathy_and_physics_of_latency.md#cassandra-lsm-trees-turning-random-writes-into-sequential-ones)
already named, restated precisely**: the write is *logically* random (an arbitrary key) but
*physically* sequential (always appended at the tail of the WAL, then the tail of a new
SSTable) — the engine has fully decoupled "where this key logically sorts" from "where
this byte physically lands," and paid for that decoupling by giving up "one row, one
physical location" entirely. A single key's history can now be scattered across the
memtable and several SSTables simultaneously, which is exactly where this structure's cost
shows up.

### The Read Path: Where the LSM-Tree's Bill Comes Due

Reading key *K* means: check the memtable, then check each SSTable that might contain *K*
— newest first, since a later SSTable's value for the same key supersedes an older one —
until a match is found. Without help, that's a linear scan across every SSTable the engine
has ever flushed, which would make reads scale *worse* over time as more SSTables
accumulate. Two structures exist specifically to keep this bounded:

- **Bloom filters** — a compact, per-SSTable probabilistic structure that answers "is key
  *K* definitely absent from this file?" in O(1) with zero false negatives (it can say
  "maybe present" incorrectly, never "absent" incorrectly). Checking a bloom filter first
  lets the engine skip opening most SSTables entirely for a given lookup, at the cost of a
  small, fixed amount of extra memory per SSTable.
- **Sparse indexes** — since each SSTable is internally sorted, a small in-memory index of
  "every Nth key's byte offset" is enough to binary-search *within* a candidate file
  without scanning it linearly.
- **Compaction itself** is a read-cost mitigation, not just housekeeping: fewer, larger
  SSTables means fewer files a read has to consult in the first place. This is why
  compaction *strategy* is a first-class tuning knob — **size-tiered compaction** (merge
  same-size SSTables together, cheaper to run but lets more stale copies of a key
  accumulate before merging) versus **leveled compaction** (organize SSTables into levels
  with bounded per-level overlap, more compaction I/O up front but a tighter bound on how
  many files a single read has to check) is a direct write-cost-vs-read-cost dial, not an
  arbitrary implementation detail.

## Naming the Trade-off Precisely: Write, Read, and Space Amplification

"B-tree is read-optimized, LSM-tree is write-optimized" (Part 6's summary) is the right
one-liner, but the trade-off is actually three separate, nameable costs — worth knowing by
name because tuning any real engine (RocksDB's dozens of knobs, Cassandra's compaction
strategy setting) means explicitly trading these against each other:

| Amplification type | What it measures | B-tree | LSM-tree |
|---|---|---|---|
| **Write amplification** | Bytes actually written to disk, per logical byte written by the application | High — a small update can trigger a full page rewrite, plus cascading page splits | Lower for the initial write (sequential append), but compaction re-writes the same data multiple times over its lifetime |
| **Read amplification** | Number of disk reads needed to answer one logical read | Low — one page-read path per level, one physical copy of each row | Higher — may need to check the memtable plus several SSTables per read, mitigated but not eliminated by bloom filters |
| **Space amplification** | Disk space used, versus the logical size of the data | Low — one live copy per row, reclaimed immediately on overwrite | Higher — stale/overwritten versions and tombstones linger physically on disk until compaction runs and reclaims them |

This is a specific instance of the **RUM conjecture** (Read, Update, Memory) worth knowing
by name: a storage engine can optimize strongly for at most two of these three costs at
once — pushing one down mechanically pushes at least one other up. A B-tree optimizes
read + memory(space) at the cost of update; an LSM-tree optimizes update + (partially)
memory at the cost of read — there is no configuration of either structure that wins on all
three simultaneously, which is exactly why "just use whichever is faster" isn't a coherent
question until a workload is specified.

## Worked Example: The Same Workload, Two Engines

A time-series ingestion workload — a constant stream of new rows, keyed by a random device
ID, almost never updated after insert, occasionally range-queried by timestamp:

| | B-tree engine (e.g., Postgres) | LSM-tree engine (e.g., Cassandra) |
|---|---|---|
| Write cost | Each insert risks a random-access page write somewhere in the tree, per [Part 6's ~9 ms tax](06_mechanical_sympathy_and_physics_of_latency.md#random-vs-sequential-access-on-a-physical-disk) if the device-ID keyspace is effectively random | Every insert is a sequential WAL append + memtable insert — no seek, regardless of key randomness |
| Sustained write throughput | Degrades as the table grows past what fits in the page cache, since more inserts land on pages not already in RAM | Stays roughly flat — the hot path never depends on where in the keyspace a write lands |
| Read cost (single row) | One tree traversal, one physical location, cheap and consistent | Potentially memtable + N SSTables, mitigated by bloom filters — variance depends on compaction state |
| Range query (by timestamp) | Cheap if timestamp is the index — sorted, sequential leaf reads | Cheap **only if data is sorted/clustered by timestamp** — otherwise scattered across many SSTables |

**Why this workload is the textbook LSM-tree case**: high write volume, effectively random
keys, few updates — exactly the shape that makes a B-tree pay Part 6's random-write tax on
every single insert, while an LSM-tree's sequential-append write path never notices the
key's randomness at all. Flip the workload — a read-heavy dashboard querying a
slowly-changing reference table — and the trade favors the B-tree just as clearly, since its
read path is strictly simpler and its low update volume never triggers the LSM-tree's real
advantage.

## fsync: The Physical Line Between "Written" and "Durable"

Everything above assumed a write "reaches disk." That assumption is false by default, and
the gap between "the OS accepted this write" and "this write will survive a power loss" is
exactly what `fsync` exists to close.

**Why the gap exists at all**: a `write()` syscall, by default, only copies data into the
OS's **page cache** — RAM the kernel uses to batch and reorder disk I/O for performance,
per [Part 6's storage-tier economics](06_mechanical_sympathy_and_physics_of_latency.md#the-economics-of-machine-cost-is-physics)
(RAM is ~100x pricier than SSD specifically because it's faster, and the kernel exploits
that speed by buffering here rather than touching the slow medium on every call). `write()`
returning success means "the kernel has your bytes," not "the bytes are on the physical
platter or flash cell" — a power failure or kernel panic between those two moments loses
data the application already believes is safely stored.

**What `fsync(fd)` actually does**: it blocks until every buffered page for that file
descriptor is physically flushed out of the page cache and onto the storage device
(and, on some hardware, forces the device's own onboard write cache to flush too) — only
after `fsync` returns does "durability" in [Part 2's ACID
sense](02_data_and_consistency.md#acid-vs-base) become a true statement rather than an
optimistic assumption.

**Why this isn't free**: `fsync` forces exactly the kind of I/O [Part 6](06_mechanical_sympathy_and_physics_of_latency.md#random-vs-sequential-access-on-a-physical-disk)
warns is expensive — a physical flush, not a RAM-speed operation — and calling it on every
single write serializes an application against real device latency on every operation. This
single cost is why database engines don't call `fsync` naively per write, and is the direct
motivation for the mechanism in the next section.

## The Write-Ahead Log: Making Durability Affordable

A **write-ahead log (WAL)** — sometimes called a **commit log** (Cassandra's term for the
same mechanism) or **redo log** (MySQL/InnoDB's term) — is the single idea that makes both
B-tree and LSM-tree engines durable *without* forcing every write to pay for a full,
expensive, random-access flush of the main data structure before acknowledging success.

**The core idea**: before applying a change to the real (expensive-to-write-durably,
possibly-random-access) data structure, first append a compact description of that change
— "set key K to value V," or "row R changed from A to B" — to a **separate, append-only log
file**. An append-only file only ever grows at its tail, which means:

1. The WAL write itself is **sequential**, [Part 6's cheap access
   pattern](06_mechanical_sympathy_and_physics_of_latency.md#random-vs-sequential-access-on-a-physical-disk),
   regardless of how random the actual keyspace being modified is.
2. `fsync`-ing *just the WAL* is far cheaper than `fsync`-ing the main structure, because
   the WAL is one small, sequential, recently-written region — not scattered pages across
   a multi-gigabyte B-tree or a stack of SSTables.
3. The engine can acknowledge the write as durable **as soon as the WAL fsync completes**,
   and apply the change to the actual B-tree page or LSM memtable **lazily**, in memory,
   on its own schedule — the expensive structure gets updated whenever is convenient, since
   the WAL already guarantees the change survives a crash even if that update hasn't
   happened yet.

**Crash recovery — the payoff**: if the process or machine dies before the memtable/B-tree
page changes were ever flushed, the engine's recovery step on restart is simply: read the
WAL from the last known-durable point, and **replay** every logged change against the
in-memory structures. The WAL is, in effect, the single source of truth for "what actually
happened," with everything else (memtable, page cache, SSTables) treated as a
reconstructible cache of that log — which is exactly the property that lets a B-tree engine
skip an expensive in-place page flush on every write and an LSM-tree's memtable exist
entirely in volatile RAM in the first place, in both cases without sacrificing Part 2's
Durability guarantee.

```mermaid
sequenceDiagram
    participant App as Application
    participant WAL as WAL (disk, sequential)
    participant Mem as Memtable / Buffer Pool (RAM)
    App->>WAL: append "SET k=v" + fsync
    WAL-->>App: durable, write acknowledged
    App->>Mem: apply change (fast, in-memory)
    Note over Mem: Flushed to the main structure<br/>lazily, on its own schedule
```

**Group commit — the other lever on `fsync` cost**: rather than `fsync`-ing the WAL after
every single transaction, an engine can batch several transactions' WAL appends together
and issue one `fsync` for the whole batch — trading a small amount of added latency (waiting
a few milliseconds to see if more transactions arrive) for dramatically higher throughput,
since the expensive physical flush is now amortized across many transactions instead of
paid once per transaction. This is the same *cost-amortization-via-batching* pattern
[Part 8](08_cost_of_communication.md#paying-less-tax-data-locality-batching-coarse-apis-and-caching)
already established for network calls, applied here to the disk's durability boundary
instead.

**The WAL is orthogonal to B-tree vs. LSM-tree, not tied to either.** Postgres, MySQL/InnoDB,
and SQLite (all B-tree engines) all have a WAL/redo log. Cassandra, RocksDB, and LevelDB
(all LSM-tree engines) all have one too — in fact an LSM-tree's WAL is *structurally*
identical to its SSTables' write pattern (both are append-only sequential files), which is
part of why LSM-trees pair so naturally with cheap durability. The lesson: **durability
(WAL + fsync) and the choice of primary data structure (B-tree vs. LSM-tree) are two
independent design decisions** — every serious storage engine needs an answer to both,
and neither answer implies the other.

## NoSQL: A Data-Model Label, Not a Storage-Engine Choice

[Part 2's ACID-vs-BASE section](02_data_and_consistency.md#acid-vs-base) already covers the
consistency-model half of "SQL vs. NoSQL." The half that section doesn't cover — and the
one this doc is actually positioned to answer — is: **what's physically storing the bytes
underneath either label?** The honest answer is that "NoSQL" describes a *data model*
(key-value, document, wide-column, graph) and often a *consistency* choice, but says nothing
guaranteed about the storage engine underneath, which is a B-tree or an LSM-tree exactly as
described above, chosen independently of the data-model label:

| Database | Marketed as | Actual default storage engine |
|---|---|---|
| PostgreSQL, MySQL/InnoDB, SQLite | SQL / relational | B-tree |
| MongoDB (WiredTiger engine) | NoSQL / document store | B-tree, by default (can be configured for a log-structured mode) |
| Cassandra, HBase | NoSQL / wide-column store | LSM-tree |
| RocksDB, LevelDB | NoSQL / embedded key-value store | LSM-tree |
| MySQL with the MyRocks engine | SQL / relational (same query language) | LSM-tree |

**The MyRocks row is the whole argument in one line**: it is MySQL — the same SQL syntax,
the same relational data model everyone associates with "SQL databases" — running on
RocksDB's LSM-tree underneath, chosen specifically by teams whose write volume makes
InnoDB's B-tree write amplification the actual bottleneck. The data model a query language
exposes and the physical structure storing the bytes are **separable design decisions**;
"NoSQL" became shorthand for "probably LSM-tree, probably eventual consistency" through
historical correlation (Cassandra, DynamoDB, and HBase popularized the label and all
happened to pick LSM-trees for exactly the write-throughput reasons this doc covers), not
because the term names a storage mechanism at all. **The question worth asking about any
data store is never "is it SQL or NoSQL" — it's "what's my read/write ratio and key
distribution, and which of B-tree or LSM-tree actually fits that shape,"** which is a
question Part 2's CAP/consistency axis and this doc's storage-engine axis both feed into
independently, not a question either axis answers alone.

## Designing and Operating From First Principles

1. Is my workload write-heavy with a wide or random keyspace (favors an LSM-tree), or
   read-heavy with infrequent updates (favors a B-tree) — have I actually characterized
   this, or picked a database by its marketing label?
2. If I'm using an LSM-tree engine, have I chosen a compaction strategy (size-tiered vs.
   leveled) deliberately, based on whether write throughput or read latency matters more
   for this specific table — or left it at a default that may not fit my read/write ratio?
3. Do I know whether my database's `fsync` behavior is actually durable-by-default, or
   whether a setting (e.g., a database's "async commit" mode, or a filesystem mount option)
   is trading durability for throughput without my having made that trade-off consciously?
4. Am I calling `fsync` (or the equivalent commit setting) once per transaction naively, or
   does my engine batch commits (group commit) to amortize that physical flush cost?
5. If my process crashed right now, do I know — concretely, not just "the database handles
   it" — what my engine's WAL replay actually reconstructs, and how long that recovery
   would take on a large log?
6. Am I choosing "SQL" or "NoSQL" based on the data model and consistency needs I actually
   have, or am I implicitly assuming a storage-engine performance profile that the label
   doesn't actually guarantee (e.g., assuming a "NoSQL" choice is fast for writes without
   checking whether its engine is actually a B-tree)?
7. For a range-scan-heavy workload on an LSM-tree store, have I checked whether my data is
   physically clustered/sorted by the scan key, or am I scattering a logically sequential
   query across many unrelated SSTables?
8. Have I named which of the three RUM-conjecture costs (read, write/update, space
   amplification) I'm deliberately accepting for this specific table, rather than assuming
   a single engine choice optimizes all three?

## Key Takeaways

- Every persistent data structure is answering the same physical question — where to pay
  the cost of [Part 6's random-vs-sequential access
  gap](06_mechanical_sympathy_and_physics_of_latency.md#random-vs-sequential-access-on-a-physical-disk)
  — and no structure pays it nowhere.
- A B-tree keeps one physical, always-current copy of each row, sorted, which is exactly
  why reads and range-scans are cheap and why in-place updates cost a random-access page
  write (and possible cascading page splits).
- An LSM-tree defers all writes to a sequential WAL append and an in-memory memtable,
  flushing to immutable, sorted SSTables later — decoupling logical key order from
  physical write location, at the cost of a read potentially having to check multiple
  files, mitigated (not eliminated) by bloom filters and compaction.
- Write, read, and space amplification are three separate, nameable costs (the RUM
  conjecture) — a storage engine can optimize strongly for at most two at once; tuning a
  real engine (compaction strategy, index type) is choosing which two.
- `write()` returning success only means the OS page cache has the bytes — `fsync` is the
  actual physical durability boundary, and it's expensive precisely because it forces the
  random/physical I/O Part 6 already established is costly.
- The write-ahead log is what makes durability affordable: a cheap sequential append (and a
  cheap `fsync` on a small recent file) stands in for an expensive flush of the whole data
  structure, with crash recovery defined as "replay the log."
- Group commit batches multiple transactions' WAL fsyncs into one physical flush — the same
  cost-amortization-via-batching idea [Part 8](08_cost_of_communication.md) applies to
  network calls, applied here to disk durability.
- The WAL/durability mechanism and the B-tree-vs-LSM-tree choice are independent design
  axes — nearly every serious engine on either side needs both a log and a primary
  structure.
- "NoSQL" names a data model and often a consistency posture (per [Part
  2](02_data_and_consistency.md#acid-vs-base)), not a storage engine — MySQL can run on an
  LSM-tree (MyRocks) and MongoDB defaults to a B-tree (WiredTiger), proving the storage
  engine is a separable decision from the SQL/NoSQL label.
- The only question that actually predicts performance is read/write ratio and key
  distribution against B-tree vs. LSM-tree — not which marketing category a database falls
  into.

## Quick Self-Check

- Why does a B-tree's write path cost a random-access disk operation even though the tree
  itself is a well-organized, sorted structure?
- What does an LSM-tree give up on the read path in exchange for turning every write into a
  sequential append — and what two structures exist specifically to limit that cost?
- Name the three amplification costs in the RUM conjecture, and explain why no engine
  minimizes all three simultaneously.
- Why does `write()` returning successfully not guarantee durability, and what does `fsync`
  specifically do that closes that gap?
- Why is a WAL append cheap to make durable while a direct update to a B-tree page or an
  LSM memtable's eventual flush is not — what specific property of an append-only file
  makes the difference?
- What does "replay the WAL" actually reconstruct after a crash, and why does that mean the
  WAL — not the B-tree or the memtable — is the true source of record?
- Why does MySQL running on RocksDB (MyRocks) prove that "SQL vs. NoSQL" and "B-tree vs.
  LSM-tree" are different, independent axes rather than the same choice under two names?

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Cost-relocation framing (the default for a B-tree vs. LSM-tree question):** "I'd frame
  this as a single physical cost — a random write on a real disk — that every storage
  engine has to pay somewhere. A B-tree pays it immediately, on every write, by mutating a
  page in place wherever it happens to live. An LSM-tree defers it entirely by always
  appending, and pays instead on the read side, checking multiple files instead of one. It's
  the same tax, just relocated to a different part of the request path."
- **Durability-boundary framing (good for a 'what happens if the power dies' or crash-safety
  question):** "I'd separate 'the OS accepted this write' from 'this write is durable' — the
  first is just a page-cache copy, the second only becomes true once `fsync` physically
  flushes it. The reason every serious engine has a write-ahead log is that fsyncing a
  small, sequential log is cheap, while fsyncing the real data structure on every write
  would be brutally expensive — the log is what lets you get a durability guarantee without
  paying full price for it on every single write."
- **Axis-separation framing (good for a 'should we use a NoSQL database' question):** "I'd
  push back gently on 'NoSQL' as the deciding factor — it describes a data model and often
  a consistency posture, not a storage engine. MySQL can run on RocksDB's LSM-tree and
  MongoDB defaults to a B-tree, so the real question I'd actually ask is what our read/write
  ratio and key distribution look like, and pick a storage engine — B-tree or LSM-tree —
  based on that, independent of whatever the marketing label says."

### Vocabulary Builder

**Technical shorthand — use these instead of over-explaining the concept every time:**

- **memtable** (n.) — an LSM-tree's in-memory, sorted buffer for recent writes, flushed to
  disk as an SSTable once full.
- **SSTable (Sorted String Table)** (n. phrase) — an immutable, sorted, on-disk file an
  LSM-tree writes once and only ever reads, merges, or deletes — never edits in place.
- **compaction** (n.) — the background process that merges multiple SSTables into fewer,
  larger ones, reclaiming space from overwritten/deleted keys and bounding read
  amplification.
- **bloom filter** (n. phrase) — a compact probabilistic structure answering "definitely
  absent, or maybe present" for a key in a given SSTable, letting reads skip files that
  can't possibly contain the target key.
- **write/read/space amplification** (n. phrases) — the three costs of the RUM conjecture; a
  storage engine trades between them rather than minimizing all three at once.
- **write-ahead log (WAL)** (n. phrase) — an append-only log of intended changes, written
  and fsynced before (or instead of) the expensive main structure, replayed on crash
  recovery; also called a commit log (Cassandra) or redo log (InnoDB).
- **group commit** (n. phrase) — batching multiple transactions' WAL fsyncs into a single
  physical flush to amortize `fsync`'s cost across more work.
- **page cache** (n. phrase) — the OS's in-RAM buffer for disk I/O; a `write()` call lands
  here first, which is exactly the gap `fsync` exists to close.

**Expressive phrases — for stating a trade-off fluently instead of listing pros/cons:**

- **"…the same tax, just relocated"** — a compact way to describe B-tree-vs-LSM-tree (or
  any similar architectural choice) as moving a fixed cost to a different part of the
  system, rather than eliminating it.
- **"…a hint that it's in RAM, not proof it's durable"** — a fluent way to describe what a
  successful `write()` call actually promises before `fsync`.
- **"…the log is the source of truth; everything else is a reconstructible cache of it"** —
  a precise way to explain what WAL-based crash recovery actually means mechanically.
- **"…a data-model label, not a storage-engine guarantee"** — a reusable way to push back on
  treating "NoSQL" as if it implies a specific performance profile.

---

**Previous:** [Part 9: The Anatomy of a Request (DNS, BGP, and the Edge)](09_dns_bgp_and_the_edge.md)  |  **Next:** [0. The Interview Framework](../00_interview_framework/00_interview_framework.md)
