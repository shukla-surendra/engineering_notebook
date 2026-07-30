# Framework: Object-Oriented / Low-Level Design (LLD)

## What problem does this solve?

System design (`../system_design_practice/`) asks "how do these services talk to each
other across machines" — the unit of design is a service, and the hard part is the
network, consistency, and scale. LLD asks a different question: "how do these *classes*
talk to each other inside one process" — the unit of design is a class or interface, and
the hard part is picking abstractions that stay correct and extensible as requirements
grow. It's the round that tests whether you can turn a fuzzy prompt ("design a parking
lot") into a class hierarchy a team could actually build from, not whether you know a
specific algorithm.

## How to recognize it

Signals you're in an LLD round, not a system design or DSA round:
- The prompt is a single-machine, in-memory system: "design a parking lot," "design an
  elevator system," "design a vending machine," "design a deck of cards," "design a
  library management system."
- The interviewer cares about your class diagram, method signatures, and how you'd extend
  the design for a follow-up requirement — not about databases, load balancers, or
  network partitions.
- There's no single "correct" algorithm to find (unlike DSA) — the evaluation is about
  the *quality of the abstractions*: are responsibilities cleanly separated, does the
  design survive a "now add X" follow-up without a rewrite.

## The general template

Work through these steps out loud, in order — skipping straight to code is the single
most common way this round goes wrong, because the interviewer can't see your design
reasoning if the first thing you produce is a class:

1. **Clarify requirements and scope.** List the actors and the core use cases explicitly
   ("a car enters, gets a ticket, parks in an available spot, pays, exits") before naming
   any class. Ask 2-3 clarifying questions that would actually change the design (e.g.
   "multiple entry gates?" "reserved spots?") — this is also where you negotiate scope
   down to something codeable in 30-40 minutes.
2. **Identify the core entities (nouns).** Walk the use cases and pull out the nouns —
   these become your candidate classes. Separate *things with identity and behavior*
   (`ParkingSpot`, `Elevator`) from *pure data* (`Ticket`, `Request`) from *the system
   itself* (`ParkingLot`, `ElevatorSystem` — usually a Singleton or otherwise
   single-instance coordinator).
3. **Identify the actions (verbs) → methods.** Walk the use cases again for verbs
   ("park," "pay," "dispatch," "assign spot") — each becomes a method, and *which class
   owns that method* is often the real design decision (does `Vehicle` know how to park
   itself, or does `ParkingLot` park it? Usually the coordinator does, so the entity stays
   simple).
