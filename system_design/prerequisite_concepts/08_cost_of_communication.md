# Prerequisite Concepts, Part 8: The Cost of Communication

[Part 7](07_saturation_amdahls_law_and_hedged_requests.md) covered saturation and the
statistical tricks (hedged requests) that buy back tail latency. This part zooms out to the
idea underneath almost everything in this primer: **a network call is never a function
call, and the gap between how it looks in your source file and what it actually costs is
where most "why is this slow" investigations go to die.** [Part 3](03_communication_and_resilience.md)
already walked through the DNS → TCP → TLS → HTTP sequence and the resilience vocabulary
for when a call fails; [Part 6](06_mechanical_sympathy_and_physics_of_latency.md) already
derived Little's Law and the physical distance/bandwidth model. This part's job is
different: it treats communication as a **stack of taxes** — layers a request pays through
before your application logic ever runs — points back to Parts 3 and 6 for the pieces
already covered in depth, and goes further into the layers they don't: the kernel's
privilege boundary, the CPU cost of encryption and serialization, the protocol evolution
that exists specifically to avoid paying these taxes twice, and the concrete design moves
(batching, coarse APIs, caching, data locality) that reduce how often you pay them at all.

## The Fallacy of Local Code

Consider this code:

```python
def get_user_profile(user_id):
    user = db.fetch_user(user_id)
    preferences = cache.get(f"prefs:{user_id}")
```

Syntactically, it reads like two local function calls — `CPU → Database`, `CPU → Cache`,
done. What actually happens on the first line:

```mermaid
flowchart TD
    A[Your Process] -->|syscall| B[Linux Kernel]
    B --> C[Network Stack]
    C --> D[NIC]
    D --> E[Switches / Routers]
    E --> F[Remote Machine]
    F --> G[Remote Kernel]
    G --> H[Database Process]
```

One line of Python may involve a DNS lookup, a TCP handshake, a TLS negotiation,
encryption, a kernel context switch, packet transmission, router hops, remote kernel
scheduling, database execution, response serialization, transmission back, and
deserialization — an entire distributed protocol wearing the syntax of a function call.
This gap between *looks local* and *is actually a WAN round trip* is called the **fallacy
of local code**, and it's a specific instance of a much older, well-known list worth
knowing by name.

**Deutsch and Gosling's Eight Fallacies of Distributed Computing** (Sun Microsystems, the
1990s) names the assumptions programmers unconsciously carry over from writing local code,
each one false the moment a call crosses a process boundary:

1. The network is reliable.
2. Latency is zero.
3. Bandwidth is infinite.
4. The network is secure.
5. Topology doesn't change.
6. There is one administrator.
7. Transport cost is zero.
8. The network is homogeneous.

**Why this list is worth citing by name in an interview**: naming it signals the gap isn't
a personal observation — it's a documented, decades-old failure mode that every distributed
system design has to actively design against, not something a good-enough programming
language eventually fixes. Every section below is really just one of these eight fallacies,
made concrete and given a number.

## The Request Lifecycle as a Stack of Taxes

Every remote call pays a sequence of layered costs before application logic runs:

```mermaid
flowchart LR
    A[Application] --> B[Serialization]
    B --> C[Kernel]
    C --> D[TCP]
    D --> E[TLS]
    E --> F[Network]
    F --> G[Remote Kernel]
    G --> H[Remote Application]
```

Most developers only budget for the last box — "database execution time." Architects
budget for everything before it. The table below is the map; the sections after it cover
the entries that Parts 3 and 6 don't already own in depth.

