# 2. Elevator System

**Difficulty:** Hard
**Topic:** Low-Level Design
**Pattern:** State (elevator motion) + Strategy (dispatch)

## Requirements

Design a multi-elevator system for a building that:
- Accepts a **hall call**: a person on some floor requests a pickup in a direction
  (up/down).
- Accepts a **car call**: someone already inside an elevator requests a destination floor.
- Dispatches hall calls to a suitable elevator — not always the nearest one, since an
  elevator already moving away from the request is a worse choice than an idle one
  farther away.
- Each elevator moves one floor at a time, serving requests roughly in the order they lie
  along its current direction of travel before reversing (the classic **LOOK/SCAN**
  elevator algorithm — service everything in the current direction before turning
  around, rather than round-robin').

Clarifying questions worth asking: how many floors/elevators? Is capacity (max
passengers) in scope? This solution assumes a fixed elevator count, floors as plain
integers, and no capacity limit — enough to have a real dispatch decision without
over-scoping a 40-minute round.

## Core entities

- **`Elevator`** — owns its `current_floor`, its pending `destinations`, and delegates
  *how it responds to a request and how it advances one step* to its current
  **`ElevatorState`**. This is the State pattern: `IdleState`, `MovingUpState`,
  `MovingDownState` each implement `handle_request`/`step` differently, and `Elevator`
  itself contains no direction-specific `if/elif` logic — it just asks its current state
  what to do. Compare this to `03_vending_machine/problem.md`, the textbook version of
  this same pattern.
- **`ElevatorState`** (interface) — `IdleState.handle_request` picks a direction and
  transitions; `MovingUpState`/`MovingDownState.step` advances one floor, drops any
  destination reached, and decides whether to keep going, reverse, or go idle once no
  destinations remain in the current direction — this is the LOOK behavior.
- **`DispatchStrategy`** (interface) — decides which `Elevator` answers a given hall call.
  `NearestIdleOrSameDirectionStrategy` prefers an idle car; otherwise a car already moving
  the right way and not yet past the requested floor; otherwise falls back to nearest.
  Swapping in a different dispatch policy (e.g., weighting by current passenger load) is a
  new class, not a rewrite of `ElevatorSystem`.
- **`ElevatorSystem`** — the coordinator; owns all elevators and the chosen dispatch
  strategy, and is the only place that knows there's more than one elevator.

## Relationships

`ElevatorSystem` **has** many `Elevator`s (composition) and **uses** a
`DispatchStrategy` (composition over inheritance — swappable, not a superclass).
`Elevator` **delegates to** its current `ElevatorState` rather than **is-a** state —
this is a deliberate choice: an elevator doesn't inherit from `MovingUpState`, it *holds
a reference* to one and swaps it at runtime, which is what lets the direction change
mid-flight without changing the elevator's identity or type.

## Why State here, not a flag

A tempting shortcut is a `direction: str` field on `Elevator` with `if direction ==
"up": ...` scattered across methods. That's the same information encoded worse: every
new behavior (e.g., "doors held open," "out of service") means editing every method that
branches on direction. The State pattern makes each state's behavior local to its own
class — adding `OutOfServiceState` means adding one class, not editing `Elevator`.

## Extension follow-up

*"Now add a maintenance mode — a specific elevator should stop accepting new requests and
finish only its current destinations before going out of service."* With this design: add
an `OutOfServiceState` (rejects `handle_request`, `step` behaves like the current
`Moving*State` until `destinations` empties, then stays out of service instead of going
`Idle`), and have `DispatchStrategy` skip elevators in that state when selecting a car for
a new hall call. No change to `IdleState`, `MovingUpState`, `MovingDownState`, or
`ElevatorSystem`'s core loop.

## Solution
Runnable, with sample test cases at the bottom (`python3 lld/02_elevator_system/solution.py`):

```python
--8<-- "02_elevator_system/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **State-vs-flag framing (the core design decision to narrate first):** "I'd reject a
  `direction` flag with branching logic early and explain why: every new elevator mode
  becomes an edit to every branching method. Modeling direction as a `State` object the
  elevator delegates to means a new mode is a new class — I'd say that trade-off out loud
  before writing any code, since it's the single decision that makes or breaks this
  design."
- **Dispatch framing (good for showing you're not just doing nearest-neighbor):** "The
  naive dispatch — send whichever elevator is physically closest — is wrong the moment an
  elevator is already moving away from the request; I'd walk through why an idle elevator
  farther away can be the better choice, which is what pushed dispatch into its own
  Strategy rather than a one-line distance comparison."
- **Generalization framing (good for connecting to the pattern family):** "This is the
  same State pattern as a vending machine or a traffic light — an object whose *legal next
  behaviors* depend entirely on which state it's currently in — and once I recognize that
  shape, the class boundaries mostly follow: one class per state, one method per action."

### Vocabulary Builder

- **LOOK / SCAN algorithm** (n. phrase) — the elevator-scheduling strategy of servicing
  all requests in the current direction before reversing, rather than answering calls in
  arrival order; the real-world justification for the `step()` logic here.
- **delegate to** (v. phrase) — an object handing off a decision to a collaborator object
  it holds a reference to, rather than deciding itself; `Elevator.step()` delegates to
  `self._state.step(self)` instead of containing the branching logic directly.
- **hall call vs. car call** (n. phrases) — the two distinct request types in any elevator
  system (someone waiting on a floor vs. someone already inside choosing a destination);
  precise vocabulary that signals domain familiarity beyond "elevator gets a request."
- **"…is a new class, not an edit"** — the reusable phrase for demonstrating Open/Closed
  concretely when narrating an extension follow-up.
