# 1. Parking Lot System

**Difficulty:** Medium
**Topic:** Low-Level Design
**Pattern:** Strategy (spot assignment) + class hierarchy (vehicle sizing)

## Requirements

Design a parking lot that:
- Has multiple spot types: motorcycle, compact (car), large (bus).
- Accepts a vehicle at entry, finds it a suitable free spot, and issues a ticket.
- Accepts the ticket at exit, computes the fee from duration parked, charges payment, and
  frees the spot.
- A smaller vehicle can use a larger spot (a motorcycle can park in a compact or large
  spot) but not vice versa.
- Reports live availability per spot type.

Clarifying questions worth asking out loud before designing: is there one entry gate or
many (affects whether spot assignment needs to be thread-safe)? Is pricing flat-rate or
tiered by vehicle type? Are spots reservable ahead of time? This solution assumes multiple
concurrent entry points (so assignment must be safe under contention), hourly flat pricing,
and no reservations — the simplest version that still has a real design decision in it.

## Core entities

- **`Vehicle`** (abstract, `Motorcycle`/`Car`/`Bus` subclasses) — knows its own
  `spot_type`. This is an **is-a** hierarchy: every vehicle genuinely is a specialization
  with one differing property, not a case of forcing inheritance for reuse.
- **`ParkingSpot`** — has a `spot_type` and current occupant; knows how to check
  `can_fit(vehicle)` and to park/vacate itself. Deliberately dumb — it doesn't know how
  it gets chosen, only whether it can host a given vehicle.
- **`SpotAssignmentStrategy`** (interface) — decides *which* free spot a vehicle gets.
  `NearestFitStrategy` is the one implementation here (smallest spot that still fits).
  This is the Open/Closed seam: a different assignment policy (e.g., "prefer spots near
  the elevator") is a new class, not a rewrite.
- **`Ticket`** — pure data: which vehicle, which spot, entry/exit time.
- **`PaymentProcessor`** (interface) — `ParkingLot` depends on this abstraction, not a
  concrete payment gateway (Dependency Inversion), so swapping payment providers doesn't
  touch the coordinator.
- **`ParkingLot`** — the single coordinator. Owns the spots, the active tickets, and the
  chosen strategy; every use case (`park_vehicle`, `exit_vehicle`) is a method here, not
  on `Vehicle` or `ParkingSpot` — those stay simple data-plus-a-few-queries objects.

## Relationships

`ParkingLot` **has** `ParkingSpot`s (composition — a spot has no meaning outside its lot).
`ParkingLot` **uses** a `SpotAssignmentStrategy` and a `PaymentProcessor` (composition
over inheritance — both are swappable collaborators, not superclasses). `Vehicle` subtypes
are an **is-a** hierarchy. `Ticket` **references** a vehicle and spot by ID rather than
holding object references, keeping it a clean data record independent of the objects'
lifecycles.

## Concurrency

Two vehicles arriving at the same instant must not both be assigned the last free spot —
that's a race on "find a free spot, then mark it occupied." `park_vehicle` and
`exit_vehicle` take a lock around the read-then-write sequence. In a real multi-process
deployment (not just multi-threaded) this would need to become a database-level
`SELECT ... FOR UPDATE` or an atomic compare-and-swap on the spot's status — worth naming
even though the reference implementation here is single-process.

## Extension follow-up

*"Now add electric vehicle charging spots — a vehicle can optionally request one, and it
should still fall back to a regular spot of the right size if none is free."* With this
design: add `SpotType.EV_COMPACT`/`EV_LARGE` (or an `has_charger: bool` flag on
`ParkingSpot`), and add an `EVAwareStrategy` that tries a charging spot first, falls back
to `NearestFitStrategy`. No change needed to `Vehicle`, `Ticket`, `PaymentProcessor`, or
the locking logic in `ParkingLot` — which is the payoff of putting the assignment logic
behind an interface in the first place.

## Solution

### Python
Runnable, with sample test cases at the bottom (`python3 lld/01_parking_lot/solution.py`):

```python
--8<-- "01_parking_lot/solution.py"
```

### Rust
Same design, translated idiomatically (`Vehicle`/`SpotAssignmentStrategy`/
`PaymentProcessor` become traits; the coordinator's `&mut self` methods replace
solution.py's explicit `threading.Lock` — see the module doc comment for why). Runnable
via `cd lld/01_parking_lot/parking_lot_rusty && cargo test`:

```rust
--8<-- "01_parking_lot/parking_lot_rusty/src/main.rs"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Requirements-first framing:** "Before naming any class I'd restate the flow — vehicle
  enters, gets assigned a spot, gets a ticket, pays and exits — and ask whether entry is
  single- or multi-gate, since that decides whether spot assignment needs to be
  thread-safe. That question changes the design, so I'd ask it before writing anything."
- **Pattern framing (good for justifying the Strategy interface):** "The spot-assignment
  algorithm is exactly the kind of thing that varies independently of the rest of the
  system — nearest-fit today, proximity-to-elevator tomorrow — so I'd pull it behind a
  `SpotAssignmentStrategy` interface from the start rather than hardcoding it into
  `ParkingLot`, which is Open/Closed in practice, not just in theory."
- **Concurrency framing (good for showing senior+ judgment beyond a clean class diagram):**
  "Even though this is a single-process design, I'd flag out loud that assigning the last
  free spot is a genuine race condition between concurrent entries, and that the fix here
  — a lock around find-then-mark — would need to become a DB-level atomic operation the
  moment this runs across multiple processes."

### Vocabulary Builder

- **is-a vs. has-a** (n. phrases) — the two relationship types this design leans on:
  `Vehicle` subtypes are is-a (inheritance), `ParkingLot`'s ownership of spots and
  strategies is has-a (composition); confusing the two is the most common LLD mistake.
- **coordinator class** (n. phrase) — a class (here, `ParkingLot`) that owns the use-case
  methods so simpler entities (`ParkingSpot`, `Vehicle`) can stay narrow and dumb.
- **"…is the payoff of putting that behind an interface"** — a reusable phrase for
  closing out an extension follow-up: naming *why* the earlier design choice made the new
  requirement cheap, not just that it happened to work.
- **atomic compare-and-swap** (n. phrase) — a hardware/DB-level operation that reads and
  conditionally writes a value in one indivisible step; the production-grade fix for the
  race condition a single-process lock only approximates here.
