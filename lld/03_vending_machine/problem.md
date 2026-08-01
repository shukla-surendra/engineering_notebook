# 3. Vending Machine

**Difficulty:** Medium
**Topic:** Low-Level Design
**Pattern:** State (textbook example)

## Requirements

Design a vending machine that:
- Lets a user select a product by code.
- Accepts coins toward the selected product's price, across multiple insertions.
- Dispenses the product and returns change once enough money has been inserted.
- Lets the user cancel at any point before dispensing and get a refund.
- Refuses to sell a product that's out of stock, and reports sold-out state distinctly
  from "not enough money yet."

This is the canonical example for the **State pattern**, and worth doing even though it
looks simple: the whole point is to feel, by the end, why "one class per state" beats "one
class with a `status` flag and branching everywhere" — a distinction most engineers can
recite but haven't actually felt the pain of *not* doing.

## Core entities

- **`VendingMachineState`** (interface) — `select_product`, `insert_coin`, `dispense`,
  `cancel`. Four concrete states implement it: `IdleState` (nothing selected),
  `HasSelectionState` (product picked, insufficient funds), `HasEnoughMoneyState` (ready
  to dispense), `SoldOutState`. Each state defines what's *legal* right now — e.g.
  `IdleState.insert_coin` raises, because you can't pay before picking a product.
- **`VendingMachine`** (context) — holds `balance`, `selected_code`, `inventory`, and a
  reference to its current `VendingMachineState`. Every public method
  (`select_product`/`insert_coin`/`dispense`/`cancel`) is a **one-line delegate** to the
  current state — `VendingMachine` itself contains zero business logic, which is the
  signature of a correctly-applied State pattern.
- **`Inventory`** / **`Product`** — plain data plus lookup/decrement; deliberately dumb,
  same reasoning as `ParkingSpot` in problem 1.

## Why not a status flag

The alternative design — `status: Literal["idle", "selected", "paid", "sold_out"]` on
`VendingMachine`, with `if status == "selected": ...` in every method — encodes the exact
same information, but every method has to know about every status. Adding a fifth state
(say, `MaintenanceState`) means finding and editing every branch across every method. With
the State pattern, adding a state is **adding one class** that implements the same four
methods; the methods that already exist don't change at all. This is the same argument
made for `Elevator` in problem 2 — recognizing it twice is the point: once you've felt this
trade-off in two different problems, you'll reach for State by instinct, not by recalling
"the vending machine example."

## State transition table

| From \ Event | select_product | insert_coin | dispense | cancel |
|---|---|---|---|---|
| Idle | → HasSelection (or SoldOut if empty) | error | error | no-op |
| HasSelection | update selection | accumulate; → HasEnoughMoney if paid in full | error | refund, → Idle |
| HasEnoughMoney | error (already committed) | accumulate (overpay) | dispense, refund change, → Idle or SoldOut | refund, → Idle |
| SoldOut | → HasSelection if a *different* product has stock, else error | error | error | no-op |

Walking an interviewer through a table like this — not necessarily this exact one — is
often more convincing than the code, because it proves you enumerated the full transition
space instead of only handling the happy path.

## Extension follow-up

*"Now support paying by card in addition to coins, and the machine should be able to
partially refund coins while a card charge covers the rest."* With this design: introduce
a `PaymentMethod` interface (mirroring `PaymentProcessor` from problem 1) that
`insert_coin`/a new `charge_card` method delegate to for tracking amount paid; the state
classes themselves barely change, since they already only care about *whether* enough has
been paid, not *how*. The states' logic is payment-method-agnostic by construction, which
is what makes this extension cheap.

## Solution

### Python
Runnable, with sample test cases at the bottom (`python3 lld/03_vending_machine/solution.py`):

```python
--8<-- "03_vending_machine/solution.py"
```

### Rust
Same self-referential-mutation issue as the elevator's motion state, translated the same
way — a closed `enum` + `match` in place of a trait-object state hierarchy. Runnable via
`cd lld/03_vending_machine/vending_machine_rusty && cargo test`:

```rust
--8<-- "03_vending_machine/vending_machine_rusty/src/main.rs"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **State-vs-flag framing (the crux of this problem):** "I'd explicitly reject a
  `status` flag with branching logic and explain why: every method would need to know
  about every status, and adding a state means editing every method. Modeling each status
  as its own class means adding a state is adding one class, full stop — I'd say that
  trade-off out loud before writing code, since the whole problem is designed to test
  whether I reach for this instinctively."
- **Transition-table framing (good for proving completeness, not just happy-path
  code):** "Before coding I'd sketch the state transition table — every state crossed with
  every event — because that's what catches the case I'd otherwise miss silently, like
  'what does cancel do while idle' or 'what does insert-coin do on a sold-out machine.'
  Enumerating the table is cheap; discovering a missing transition during a demo isn't."
- **Generalization framing (good for connecting to problem 2):** "This is the same State
  pattern as the elevator system, just with a smaller state space — recognizing the same
  shape twice in one prep session is the actual skill being tested, not memorizing either
  example individually."

### Vocabulary Builder

- **context object** (n. phrase) — in the State pattern, the object (here,
  `VendingMachine`) that holds a reference to its current state and delegates all
  behavior to it, containing no branching logic of its own.
- **state transition table** (n. phrase) — an explicit enumeration of every (state, event)
  pair and its result; a fast way to prove design completeness to an interviewer without
  walking through every line of code.
- **happy path** (n. phrase) — the sequence of events assuming everything goes right (pick
  product, pay exactly, dispense); worth naming explicitly when you pivot to discussing
  edge cases, since it signals you're deliberately expanding scope, not missing it.
- **"…is the signature of a correctly-applied [pattern]"** — a reusable phrase for
  justifying that a pattern was applied well, not just applied; here, zero business logic
  left in the context class is the tell.
