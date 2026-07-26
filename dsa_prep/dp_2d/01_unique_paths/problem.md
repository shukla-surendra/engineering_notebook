# 1. Unique Paths

**Difficulty:** Medium
**Topic:** 2-D Dynamic Programming
**Pattern:** Grid DP, each cell = sum of cell above + cell to the left

## Problem
A robot sits at the top-left of an `m x n` grid and can only move down or right. Return
the number of distinct paths to reach the bottom-right corner.

## Examples
```
Input: m = 3, n = 7 -> 28
Input: m = 3, n = 2 -> 3
```

## Approach
`paths(r, c) = paths(r-1, c) + paths(r, c-1)` — the number of ways to reach a cell is the
sum of ways to reach the cell above and the cell to the left, since those are the only two
places you could have moved from. The first row and first column each have exactly 1 path
(straight line). This can be computed with a single 1-D array reused across rows, updating
in place left to right.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Grid DP, each cell = sum of cell above + cell
to the left**, which itself belongs to the broader **2-D Dynamic Programming** family of
techniques. If the specific trick above feels like it came out of nowhere, that's the
signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to
recognize this family of problems in general (not just this one), the reusable template
you can write from memory, the usual variations, and the mistakes people make applying
it. Coming back to re-read this problem's approach afterward should make the specific
choices here feel inevitable rather than clever.

## Complexity
- Time: O(m·n)
- Space: O(n) using a rolling 1-D array

## Solution
Runnable, with sample test cases at the bottom (`python3 dp_2d/01_unique_paths/solution.py`):

```python
--8<-- "dp_2d/01_unique_paths/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "I'd start with the recursive
  definition — from any cell you either came from above or from the left, so
  `paths(r,c) = paths(r-1,c) + paths(r,c-1)`. Called naively that's exponential because the
  same cell gets recomputed from many paths; caching those calls, or building the table
  bottom-up instead, collapses it to O(m·n)."
- **Invariant framing (good for explaining why the fill order matters):** "The invariant is
  that by the time I compute a cell, both its 'above' and 'left' neighbors are already
  final answers — that's guaranteed by filling row by row, left to right, with the first
  row and column seeded at 1 since there's only one straight-line path to any edge cell."
- **Generalization framing (good for signaling this is a known shape, not a one-off):** "This
  is the canonical Grid DP template — `dp[cell] = combine(neighbors)` — the same shape that
  generalizes to min-cost-path problems by swapping the combine function from sum to min. I'd
  name that family out loud rather than presenting this as a bespoke trick."

### Vocabulary Builder

- **recurrence relation** (n.) — an equation defining a value in terms of earlier values in
  the same sequence/table. *"The recurrence here is just 'sum of the neighbor above and the
  neighbor to the left.'"*
- **rolling array** (n. phrase) — reusing a single 1-D array across rows instead of keeping
  the full 2-D table, since each row only needs the row before it.
- **closed-form** (adj.) — a direct formula (here, the binomial coefficient
  `C(m+n-2, m-1)`) instead of iterative computation — worth mentioning to show you see the
  combinatorial identity, even if you'd still code the DP for clarity.
- **"the crux of it is…"** — a clean way to pivot from restating the problem into stating
  the recurrence, without narrating every intermediate thought.
