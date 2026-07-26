# Prerequisite Concepts, Part 2: Data Distribution and Consistency

[Part 1](01_performance_and_scale.md) covered how to measure a system and why horizontal
scaling requires statelessness. This part covers what happens to the *state* itself once
it can no longer live on one machine — the first-principles reasoning behind sharding,
replication, indexing, and the consistency trade-offs that follow from splitting data
across many machines.

## Why One Machine Stops Being Enough — Two Different Problems

It's worth separating these explicitly, because the two most common "split the data up"
techniques each solve a *different* one:

- **Problem 1: capacity.** A single machine has finite CPU, RAM, disk, and network
  bandwidth. Past a certain data volume or query rate, no single machine — however
  powerful — can hold or serve it all. **Sharding/partitioning** solves this: split the
  data so each machine holds and serves only a slice of it.
- **Problem 2: durability and availability.** Even a machine that *could* hold all the
  data is a single point of failure — if it dies, the data is gone (or at least
  unreachable) until it's restored. **Replication** solves this: keep copies of the same
  data on multiple machines, so losing one doesn't lose the data or the ability to serve
  it.

**These are orthogonal, and real systems need both, deliberately combined**: shard the
data for capacity, then replicate each shard for durability. Confusing the two — thinking
replication gives you more capacity, or that sharding gives you durability — is a common
first-principles gap; sharding *without* replication just gives you many single points of
failure instead of one.

## Sharding / Partitioning, Briefly

