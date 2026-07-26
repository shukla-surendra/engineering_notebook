# 1. Climbing Stairs

**Difficulty:** Easy
**Topic:** 1-D Dynamic Programming
**Pattern:** Fibonacci-style recurrence

## Problem
You're climbing a staircase of `n` steps. Each move you can climb 1 or 2 steps. How many
distinct ways can you reach the top?

## Examples
```
Input: n = 2 -> 2  (1+1, 2)
Input: n = 3 -> 3  (1+1+1, 1+2, 2+1)
```

## Approach
The number of ways to reach step `n` is the sum of ways to reach `n-1` (then take a
1-step) and ways to reach `n-2` (then take a 2-step) — exactly the Fibonacci recurrence.
Iteratively build up from the base cases (`ways(1)=1`, `ways(2)=2`) using just two
running variables, no need for a full array.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Fibonacci-style recurrence**, which itself
belongs to the broader **1-D Dynamic Programming** family of techniques. If the specific
trick above feels like it came out of nowhere, that's the signal to step back and read
[`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of problems in
general (not just this one), the reusable template you can write from memory, the usual
variations, and the mistakes people make applying it. Coming back to re-read this
problem's approach afterward should make the specific choices here feel inevitable
rather than clever.

## Complexity
- Time: O(n)
- Space: O(1)

## Solution
Runnable, with sample test cases at the bottom (`python3 dp_1d/01_climbing_stairs/solution.py`):

```python
--8<-- "dp_1d/01_climbing_stairs/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The naive approach recurses on
  every choice at every step — O(2^n) — and I'd name that complexity out loud before
  pointing out it re-solves the same subproblem repeatedly, which is exactly the signal for
  DP."
- **Invariant framing (good for explaining the space-optimized version):** "The invariant is
  that `prev1` and `prev2` always hold ways(n-1) and ways(n-2) relative to the current step —
  so I only need two rolling variables, not a full array, once I trust the recurrence is
  correct."
- **Pattern-recognition framing (good for naming the family):** "This is the canonical
  Fibonacci-style recurrence — I'd say that explicitly, since recognizing it early is what
  lets me skip straight to two running variables instead of building a full DP array first."

### Vocabulary Builder

- **recurrence** (n.) — the rule relating the answer at step `n` to smaller steps; stating
  it in words ("ways to reach `n` is ways to reach `n-1` plus ways to reach `n-2`") before
  coding shows derivation, not memorization.
- **overlapping subproblems** (n. phrase) — the same smaller question gets asked repeatedly
  in a naive recursion; the core reason DP beats brute force here.
- **"…the crux of it is recognizing the recurrence before writing any code"** — a reusable
  way to open a DP explanation without diving straight into syntax.
- **base case** (n.) — the smallest input where the answer is known outright (`n=1`, `n=2`
  here); getting this wrong is the most common source of off-by-one bugs in DP.