4. **Draw relationships.** For every pair of related classes, decide: **is-a**
   (inheritance — `Car is-a Vehicle`), **has-a / composition** (`ParkingLot has Levels`,
   and a `Level` can't outlive its `ParkingLot`), or **has-a / aggregation** (`Elevator
   has current Requests`, but a `Request` can exist independently). Getting
   inheritance-vs-composition right here is usually the single biggest quality signal in
   the round.
5. **Apply SOLID deliberately, and say which principle you're applying.** Don't just
   write clean code — narrate it: "I'm making `SpotAssignmentStrategy` an interface here
   so I can swap the assignment algorithm without touching `ParkingLot` — that's
   Open/Closed." This is what turns "I wrote working code" into a design-round signal.
6. **Handle the edge cases and (if relevant) concurrency.** What happens when the system
   is full, when two threads race to grab the last spot/spare part, when a request is
   invalid. At senior+, explicitly naming a race condition and how you'd guard it (a lock,
   an atomic compare-and-swap, a queue) is expected even in a single-process design.
7. **Anticipate the extension follow-up.** Nearly every LLD interview ends with "now add
   X" (e.g., "now add electric vehicle charging spots," "now support multiple elevator
   banks"). If step 5 was done honestly, this should require adding a class/implementing
   an interface, not editing five existing ones — say so explicitly when you get there.

## SOLID, applied concretely (not as a definitions list)

- **Single Responsibility** — a class should have one reason to change. The most common
  violation in this round is a `System`/`Manager` god-class that does assignment,
  payment, *and* notification — split by responsibility even under time pressure.
- **Open/Closed** — favor an interface + multiple implementations over a big
  `if/elif` chain when the thing that varies is an *algorithm* (spot assignment strategy,
  elevator dispatch strategy, rate-limiting algorithm). This is the **Strategy pattern**,
  and it's the single most-reused pattern across LLD interviews.
- **Liskov Substitution** — a subclass must be usable anywhere its parent is expected
  without surprising behavior. If `Motorcycle extends Vehicle` but breaks an assumption
  every other `Vehicle` satisfies (e.g. "every vehicle needs exactly one spot"), that's a
  signal the hierarchy is wrong, not that you need a special case.
- **Interface Segregation** — don't force a class to implement methods it doesn't need.
  Prefer several small interfaces (`Payable`, `Notifiable`) over one fat one.
- **Dependency Inversion** — high-level coordinators (`ParkingLot`) should depend on
  abstractions (`PaymentProcessor` interface), not concrete classes (`CreditCardPayment`)
  — this is what makes step 7's "now add X" cheap.

## Variations you'll see

- **State-machine-shaped systems** (vending machine, elevator, traffic light, media
  player) — model each state as its own class implementing a shared `State` interface,
  with the context object (`VendingMachine`) holding a reference to its current state and
  delegating to it. See `03_vending_machine/problem.md` for a full worked example — this
  is the **State pattern**, and it's the second most-reused pattern after Strategy.
- **Resource-allocation systems** (parking lot, hotel booking, meeting room booking) —
  the core question is always "how do I find and lock an available resource," and the
  interesting design decision is usually the assignment strategy plus concurrency safety
  under contention.
- **Data-structure-as-a-system questions** (LRU/LFU cache, rate limiter) — these look like
  `dsa_prep/` questions but are graded as LLD when the prompt is "design a class," not
  "write a function," because the evaluation includes API design and extensibility
  (pluggable eviction policy, pluggable rate-limiting algorithm), not just the core
  data structure. See `04_lru_cache/problem.md` and `05_rate_limiter/problem.md`.

## Common pitfalls

- **Jumping straight to code.** Always narrate requirements → entities → relationships
  first; a design with no verbal reasoning attached reads as memorized, not derived.
- **One giant class that does everything.** The fastest way to fail SRP and make the
  "now add X" follow-up expensive.
- **Inheritance where composition belongs.** A `Car` is not a kind of `ParkingSpot` — if
  you find yourself inheriting for code reuse rather than a genuine is-a relationship,
  switch to composition.
- **Ignoring concurrency entirely.** Even a single-process design has threads/requests
  racing for a shared resource (the last parking spot, the last inventory unit) — naming
  the race condition, even without fully implementing the fix, is expected at senior+.
- **Skipping the extension follow-up in practice.** Don't just claim your design is
  "extensible" — when you get the follow-up question, actually walk through which class
  you'd add and which you'd leave untouched.

## Complexity characteristics

There's no Big-O here in the traditional sense — the "cost function" being evaluated is
design quality: how many existing classes need to change for a plausible new requirement
(lower is better), and whether responsibilities map cleanly to real-world nouns/verbs from
the prompt.

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Requirements-first framing (the default opening move):** "Before I name a single
  class, I'd restate the use cases as a short list and ask 1-2 clarifying questions that
  would actually change the design — jumping to code without this is the most common way
  this round goes badly, because the interviewer can't evaluate reasoning they never saw."
- **Pattern-recognition framing (good for signaling you're not deriving from scratch):**
  "Most LLD prompts collapse into one of a small number of shapes — a state machine, a
  resource-allocation system, or a data-structure-as-a-class — and naming which shape
  applies up front, the same way I'd name a DSA pattern, shows I'm pattern-matching
  against known structure, not inventing a design cold."
- **Extensibility framing (good for the inevitable 'now add X' follow-up):** "The real
  test of a design isn't whether it handles today's requirements, it's how many existing
  classes I'd have to touch to add tomorrow's. I'd say that explicitly when applying
  Open/Closed — 'I'm making this an interface specifically so the next requirement is an
  addition, not an edit.'"

### Vocabulary Builder

- **composition over inheritance** (n. phrase) — the default preference for "has-a"
  relationships over "is-a" ones unless a true substitutability relationship exists; the
  most commonly cited LLD design heuristic.
- **god class** (n. phrase, informal) — a class that has accumulated too many
  responsibilities (a Single Responsibility Principle violation); naming this out loud
  when you spot the risk shows self-awareness before the interviewer has to point it out.
- **Strategy pattern** / **State pattern** (n. phrases) — the two most-reused design
  patterns in LLD interviews; naming the pattern you're applying, not just the code, is
  the difference between "designed" and "happened to write."
- **"…so the next requirement is an addition, not an edit"** — a reusable phrase for
  justifying an interface/abstraction choice in terms of the Open/Closed Principle without
  reciting the textbook definition.
- **race condition** (n. phrase) — two threads/requests concurrently competing for the
  same shared, mutable resource (the last parking spot, the last unit of inventory);
  naming where one could occur is expected even in a single-process design.