| Tax | What it is | Covered in depth |
|---|---|---|
| DNS resolution | Resolving a hostname to an IP before anything else can happen | [Part 3](03_communication_and_resilience.md#what-actually-happens-when-you-hit-enter) |
| TCP handshake | SYN / SYN-ACK / ACK before any application byte flows | [Part 3](03_communication_and_resilience.md#what-actually-happens-when-you-hit-enter) |
| TLS handshake | Negotiating encryption before any application byte flows | [Part 3](03_communication_and_resilience.md#what-actually-happens-when-you-hit-enter) |
| Kernel / syscall | Crossing the privilege boundary between your process and the NIC | This doc |
| Serialization | Turning language objects into bytes and back | This doc |
| TLS compute | The CPU cost of encrypting/decrypting every byte, *after* the handshake | This doc |
| Reliability / retries | Detecting and recovering from lost packets | This doc, [Part 3](03_communication_and_resilience.md#resilience-vocabulary) |
| Queueing | Waiting for a busy server, governed by Little's Law | [Part 6](06_mechanical_sympathy_and_physics_of_latency.md#littles-law-l-w) |
| Physics | The speed-of-light floor on propagation delay | [Part 6](06_mechanical_sympathy_and_physics_of_latency.md#distance-of-data-one-physical-idea-two-different-scales), worked example below |

## The Kernel Tax: Every Byte Crosses a Privilege Boundary

An application cannot touch the network card directly — hardware access is a privileged
operation, so every send/receive goes `Application → syscall → Kernel → NIC`. Each syscall
costs a **privilege switch**, a register save, a context switch, and kernel scheduling —
individually cheap (illustrative and approximate: low microseconds), but multiplied across
thousands of small requests per second it becomes a real, measurable tax, and it compounds
with the context-switching cost already covered from the CPU-cache side in
[Part 6](06_mechanical_sympathy_and_physics_of_latency.md): each kernel transition
invalidates CPU caches, stalls the pipeline, and resets branch predictors, so the CPU spends
cycles switching contexts instead of computing.

This is precisely the tax that **kernel-bypass technologies** exist to avoid — a strategy
of "stop asking the kernel to mediate every single packet":

- **DPDK** (Data Plane Development Kit) — lets a userspace application drive the NIC
  directly, skipping the kernel's networking stack entirely for latency-critical paths.
- **io_uring** — a Linux kernel interface that batches I/O submission and completion
  through shared ring buffers, sharply cutting the *number* of syscalls needed for a given
  amount of I/O.
- **eBPF** — runs sandboxed code *inside* the kernel at hook points, avoiding a full
  userspace round trip for packet filtering, load balancing, and observability.
- **RDMA** (Remote Direct Memory Access) — lets one machine read/write another machine's
  memory directly, bypassing both machines' kernels and CPUs for the transfer itself.

**Why large cloud providers invest heavily here**: at the scale of a hyperscaler's fleet,
shaving even a few microseconds of kernel overhead per packet compounds into measurable
fleet-wide CPU and cost savings — the same "small constant factor, huge scale" logic behind
most low-level infrastructure investment in this repo.

## The TLS Compute Tax (Distinct From the TLS Handshake)

Part 3 covers the TLS *handshake* — the negotiation that happens once per connection.
There's a second, ongoing TLS cost that's easy to miss: once the handshake is done, **every
byte of the payload still has to be encrypted on send and decrypted on receive**, for the
entire lifetime of the connection. This is real CPU work, not a one-time setup fee, which is
exactly why modern CPUs ship **AES-NI** — dedicated instructions that accelerate AES
encryption/decryption in hardware rather than software, because at the volume a busy service
handles, software-only crypto would visibly tax the CPU budget on every single request.

## The Serialization Tax: Objects Aren't Bytes

Your program holds Python objects, Java objects, Go structs, C++ classes — the network
understands only bytes. Every remote call pays `Object → Serialization → Bytes → Network →
Bytes → Deserialization → Object`, and the *format* chosen for that middle step has a real
performance cost, not just a readability one.

**JSON** — easy for humans, expensive for CPUs:

```json
{"id": 123, "name": "Alice"}
```

Parsing it costs string parsing, UTF-8 decoding, dynamic memory allocation, hash-map
construction, and repeated copies — a CPU spends real, measurable time turning text back
into structured memory on both ends of every call.

**Binary protocols** — Protocol Buffers, FlatBuffers, Cap'n Proto, Avro, MessagePack — skip
almost all of that: a field is a fixed number of bytes at a known offset, no string parsing,
smaller payloads, less CPU, less memory, less bandwidth. **Zero-copy serialization** goes
one step further, arranging memory so data can move `Memory → Network → Memory` without a
copy step at all — copying memory is, itself, surprisingly expensive at volume, for exactly
the mechanical-sympathy reasons Part 6 covers for memory hierarchies generally.

### Apache Arrow: Killing Serialization Between Systems, Not Just Across the Wire

A traditional analytics pipeline pays the serialization tax *repeatedly*, once per hop:

```mermaid
flowchart LR
    A[Database] --> B[CSV]
    B --> C[Parse in Python]
    C --> D[NumPy]
    D --> E[Pandas]
```

Every arrow in that diagram is a copy-and-reparse step. **Apache Arrow** defines a standard
**columnar in-memory format** that every hop can share directly:

```mermaid
flowchart LR
    A[Database] --> B[Arrow Buffer]
    B --> C[Network]
    C --> D[Arrow Buffer]
    D --> E[Pandas / NumPy / Spark]
```

The same memory layout survives the whole trip, giving zero-copy reads, SIMD-friendly
processing, better CPU cache utilization, and cross-language interoperability (Python,
Java, C++, Rust, Go) — a dramatic cut to serialization overhead specifically for analytical
workloads. (Worth flagging explicitly since the two names are easy to conflate: **Apache
Airflow** is a workflow orchestrator — see [Part 8: ML Orchestration](../08_ml_orchestration/tutorial.md)
— and has nothing to do with in-memory data layout; Arrow is the one relevant here.)

## Reliability, Retries, and Tail Latency

Networks lose packets. TCP detects a missing one via timeout and retransmits it — which
means a request budgeted for 2 ms can, on a bad packet, suddenly cost 200 ms. Average
latency can stay low while **tail latency explodes**, which is exactly why large-scale
systems optimize P95/P99/P99.9 instead of the mean — Google's own observation is that slow
requests dominate perceived user experience in systems operating at real scale, and Part 7's
hedged-request pattern exists specifically to buy back this tail.

TCP also deliberately **slows itself down under congestion** (algorithms: Reno, Cubic, BBR)
— without this, every sender would keep transmitting into an already-overloaded network,
producing full collapse instead of graceful slowdown. The queueing side of this tax — what
happens when the *server*, not the network, is the busy resource — is exactly Little's Law
territory, fully derived with its feedback-loop failure mode in
[Part 6](06_mechanical_sympathy_and_physics_of_latency.md#littles-law-l-w); the
short version worth repeating here is that **processing time is frequently not where the
latency is** — a 2 ms database query behind an 80 ms queue is an 82 ms request whose
bottleneck has nothing to do with the query.

## Physics Sets the Floor: A Worked RPC Example

Part 6 derives the general distance/bandwidth model; here's what it costs a concrete,
cold cross-region RPC. Light in vacuum travels at 300,000 km/s; in fiber-optic glass, the
refractive index slows it to roughly 200,000 km/s (illustrative and approximate — exact
figures depend on the fiber and route, the relationship is the point). San Francisco to
London is roughly 8,500 km one-way, ~17,000 km round trip — giving a theoretical minimum
RTT of `17,000 / 200,000 ≈ 85 ms` that no engineering effort can beat; it's a property of
the glass and the distance, not the software.

Stack a **cold** connection's taxes on top of that floor (HTTP/1.1, TLS 1.2, illustrative
figures):

| Step | Cost |
|---|---|
| TCP handshake | ~85 ms (1 RTT) |
| TLS handshake | ~170 ms (2 RTTs, TLS 1.2) |
| HTTP request/response | ~85 ms (1 RTT) |
| Database execution | ~5 ms |
| **Total** | **≈345 ms** |

Of that ≈345 ms, the database itself accounts for 5 ms — under 2% of the total. **The
database was never the bottleneck; the connection setup was.** This single number is the
entire argument for everything in the rest of this doc: keep-alive connections, TLS session
resumption, HTTP/2 multiplexing, and QUIC/HTTP/3's 0-RTT resumption all exist because paying
~340 ms of setup tax on every request, when the actual work costs 5 ms, is not a database
performance problem — it's a communication-design problem.

## Protocol Evolution: Paying the Setup Tax Fewer Times

**HTTP/1.1** — one connection per request (or limited keep-alive), head-of-line blocking,
many repeated handshakes.

**HTTP/2** — a single TCP connection, multiplexed so many requests travel concurrently,
binary framing, and header compression (HPACK) to shrink repeated header overhead.

**HTTP/3 (QUIC)** — replaces the layered `TCP → TLS → HTTP` stack with one integrated
protocol running over UDP, so connection setup and encryption negotiate together instead of
sequentially:

| Protocol | Transport | Multiplexing | Handshake cost |
|---|---|---|---|
| HTTP/1.1 | TCP | No | High |
| HTTP/2 | TCP | Yes | Medium |
| HTTP/3 | QUIC | Yes | Lowest |

QUIC also enables **0-RTT**: a returning client that already has a valid session can send an
encrypted request in its very first packet, skipping the handshake round trips entirely —
for the mobile and cross-region cases where every RTT is expensive (per the worked example
above), this is a large, direct performance win rather than a marginal one.

### gRPC: Multiplexing the Tax Away at the Application Layer

Plain REST over HTTP/1.1 often pays a fresh `Request → TCP → Close → Repeat` cycle per call.
**gRPC** runs over HTTP/2, so many requests share one already-established, already-encrypted
connection — fewer handshakes, less congestion, lower latency, native streaming, and binary
serialization (Protocol Buffers) instead of JSON. This combination is exactly why Discord,
Google, and most cloud-native service meshes lean on gRPC for internal service-to-service
traffic rather than REST.

## Microservices: A Communication Tax by Design Choice

Microservices buy independent deployability and team autonomy — genuinely worth having —
but every service boundary drawn on an architecture diagram is also a **network boundary**,
and every hop across it re-pays DNS, TCP, TLS, serialization, retries, monitoring,
authentication, and rate limiting:

```mermaid
flowchart LR
    A[Service A] -->|network| B[Service B]
    B -->|network| C[Service C]
    C --> D[(Database)]
```

versus a monolith's `Function() → Function() → Function()`, which pays none of it. Many
small services compound into a **latency amplification** problem — a single user-facing
request can fan out into a dozen internal RPCs, each carrying its own tail-latency risk
(per the reliability section above), so the *slowest* hop, not the average one, sets the
user-visible latency floor. A **service mesh** (Envoy/Istio-style sidecar proxies) adds yet
another hop — intercepting every call for retries, mTLS, and observability — which buys
real operational value but is itself an additional tax layered on top, worth naming
explicitly when someone proposes adding a mesh "for free."

None of this means microservices are the wrong call. It means **every service boundary
should justify its communication cost** — a boundary drawn along a genuine team/ownership
line is worth the tax; a boundary drawn for its own sake (a "user service" and a "profile
service" that always change together and are always called together) is paying real,
compounding latency for an org-chart preference.

## Paying Less Tax: Data Locality, Batching, Coarse APIs, and Caching

Four concrete design moves, each reducing *how often* or *how far* communication has to
happen, rather than making any single call faster:

**Data locality — move computation to the data, not data to the computation.** Pulling
100 GB out of a database into application code to filter it there pays the full
serialization-and-transfer tax on data that's mostly discarded; pushing a `WHERE` clause
into SQL and pulling back 100 rows pays that tax on only what's kept. Spark, Snowflake,
ClickHouse, and Databricks are all built around this same principle at larger scale.

**Batching — amortize the round trip across many items.** 1,000 individual `GET /user/{id}`
calls pay 1,000 round trips; one `POST /users/batch` call carrying all 1,000 IDs pays one.
The per-item cost of a network round trip is fixed regardless of payload size within
reason, so batching turns a linear cost in *requests* into a roughly constant one.

**Coarse-grained APIs — let the server orchestrate, not the client.** Fetching `Get User →
Get Orders → Get Address → Get Preferences` as four separate calls pays four round trips
from a client that might be on a slow or distant connection; a single `Get Dashboard` call
that performs that orchestration server-side (where the fan-out is likely happening over a
fast, low-latency datacenter network instead) pays one round trip from the client and
absorbs the internal fan-out where it's cheapest.

**Caching — the cheapest network request is the one you never make.** Every step further
from the CPU costs roughly another order of magnitude in latency (illustrative and
approximate — exact figures vary by hardware generation, the *relationship* is the point;
see [Part 6](06_mechanical_sympathy_and_physics_of_latency.md#distance-of-data-one-physical-idea-two-different-scales)
for why distance drives this):

| Storage | Latency |
|---|---|
| CPU register | <1 ns |
| L1 cache | ~1 ns |
| L2 cache | ~4 ns |
| L3 cache | ~10-20 ns |
| RAM | ~100 ns |
| NVMe SSD | ~100 µs |
| Network (same datacenter) | ~100-500 µs |
| Cross-region network | 10-100+ ms |

A local cache hit doesn't just return data faster than a remote fetch — it eliminates an
entire round trip's worth of DNS/TCP/TLS/kernel/serialization tax from the table earlier in
this doc, all at once.

## Designing From First Principles

When designing any distributed system, run through these questions — they matter more, more
often, than the choice of language or framework:

1. Can I avoid the communication entirely?
2. Can I cache the result?
3. Can I batch multiple requests into one?
4. Can I move computation closer to the data?
5. Can I keep the connection open (keep-alive, HTTP/2, gRPC) instead of re-establishing it?
6. Can I reduce serialization cost (a binary format instead of JSON)?
7. Can I reduce kernel crossings (fewer, larger syscalls)?
8. Can I reduce cross-region traffic specifically, given the speed-of-light floor?
9. Can I tolerate eventual consistency to reduce coordination overhead (the trade-off
   [Part 2](02_data_and_consistency.md) covers in depth)?
10. Given the tail-latency risk any single hop carries, does this call even need to be
    synchronous (per [Part 3](03_communication_and_resilience.md#synchronous-vs-asynchronous-communication))?

**The mental model that ties it together**: a local function call is handing a note to the
person sitting next to you — instant, no packaging required. A remote call is an
international courier shipment — it must be packaged (serialized), given an address (DNS),
routed (TCP), passed through security (TLS), moved through congested infrastructure
(network, queueing), and possibly resent if lost in transit (retries). The syntax on the
page looks identical either way; the cost is not, and mistaking one for the other is the
fallacy this whole doc is named after.

## Key Takeaways

- Local-looking code can hide an entire distributed protocol's worth of remote cost.
- In distributed systems, latency is usually dominated by communication, not computation —
  the worked example above spent 98% of its time on setup, 2% on the actual query.
- Every request pays some subset of: DNS, TCP, TLS (handshake *and* ongoing compute),
  kernel transitions, serialization, queueing, congestion control, and retries.
- Physics sets an absolute floor via the speed of light — no engineering effort crosses it,
  only shortens the distance that has to be crossed.
- Binary protocols (Protocol Buffers, Arrow) beat verbose text formats (JSON) whenever
  performance, not just human readability, is the goal.
- Reusing connections (HTTP/2, gRPC, HTTP/3/QUIC) amortizes handshake cost across many
  requests instead of re-paying it every time.
- Move computation to the data, not the other way around.
- Batch operations and prefer coarse-grained APIs to amortize round-trip cost.
- Cache aggressively — the fastest network request is the one that never happens.
- Optimize tail latency (P95/P99), not just the average, since that's what users feel.

## Quick Self-Check

- A single line of application code calling a database involves DNS, TCP, TLS, kernel
  transitions, and serialization before the query executes. Which of Deutsch and Gosling's
  eight fallacies does each of those correspond to?
- In the worked SF↔London example, why does trimming the database query time have almost no
  effect on total latency, while switching to a resumed/kept-alive connection (or HTTP/3's
  0-RTT) has a large one?
- Why is JSON's *cost* mostly about parsing (string handling, allocation, hash-map
  construction) rather than raw payload size — and why doesn't a smaller JSON payload fix
  that?
- A team proposes splitting one service into five microservices along org-chart lines, with
  no change in which data is usually read together. What communication tax did that
  decision just introduce, and what would justify paying it?
- Why does caching eliminate *more* than just the data-fetch time — what specific taxes
  from the request lifecycle table does a cache hit skip entirely?

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Taxation framing (the default for "why is this slow" or system-design trade-off
  questions):** "I think of every remote call as paying a stack of taxes before any
  application logic runs — DNS, TCP, TLS, kernel transitions, serialization, queueing — and
  in one worked cross-region example, those taxes were 98% of total latency while the
  actual database query was 2%. That's usually where I'd look first, not the query."
- **Fallacies-of-distributed-computing framing (good for signaling this isn't a personal
  observation but a known failure mode):** "This is really Deutsch and Gosling's eight
  fallacies made concrete — the code assumes the network is reliable and free, and it isn't
  either. Naming the specific fallacy in play tells you which fix applies: retries for
  'the network is reliable,' caching or a CDN for 'latency is zero,' mTLS for 'the network
  is secure.'"
- **Amortization framing (good for batching/coarse-API/protocol questions):** "Almost every
  fix here is really the same move: pay the fixed cost of a round trip once and spread more
  work across it — batching spreads it across items, gRPC/HTTP2 spread it across requests,
  a coarse API spreads it across what would've been several client calls."

### Vocabulary Builder

**Technical shorthand — use these instead of over-explaining the concept every time:**

- **fallacies of distributed computing** (n. phrase) — Deutsch and Gosling's canonical list
  of eight false assumptions programmers carry over from local code into distributed
  systems; citing it by name signals the failure mode is well-documented, not anecdotal.
- **kernel bypass** (n. phrase) — techniques (DPDK, io_uring, RDMA) that let an application
  avoid routing every packet through the kernel's networking stack, cutting syscall and
  context-switch overhead.
- **0-RTT** (n. phrase) — a returning client sending an encrypted request in its very first
  packet, skipping connection-setup round trips entirely; a headline feature of QUIC/HTTP/3.
- **coarse-grained API** (n. phrase) — an endpoint that performs server-side orchestration
  across several internal calls so the client pays one round trip instead of several.
- **latency amplification** (n. phrase) — the compounding effect of chaining several
  network hops (as in a microservice fan-out), where the slowest hop, not the average,
  determines user-visible latency.

**Expressive phrases — for stating a trade-off fluently instead of listing pros/cons:**

- **"…the database was never the bottleneck; the connection setup was"** — a precise way to
  redirect a performance conversation away from query tuning toward connection/protocol
  design when the numbers show setup dominating.
- **"…every service boundary should justify its communication cost"** — a fluent way to
  push back on a microservice split that isn't backed by an actual team or scaling reason.
- **"…the fastest network request is the one you never make"** — a compact, quotable
  argument for caching, batching, or data locality over optimizing the call itself.

---

**Previous:** [Part 7: Saturation, Amdahl's Law & Hedged Requests](07_saturation_amdahls_law_and_hedged_requests.md)  |  **Next:** [0. The Interview Framework](../00_interview_framework/tutorial.md)
