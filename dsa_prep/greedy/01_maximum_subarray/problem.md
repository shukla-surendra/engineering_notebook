# 1. Maximum Subarray

**Difficulty:** Medium
**Topic:** Greedy
**Pattern:** Kadane's Algorithm

## Problem
Given an integer array `nums`, find the contiguous subarray with the largest sum and
return that sum.

## Examples
```
Input: nums = [-2,1,-3,4,-1,2,1,-5,4] -> 6   ([4,-1,2,1])
Input: nums = [1]                     -> 1
```

## Approach
Kadane's algorithm: track `current_sum`, the best sum of a subarray ending exactly at the
current position. At each element, decide greedily whether extending the previous
subarray is better than starting fresh here: `current_sum = max(num, current_sum + num)`.
Track the running `best` across all positions. The key insight: a negative `current_sum`
can never help a future subarray, so it's always better to restart there.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Kadane's Algorithm**, which itself belongs to
the broader **Greedy** family of techniques. If the specific trick above feels like it
came out of nowhere, that's the signal to step back and read
[`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of problems in
general (not just this one), the reusable template you can write from memory, the usual
variations, and the mistakes people make applying it. Coming back to re-read this
problem's approach afterward should make the specific choices here feel inevitable
rather than clever.

## Complexity
- Time: O(n)
- Space: O(1)

## Solution
Runnable, with sample test cases at the bottom (`python3 greedy/01_maximum_subarray/solution.py`):

```python
--8<-- "greedy/01_maximum_subarray/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The naive approach checks
  every subarray — O(n²) with a running sum, or O(n³) if I recompute sums from scratch —
  so I'd name that first, then say the trade I'm making: instead of comparing all
  subarrays, I only need to know the best sum *ending at* each position, which collapses
  the whole thing to one O(n) pass."
- **Invariant framing (good for justifying Kadane's line-by-line):** "The invariant is
  `current_sum` is always the best sum of a subarray that ends exactly at index i. At each
  step I ask: does extending the previous subarray still help, or has it turned into dead
  weight I should drop? A negative `current_sum` can only drag down anything I add to it,
  so resetting isn't a heuristic — it's provably never worse."
- **Generalization framing (good for signaling you know this beyond memorization):** "This
  is the canonical instance of Kadane's algorithm, which is itself a greedy technique —
  I'd name that family out loud, since the same 'track a running local optimum, reset when
  it can't help the future' shape reappears in max-product-subarray and best-time-to-buy-
  sell-stock variants."

### Vocabulary Builder

- **invariant** (n.) — a condition an algorithm maintains at every step; here, "`current_sum`
  is the best sum ending at the current index." *"Stating the invariant out loud is how I
  prove the reset rule is correct, not just plausible."*
- **local optimum** (n. phrase) — the best choice available given only current
  information, as opposed to a global optimum that requires foresight; Kadane's works
  because the local optimum at each step never needs revisiting.
- **"the naive approach breaks down when…"** — a reusable phrase for pivoting from brute
  force to the real solution; here it breaks down because recomputing every subarray's sum
  from scratch is wasteful when the running sum already carries that information.
- **degenerate case** (n. phrase) — a trivial but valid input, like a single-element or
  all-negative array; worth naming explicitly since Kadane's must still return the least
  negative element, not zero.
