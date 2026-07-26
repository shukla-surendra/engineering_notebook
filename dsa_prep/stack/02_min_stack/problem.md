# 2. Min Stack

**Difficulty:** Medium
**Topic:** Stack
**Pattern:** Auxiliary stack tracking running minimum

## Problem
Design a stack that supports `push`, `pop`, `top`, and retrieving the minimum element, all
in O(1) time.

## Examples
```
push(-2); push(0); push(-3)
getMin() -> -3
pop()
top()    -> 0
getMin() -> -2
```

## Approach
Maintain a second stack alongside the main one that tracks the minimum "so far" at each
depth. When pushing `x`, push `min(x, current_min)` onto the min-stack (or just `x` if the
min-stack is empty). When popping, pop from both stacks in lockstep. `getMin()` is just
the min-stack's top — always O(1).

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Auxiliary stack tracking running minimum**,
which itself belongs to the broader **Stack** family of techniques. If the specific
trick above feels like it came out of nowhere, that's the signal to step back and read
[`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of problems in
general (not just this one), the reusable template you can write from memory, the usual
variations, and the mistakes people make applying it. Coming back to re-read this
problem's approach afterward should make the specific choices here feel inevitable
rather than clever.

## Complexity
- Time: O(1) for every operation
- Space: O(n)

## Solution
Runnable, with sample test cases at the bottom (`python3 stack/02_min_stack/solution.py`):

```python
--8<-- "stack/02_min_stack/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The naive `getMin()` just
  scans the whole stack every call — O(n) per call. I'd name that first, then say the
  fix is to *precompute and cache* the running minimum at every depth, so the query
  becomes O(1) at the cost of O(n) extra space in a parallel stack."
- **Invariant framing (good for explaining the auxiliary stack precisely):** "The
  invariant is: the min-stack's top always equals the minimum of every element currently
  on the main stack, at that exact depth. Popping both stacks in lockstep is what
  preserves that invariant — if I only popped the main stack, the min-stack would go
  stale."
- **Generalization framing (good for signaling pattern recognition):** "This is the
  'auxiliary stack for a running aggregate' pattern — the same idea generalizes to
  tracking a running max, running sum, or any aggregate that needs to 'undo' cleanly when
  you pop, which a plain recomputation approach can't do efficiently."

### Vocabulary Builder

- **lockstep** (adv./adj.) — two structures updated together, one operation at a time, so
  they stay synchronized. *"I pop both stacks in lockstep so the min-stack never goes
  stale relative to the main one."*
- **auxiliary structure** (n. phrase) — a secondary data structure maintained alongside
  the primary one specifically to cache information that would otherwise cost a rescan.
- **"trades memory for O(1) queries"** — a reusable phrase justifying any cache-alongside
  approach: you pay O(n) space once to make every future query O(1).
- **amortized** (adj.) — worth contrasting here: this isn't amortized O(1), it's *worst-
  case* O(1) per operation, since every operation is truly constant time, not just
  averaged over a sequence.
