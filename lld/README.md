# Low-Level Design (LLD) / Object-Oriented Design Prep

The fifth track alongside `dsa_prep/` (algorithms), `system_design/` (ML/LLM systems),
`system_design_practice/` (general distributed systems), and `security/`. This one covers
the **class-design round** — "design a parking lot," "design an elevator system" — which
tests OOP judgment (SOLID, design patterns, extensibility) rather than distributed-systems
trade-offs or algorithmic complexity.

Run `make docs` from the repo root to serve all five sites together, or `make lld-serve`
for this one individually (http://127.0.0.1:8004).

Each problem folder has:

- `problem.md` — requirements, the core entities/relationships, which design pattern(s)
  apply and why, the class design itself, and the extensibility follow-up an interviewer
  would ask next.
- `solution.py` — a working Python implementation of the class design, runnable with a
  small demo at the bottom (`python3 solution.py`).

Start with [`OOD_FRAMEWORK.md`](OOD_FRAMEWORK.md) — a problem-agnostic writeup of how to
approach *any* LLD prompt (the step-by-step process, SOLID applied concretely, the
recurring pattern shapes, and common pitfalls). Read it once; it's what makes the reasoning
in every problem below feel inevitable instead of memorized, the same way `PATTERN.md`
works in `dsa_prep/`.

## How to use this

1. Read `OOD_FRAMEWORK.md` first to get the general process and vocabulary in your head.
2. Pick a problem below, read only the **Requirements** section of its `problem.md`, and
   try to design the class hierarchy yourself on paper/whiteboard before reading further —
   this round is evaluated on your derivation, not your final answer.
3. Compare against the rest of `problem.md`, then read `solution.py` to see it as working
   code.
4. Run the solution: `python3 lld/<NN>_<name>/solution.py`.
5. For each problem, try answering the "extension follow-up" named at the end of its
   `problem.md` *before* looking at how the existing design accommodates it — that's the
   real test of whether the abstractions were chosen well.

## Problems (in suggested order)

| # | Folder | Problem | Core pattern(s) |
|---|--------|---------|------------------|
| 1 | `01_parking_lot/` | Parking Lot System | Strategy (spot assignment), class hierarchy |
| 2 | `02_elevator_system/` | Elevator System | State (elevator state), Strategy (dispatch) |
| 3 | `03_vending_machine/` | Vending Machine | State pattern (textbook example) |
| 4 | `04_lru_cache/` | LRU Cache (as a designed class, not just an algorithm) | Composition, Strategy (eviction policy) |
| 5 | `05_rate_limiter/` | Rate Limiter (class-level, pluggable algorithms) | Strategy, Interface segregation |

Order follows increasing pattern-combination complexity: parking lot is mostly entity
modeling, the elevator and vending machine problems introduce the State pattern, and the
last two show that "design a data structure" prompts get graded as LLD once the ask is
API design + extensibility rather than a single function.

Note: `05_rate_limiter/` is the **class-level** design question ("design a `RateLimiter`
you'd import into one process") — for the **distributed** version ("design a rate limiter
service in front of a fleet of API servers"), see
[System Design Practice: Design a Rate Limiter at Global Scale](http://127.0.0.1:8002/07_design_rate_limiter_at_scale/tutorial/).
Interviewers sometimes ask the class-level version first, then escalate to the distributed
one as a follow-up — recognizing that pivot in the room is itself a signal.

## Status

**5 problems across the core recurring LLD shapes: entity modeling, state machines, and
data-structure-as-a-class.** Every `solution.py` has been run and its demo executes
cleanly.

- [x] 01_parking_lot
- [x] 02_elevator_system
- [x] 03_vending_machine
- [x] 04_lru_cache
- [x] 05_rate_limiter