Covered in depth in [Fundamentals](../01_fundamentals/tutorial.md#sharding-partitioning)
and extended further in the [distributed systems foundations
tutorial](http://127.0.0.1:8002/01_distributed_systems_foundations/tutorial/#consistent-hashing-advanced-sharding)
(consistent hashing, virtual nodes) — the short version: split data across machines by
some key (range-based, hash-based, or directory-based), and watch for **hot shards**
(uneven access concentrating load on one partition despite even data distribution) as the
recurring failure mode to name proactively.

## Replication, Briefly

Also covered in [Fundamentals](../01_fundamentals/tutorial.md#replication) — the short
version: leader-follower replication is the standard pattern (writes go to a leader, reads
can be served from followers, at the cost of replication lag), and multi-leader/leaderless
setups trade that simplicity for higher write availability at the cost of needing an
explicit conflict-resolution strategy.

## Indexing: Why Databases Don't Scan Everything

A concept assumed by name in almost every tutorial in this repo, worth actually
understanding from first principles.

**The problem an index solves**: without one, finding a specific row (`WHERE user_id =
12345`) means scanning every row in the table, checking each one — a **full table scan**,
O(n) in table size. Fine for a thousand rows, unusable for a billion.

**The core idea**: maintain a separate, smaller, *sorted* (or otherwise organized)
structure that maps a column's values to the location of the corresponding rows — so
looking a value up means searching the small sorted structure, not the whole table.

- **B-Tree indexes** (the default for most relational databases): a balanced tree
  structure keeping keys sorted, giving O(log n) lookups *and* efficient range queries
  (`WHERE created_at BETWEEN X AND Y`) because sorted order is preserved — you can walk
  sideways through the tree for a range instead of doing n separate lookups.
- **Hash indexes**: map a key directly to a location via a hash function — O(1) average
  lookup for exact-match queries (`WHERE user_id = 12345`), but **can't support range
  queries at all**, since a hash function deliberately scrambles order (a hash index has no
  concept of "the value right after this one").

**The trade-off that makes indexing a genuine design decision, not a free win**: every
index has to be updated on every write that touches its column — more indexes means faster
reads but slower writes and more storage. This is why database schema design involves
deliberately choosing *which* columns get indexed (the ones actually queried against
frequently) rather than indexing everything defensively — an over-indexed table pays a
real, continuous write-latency tax for read speed most of its indexes never actually
deliver.

## CAP Theorem, Briefly

Covered in depth in [Fundamentals](../01_fundamentals/tutorial.md#cap-theorem-consistency-models)
— the short version, for continuity: under a network partition, a distributed system must
choose between **C**onsistency (every read sees the latest write) and **A**vailability
(every request gets a response) — you cannot have both. Naming which side a given stateful
component of your design lands on, and why, is exactly the [trade-off vocabulary the
interview framework](../00_interview_framework/tutorial.md#the-trade-off-vocabulary-cheat-sheet)
expects you to use fluently.

## The Consistency Spectrum: It's Not Just Strong vs. Eventual

CAP theorem's "C" is really one end of a spectrum, not a binary — worth knowing the
intermediate points by name, since real systems often deliberately choose one of these
rather than either extreme:

| Model | Guarantee | Example |
|---|---|---|
| **Strong consistency** | Every read sees the most recent write, globally, immediately | A bank balance after a transfer |
| **Read-your-own-writes** | You always see your *own* recent writes, but might see stale data from others' writes | You post a comment and see it immediately, even if a friend's feed hasn't updated yet |
| **Causal consistency** | Writes that are causally related are seen in order by everyone; unrelated writes can appear in any order | A reply to a comment never appears *before* the comment it's replying to |
| **Eventual consistency** | Given enough time with no new writes, all replicas converge to the same value — no ordering guarantee in between | A DNS record change propagating across resolvers over some minutes |

**Why this spectrum matters practically**: "eventual consistency" is often too weak a
guarantee for what a product actually needs (users find "I posted something and can't see
it" jarring), while full strong consistency is often far more expensive than necessary.
Read-your-own-writes is frequently the sweet spot — cheaper to implement than global strong
consistency, but resolves the specific user-facing confusion eventual consistency alone
tends to cause.

## ACID vs. BASE

Two different philosophies for what guarantees a data store makes about transactions —
worth knowing as named opposites, since interviewers use both terms expecting you to place
a design on this spectrum:

- **ACID** (traditional relational databases): **A**tomicity (a transaction fully succeeds
  or fully fails, never partially), **C**onsistency (a transaction moves the database from
  one valid state to another, per its constraints), **I**solation (concurrent transactions
  don't see each other's uncommitted intermediate state), **D**urability (once committed, a
  write survives a crash). Strong guarantees, generally at the cost of throughput and
  horizontal scalability.
- **BASE** (many distributed/NoSQL systems): **B**asically **A**vailable, **S**oft state
  (state may change over time even without new input, as replicas converge), **E**ventual
  consistency. Weaker guarantees, deliberately traded for availability and horizontal
  scale.

**The practical framing**: ACID vs. BASE is CAP theorem's C-vs-A trade-off, restated as a
design philosophy rather than a single-partition-event decision — a payments ledger
reaches for ACID because a partially-applied transfer is unacceptable; a social media like
counter reaches for BASE because a few seconds of eventual convergence is invisible to
users and the availability win is worth far more than the consistency it costs.

## Idempotency

Introduced in passing in the [ingestion pipeline
tutorial](../02_ingestion_pipeline/tutorial.md#idempotency), but foundational enough to
define here explicitly: an operation is **idempotent** if performing it multiple times has
the exact same effect as performing it once.

**Why distributed systems can't avoid needing this**: any network call can fail *after*
the server processed it but *before* the client got the confirmation — the client has no
way to distinguish "it failed" from "it succeeded but the response was lost," so the only
safe client behavior is to retry. **Retries are not optional in a distributed system; they
are inevitable.** If retrying "charge this customer $10" isn't idempotent, a lost
confirmation turns into a double charge. Making it idempotent (e.g., via a client-generated
idempotency key the server deduplicates on) means retrying is always safe, which is what
actually lets a system retry aggressively and reliably instead of walking a tightrope
between "retry and risk duplication" and "don't retry and risk silently dropping work."

## Quick Self-Check

Before moving to [Part 3: Communication & Resilience](03_communication_and_resilience.md),
you should be able to answer these without looking back:

- Why does sharding a dataset *without* replicating it leave you with more single points
  of failure, not fewer?
- Why can't a hash index support a range query, structurally?
- Where does read-your-own-writes sit between strong and eventual consistency, and what
  specific user-facing problem does it solve that eventual consistency alone doesn't?
- Why is idempotency a prerequisite for safe retries, rather than a nice-to-have?

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Two-different-problems framing (the default for 'why shard AND replicate'):** "I'd
  separate these explicitly — sharding solves capacity, replication solves durability and
  availability. They're orthogonal, and sharding without replication just gives you many
  single points of failure instead of one, which is a common first-principles gap worth
  naming out loud."
- **Cost-of-the-index framing (good for indexing questions):** "An index isn't a free
  win — every index has to be updated on every write that touches its column, so more
  indexes means faster reads but a real, continuous write-latency tax. I'd only index
  columns actually queried against frequently, not defensively."
- **Spectrum-not-binary framing (good for consistency questions):** "I wouldn't present
  this as strong versus eventual consistency — read-your-own-writes sits in between, and
  it's often the sweet spot: cheaper than global strong consistency, but it resolves the
  specific 'I posted this and can't see it' confusion eventual consistency alone causes."

### Vocabulary Builder

**Technical shorthand — use these instead of over-explaining the concept every time:**

- **full table scan** (n. phrase) — checking every row to find a match, the O(n) baseline
  an index exists specifically to avoid.
- **read-your-own-writes** (n. phrase) — a consistency guarantee where you always see your
  own recent writes immediately, even if others temporarily see stale data.
- **soft state** (n. phrase, from BASE) — state that can change over time even without new
  input, as replicas converge toward consistency.
- **idempotency key** (n. phrase) — a deterministic identifier letting a retried operation
  produce the same result as the original, the mechanism that makes retrying safe.

**Expressive phrases — for stating a trade-off fluently instead of listing pros/cons:**

- **"…are orthogonal, and real systems need both, deliberately combined"** — a precise
  template for arguing two techniques solve different problems rather than competing
  solutions to the same one.
- **converge** (v.) — for distributed replicas to reach the same value over time.
  *"Given enough time with no new writes, the replicas converge."*
- **"Retries are not optional in a distributed system; they are inevitable"** — a strong,
  quotable line for justifying why idempotency is a foundational requirement, not a
  nice-to-have.

---

**Previous:** [Part 1: Performance & Scale](01_performance_and_scale.md)  |  **Next:** [Part 3: Communication & Resilience](03_communication_and_resilience.md)
