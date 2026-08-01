# LLD vs. HLD: Concepts and the Full Interview Question Bank

A single reference for both design-round flavors: what each one actually evaluates, the
concept checklist for each, and a comprehensive, generalized list of the prompts that show
up under each — collected across the industry rather than tied to any one employer, since
the two rounds recur with remarkably little variation regardless of who's asking.

## The Core Distinction

| | LLD (Low-Level Design) | HLD (High-Level Design) |
|---|---|---|
| Unit of design | A class, interface, or small set of collaborating classes | A service, or a set of services and the infrastructure connecting them |
| Runs inside | One process, one machine | Many machines, often many regions |
| The hard part | Picking abstractions that stay correct and extensible (SRP, is-a vs. has-a, which class owns which method) | Picking a topology that stays available, consistent-enough, and fast under load and partial failure |
| Deliverable | A class diagram + method signatures a team could implement from | A box diagram + the trade-offs that justify each box |
| Typical duration | 30-45 min | 45-60 min |
| "Correct answer" exists? | No — evaluated on abstraction quality, not a single right design | No — evaluated on trade-off reasoning and whether the design matches the stated numbers |
| Concurrency concern | Thread-safety within one process (locks, atomics, races on shared in-memory state) | Coordination across machines (consensus, replication lag, distributed locks, network partitions) |
| Follow-up shape | "Now add requirement X" — tests whether your abstractions absorb change cheaply | "Now scale 100x" or "now this component fails" — tests whether your trade-offs were load-bearing or decorative |

The two rounds are frequently confused by candidates because both start with
"design a ___" — the tell is whether the prompt is single-machine/in-memory (LLD:
parking lot, elevator, deck of cards) or explicitly networked/at-scale (HLD: URL
shortener, chat system, feed ranking). When a prompt is ambiguous ("design a rate
limiter"), that ambiguity is itself the first thing to clarify — the class-level and
distributed versions of the same noun are graded completely differently, and pivoting
between them mid-interview (starting class-level, then escalating to distributed when
asked "now put this in front of a fleet of servers") is itself a strong, explicitly
recognized signal in both directions.

---

## LLD: Concepts

### Object-Oriented Fundamentals
- **Encapsulation** — hiding internal state behind a class's public interface; the
  reason getters/setters (or their absence) are a deliberate design choice, not
  boilerplate.
- **Abstraction** — exposing *what* a class does, not *how*; interfaces and abstract
  base classes are the primary tool.
- **Inheritance (is-a)** vs. **Composition/Aggregation (has-a)** — the single most
  commonly mis-applied distinction in this round. Default to composition; reach for
  inheritance only when a genuine substitutability relationship exists (Liskov).
- **Polymorphism** — the mechanism that makes the Strategy and State patterns work:
  calling code depends on an interface, not a concrete type.

### SOLID
- **S**ingle Responsibility — one class, one reason to change. The most common
  violation is a `Manager`/`System` god-class doing assignment, payment, *and*
  notification.
- **O**pen/Closed — extend behavior via new implementations of an interface, not by
  editing existing code (an `if/elif` chain that grows with every new requirement is
  the anti-pattern to name).
- **L**iskov Substitution — a subclass must be usable anywhere its parent is expected,
  with no surprising behavior.
- **I**nterface Segregation — several small interfaces over one fat one.
- **D**ependency Inversion — high-level coordinators depend on abstractions, not
  concrete implementations; this is what makes the Open/Closed follow-up cheap.

### Design Patterns (by category — know the shape, not just the name)
- **Creational** — how objects get built.
  - *Singleton* — exactly one instance (a `ParkingLot`, an `ElevatorSystem`
    coordinator); watch for thread-safety in its construction.
  - *Factory / Abstract Factory* — centralize object creation so callers don't
    depend on concrete classes (`VehicleFactory` returning `Car`/`Motorcycle`).
  - *Builder* — construct a complex object step by step (an object with many
    optional fields).
- **Structural** — how objects compose.
  - *Adapter* — make an incompatible interface usable without changing it.
  - *Decorator* — attach behavior to an object dynamically (a coffee-order
    add-ons problem is the canonical prompt).
  - *Composite* — treat individual objects and compositions of objects uniformly
    (a filesystem: files and folders share an interface).
  - *Facade* — a simple interface over a complex subsystem.
  - *Proxy* — a stand-in controlling access to another object (lazy loading,
    access control, caching).
- **Behavioral** — how objects communicate.
  - *Strategy* — swap an algorithm at runtime without touching the caller (spot
    assignment, elevator dispatch, rate-limiting algorithm). **The single
    most-reused pattern in this round.**
  - *State* — model each state as its own class implementing a shared interface,
    with a context object delegating to the current one (vending machine,
    elevator, traffic light, media player). **The second most-reused pattern.**
  - *Observer* — one-to-many notification when state changes (a stock ticker, a
    pub/sub within one process).
  - *Command* — encapsulate a request as an object (undo/redo stacks, a job
    queue).
  - *Chain of Responsibility* — pass a request along a chain of handlers until one
    handles it (middleware, an approval workflow, a logging-level filter).
  - *Template Method* — define an algorithm's skeleton in a base class, defer
    specific steps to subclasses.
  - *Iterator* — traverse a collection without exposing its internal structure.

### Concurrency (single-process)
- **Race conditions** — two threads competing for the same shared, mutable resource
  (the last parking spot, the last inventory unit); naming where one could occur is
  expected at senior+ even if you don't fully implement the fix.
- **Locks / mutexes**, **atomic compare-and-swap**, and **thread-safe collections** as
  the standard toolkit for guarding a critical section.
- **Producer-consumer** — a bounded queue between threads producing and consuming work
  (a print spooler, an order-processing pipeline) is a recurring concurrency-flavored
  LLD shape.

### Process / Evaluation Criteria
1. Clarify requirements and actors before naming a class.
2. Identify entities (nouns) → candidate classes.
3. Identify actions (verbs) → methods, and decide *which class owns each method*.
4. Draw relationships (is-a / has-a-composition / has-a-aggregation).
5. Apply SOLID deliberately, narrating which principle justifies each choice.
6. Handle edge cases and concurrency explicitly.
7. Anticipate and walk through the "now add X" extension follow-up.

*(Full step-by-step treatment with worked reasoning: [`OOD_FRAMEWORK.md`](OOD_FRAMEWORK.md).)*

## LLD: The Question Bank

Grouped by the underlying shape — recognizing the shape is most of the battle, since the
same handful of patterns (Strategy, State, resource-allocation) recur under dozens of
different nouns.

**State-machine-shaped systems** (model each state as a class, a context object
delegates to the current one):
- Vending machine
- Elevator system
- Traffic light controller
- Media player (play/pause/stop/buffering)
- Order lifecycle (placed → confirmed → shipped → delivered → cancelled)
- ATM / bank transaction machine
- Turnstile / gate access controller

**Resource-allocation systems** (find-and-lock an available resource; assignment
strategy + concurrency safety are the crux):
- Parking lot
- Hotel/room booking system
- Meeting room / conference room booking
- Movie ticket booking system (seat selection under contention)
- Airline seat reservation
- Car rental system
- Library management system (book checkout/return, holds)
- Restaurant table reservation

**Data-structure-as-a-system** (looks like a `dsa_prep/` question but graded as LLD
because the ask is a class with API design + pluggable policy, not a bare function):
- LRU cache / LFU cache (pluggable eviction policy)
- Rate limiter (pluggable algorithm — token bucket, sliding window, fixed window)
- In-memory key-value store with TTL
- Concurrent/thread-safe HashMap
- A generic object pool

**Simulation / game-rule systems** (entity modeling + rule enforcement, often with a
built-in state machine for turns/rounds):
- Tic-tac-toe
- Chess (or a simplified subset — move validation is the usual focus)
- Deck of cards / card game framework (Blackjack, Poker hand evaluator)
- Snake and Ladder
- Connect Four
- Bowling score tracker

**Workflow / notification / orchestration systems** (Observer, Command, Chain of
Responsibility show up here):
- Notification system (multi-channel: email/SMS/push, via Observer or Strategy)
- Logging framework (log levels via Chain of Responsibility, pluggable sinks)
- Task scheduler / cron-like job runner (single process)
- Expense-splitting system (Splitwise-style: who owes whom, settle-up logic)
- Food delivery / ride-booking matching **at the class level** (matching a request to
  a driver/courier object — the class-level twin of the HLD dispatch question)
- Undo/redo command stack for an editor
- Chat application **at the class level** (message/user/room classes, not the
  distributed delivery guarantees — that's the HLD version)

**Design-a-mini-framework prompts** (broader scope, closer to a take-home in spirit):
- File system / directory structure (Composite pattern)
- Text editor with plugins (Decorator/Strategy)
- Search/autocomplete trie **as a class** (contrast with the distributed HLD version)
- Vending-machine-style inventory system for a retail POS

---

## HLD: Concepts

### Scale & Estimation
- **Back-of-the-envelope math** — QPS (read and write, estimated *separately*, since
  their ratio drives most decisions), storage growth, bandwidth. The number that
  reframes the problem (e.g. "the hot set fits in RAM") is usually worth more than any
  individual component choice.
- **Vertical vs. horizontal scaling** — bigger machine vs. more machines; almost every
  HLD answer commits to horizontal scaling early and the rest of the design follows.
- **Requirements split: functional vs. non-functional** — turning clarifying questions
  into an explicit FR/NFR list is what makes later trade-offs traceable to a stated
  requirement instead of an unstated preference.

### Data Layer
- **SQL vs. NoSQL** — schema rigidity and transactional guarantees vs. horizontal
  scalability and flexible schema; the decision should follow from access patterns
  (joins and transactions → SQL; huge write volume and simple key-based lookups →
  NoSQL), not familiarity.
- **Indexing** — the read-speed/write-cost/storage trade-off underlying every "how do
  we query this fast" answer.
- **Sharding / partitioning** — splitting data across nodes by a key (hash-based,
  range-based, or geography-based), and the resulting cross-shard query and
  re-sharding pain.
- **Replication** — leader-follower vs. multi-leader vs. leaderless; read scaling,
  failover, and the replication-lag/staleness cost that comes with it.
- **Consistent hashing** — minimizes reshuffling when nodes are added/removed; the
  standard mechanism behind both sharded databases and distributed caches.

### Consistency & Coordination
- **CAP theorem** — under a network partition, choose consistency or availability;
  every distributed data-layer decision is this trade-off in a different costume.
- **PACELC** — CAP's extension: even *without* a partition, there's still a
  latency/consistency trade-off to make.
- **Consistency models** — strong, eventual, causal, read-your-writes; matching the
  model to what the *business* actually requires (not defaulting to strong
  everywhere) is a staff-level signal.
- **Consensus (Paxos/Raft)** — how multiple nodes agree on one truth; expensive, so
  the stronger move is usually designing the need for consensus out of the system
  (e.g., partitioned key spaces that make conflicts structurally impossible) rather
  than reaching for it by default.
- **Idempotency** — safely retrying a request without duplicating its effect;
  foundational to any at-least-once delivery system (payments, message queues).

### Serving & Communication
- **Load balancing** — algorithms (round robin, least connections, consistent
  hashing) and layers (L4 vs. L7, client-side vs. server-side).
- **Caching** — cache-aside, write-through, write-back; eviction policies (LRU/LFU);
  the cache-invalidation problem specifically (the classic "two hard things in
  computer science" one).
- **CDN** — pushing static/cacheable content to the edge; matters whenever latency to
  a global audience is a stated requirement.
- **Message queues / event streaming** — decoupling producers from consumers,
  smoothing load spikes, enabling async processing; at-least-once vs. exactly-once
  delivery semantics.
- **API design** — REST vs. gRPC vs. GraphQL, pagination, rate limiting at the API
  boundary, versioning.
- **Rate limiting at scale** — distributed counting under a strict latency budget;
  usually resolved via local enforcement + async global reconciliation rather than a
  synchronous global check.

### Architecture & Reliability
- **Microservices vs. monolith** — organizational and deployment trade-offs as much as
  technical ones; service boundaries should track team/domain boundaries.
- **Fault tolerance** — redundancy, health checks, circuit breakers, graceful
  degradation, retry-with-backoff.
- **Failure-mode reasoning** — naming what breaks first under load, under a single
  node failure, under a network partition, and stating the *chosen* failure direction
  (fail open vs. fail closed) explicitly, since that choice is rarely universal — it
  depends on which failure mode does less harm for that specific system.
- **Observability** — metrics, logging, tracing, and *which SLO* (p50/p99 latency,
  availability, durability) the design is actually being held to.

### Process / Evaluation Criteria
1. Clarify requirements — scope, scale, consistency needs — before drawing boxes.
2. State functional and non-functional requirements explicitly.
3. Back-of-the-envelope estimation, and let a specific number reframe the design.
4. High-level design (the box diagram), time-boxed deliberately so there's real time
   left for the deep-dive.
5. Deep-dive into the one or two components that are actually hard (not a uniform
   pass over every box).
6. Trade-offs stated as a table or explicit A-vs-B reasoning, not buried in prose.
7. Failure modes and bottlenecks named proactively, before being asked.

*(Full staff-vs-senior altitude framing:
[`system_design_foundation/prerequisite_concepts/00_staff_level_signal.md`](../system_design_foundation/prerequisite_concepts/00_staff_level_signal.md).)*

## HLD: The Question Bank

Grouped by the underlying system category — again, the shape repeats far more than the
noun changes.

**Social / feed / content systems:**
- Design a social media feed (Twitter/X-style, fan-out and ranking)
- Design Instagram/photo-sharing (media storage, feed, follow graph)
- Design a news feed ranking system
- Design a comments/reactions system at scale

**Messaging / real-time systems:**
- Design a chat system (WhatsApp/Messenger-style: delivery guarantees, ordering,
  presence)
- Design a notification system at scale (push/email/SMS fan-out)
- Design a live-comments / real-time collaboration feature (e.g. shared doc editing)
- Design a video conferencing system

**Location / matching / real-time dispatch:**
- Design a ride-hailing dispatch system (Uber/Lyft-style: geospatial indexing,
  real-time matching)
- Design a food-delivery system
- Design a proximity/nearby-search service ("find nearby X")

**Infra building blocks (often asked standalone, not as a product):**
- Design a distributed cache (Redis Cluster-style)
- Design a distributed message queue (Kafka-style)
- Design a rate limiter at global scale
- Design a distributed unique-ID generator (Snowflake-style)
- Design a distributed lock service
- Design a distributed job scheduler / cron
- Design a key-value store
- Design a distributed file storage system (S3-style)
- Design a service registry / service discovery system
- Design an API gateway

**Content delivery / media:**
- Design a video streaming service (YouTube/Netflix-style: transcoding, adaptive
  bitrate, CDN)
- Design a live video streaming/broadcast system
- Design a content delivery network from scratch

**Search / discovery:**
- Design a search autocomplete/typeahead system
- Design a web crawler
- Design a full-text search engine
- Design a recommendation system

**Transactional / consistency-critical systems:**
- Design a URL shortener
- Design a payment processing system
- Design a ticket-booking system at scale (concert/flight/event, high-contention
  writes)
- Design a distributed counter (e.g. like/view counter at scale)
- Design an inventory management system for e-commerce
- Design a wallet/ledger system (double-entry bookkeeping, idempotent transfers)

**Analytics / batch systems:**
- Design a log aggregation and analytics pipeline
- Design a metrics/monitoring system (time-series ingestion at scale)
- Design an ad click-tracking and billing system
- Design a distributed web analytics counter (uniques, approximate cardinality)

---

## Same Noun, Different Round

A handful of prompts appear in *both* banks above, and recognizing which round you're in
—or explicitly naming the pivot— is itself a signal:

| Noun | LLD version | HLD version |
|---|---|---|
| Rate limiter | A `RateLimiter` class with a pluggable algorithm, imported into one process | A rate-limiting *service* in front of a fleet of API servers, enforcing one global limit under a strict latency budget |
| Chat system | `Message`/`User`/`Room` classes and their relationships | Persistent connection management, delivery guarantees, and message ordering across a distributed fleet |
| Cache | An in-process LRU/LFU cache class with a pluggable eviction policy | A distributed cache cluster: consistent hashing, replication, hot-key handling, cache stampede |
| Ride/food dispatch | Matching a request object to a driver/courier object within one process | Geospatial indexing and real-time matching across a fleet of distributed servers |
| Search/autocomplete | A trie class with insert/search methods | A distributed trie serving ranked, real-time-updated suggestions under a brutal latency budget |

If a prompt is ambiguous, ask which version is wanted before designing — the evaluation
criteria (abstraction quality vs. trade-off-under-scale reasoning) are different enough
that guessing wrong burns the round.

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Unit-of-design framing (good for explaining the distinction itself, e.g. when a
  prompt is ambiguous):** "The fastest way to tell these apart is to ask what the unit
  of design is — if it's a class inside one process, that's LLD and I'm optimizing for
  abstraction quality and extensibility; if it's a service or set of services across
  machines, that's HLD and I'm optimizing for trade-offs under scale and partial
  failure. When a prompt like 'rate limiter' is genuinely ambiguous, I'd ask which one
  is wanted rather than guess, since the evaluation criteria are different enough that
  guessing wrong burns the round."
- **Pattern-recognition framing (good for signaling depth quickly in either round):**
  "Both of these rounds have a small number of recurring shapes underneath a large
  number of different nouns — in LLD it's usually a state machine, a
  resource-allocation problem, or a data-structure-as-a-class; in HLD it's usually a
  fan-out/feed problem, a real-time-matching problem, or an infra building block.
  Naming the shape up front, the same way I'd name a DSA pattern, shows I'm
  pattern-matching against known structure rather than deriving cold."
- **Evaluation-criteria framing (good for explaining what 'good' looks like in
  either round, e.g. to a less experienced peer):** "Neither round has one correct
  answer — LLD is graded on how few existing classes a plausible new requirement would
  force you to touch, and HLD is graded on whether your trade-offs were actually
  load-bearing when the numbers changed, not on whether you drew the 'right' boxes."

### Vocabulary Builder

- **unit of design** (n. phrase) — the granularity a round is actually evaluating at
  (class vs. service); the fastest test for which round you're in.
- **load-bearing trade-off** (n. phrase) — a design decision that would actually break
  something if reversed, as opposed to one stated for its own sake; the follow-up
  question ("now scale 100x," "now this fails") is what tests which trade-offs were
  load-bearing.
- **"…is the class-level twin of…"** — a fluent way to name that an LLD prompt and an
  HLD prompt share a noun but not a grading rubric (e.g. a class-level rate limiter vs.
  a distributed one).
- **shape** (n., informal) — the recurring underlying structure beneath a surface-level
  prompt (state machine, resource allocation, fan-out); naming the shape signals
  pattern recognition rather than first-principles derivation under time pressure.
- **"…said explicitly, not buried in prose"** — a reusable phrase for describing why a
  trade-offs table or an explicit A-vs-B framing outperforms narrating the same
  reasoning inline.

---

**See also:** [`OOD_FRAMEWORK.md`](OOD_FRAMEWORK.md) for the full LLD process walkthrough,
and [`../system_design_practice/README.md`](../system_design_practice/README.md) /
[`../system_design_foundation/prerequisite_concepts/00_staff_level_signal.md`](../system_design_foundation/prerequisite_concepts/00_staff_level_signal.md)
for the full HLD process and staff-altitude framing.
