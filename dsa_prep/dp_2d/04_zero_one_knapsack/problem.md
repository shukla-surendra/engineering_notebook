# 4. 0/1 Knapsack

**Difficulty:** Medium
**Topic:** 2-D Dynamic Programming
**Pattern:** Bounded knapsack (each item used at most once)

## Problem
Given `n` items each with a `weight` and a `value`, and a knapsack capacity `W`, choose a
subset of items (each usable at most once) maximizing total value without exceeding total
weight `W`. Return the max achievable value.

## Examples
```
Input: weights = [1,3,4,5], values = [1,4,5,7], capacity = 7 -> 9  (items with weight 3+4=7, value 4+5=9)
```

## Approach
`dp[i][w]` = max value achievable using the first `i` items with capacity `w`. For each
item, either skip it (`dp[i-1][w]`) or, if it fits (`weight[i-1] <= w`), take it
(`value[i-1] + dp[i-1][w - weight[i-1]]`) — take the better of the two. Iterating items in
the outer loop and processing weight capacity 0..W in the inner loop, using only the
*previous* item's row, is what enforces "each item used at most once" (contrast with Coin
Change's unbounded reuse, where the same row is reused within an item).

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Bounded knapsack (each item used at most
once)**, which itself belongs to the broader **2-D Dynamic Programming** family of
techniques. If the specific trick above feels like it came out of nowhere, that's the
signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to
recognize this family of problems in general (not just this one), the reusable template
you can write from memory, the usual variations, and the mistakes people make applying
it. Coming back to re-read this problem's approach afterward should make the specific
choices here feel inevitable rather than clever.

## Complexity
- Time: O(n·W)
- Space: O(n·W) (reducible to O(W) with a rolling array, iterating capacity in reverse)

## Solution
Runnable, with sample test cases at the bottom (`python3 dp_2d/04_zero_one_knapsack/solution.py`):

```python
--8<-- "dp_2d/04_zero_one_knapsack/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The brute force is trying
  every subset of items and checking which fit under the weight cap — 2^n subsets. I'd say
  that out loud, then note greedy-by-value-density doesn't provably work here because items
  are indivisible, which is exactly the signal that pushes this toward DP instead of a
  greedy heuristic."
- **Invariant framing (good for explaining why the loop order isn't arbitrary):** "`dp[i][w]`
  is 'best value achievable using only the first `i` items at capacity `w`.' The invariant
  I have to protect is that row `i` only reads from row `i-1`, never from itself — that's
  what enforces 'each item used at most once.' If I let a row read from itself I'd silently
  allow reusing an item, which is a different problem."
- **Generalization framing (good for showing you know the knapsack family, not just this
  instance):** "This is bounded knapsack. I'd contrast it out loud with unbounded knapsack
  (like Coin Change), where the recurrence deliberately reads from the *current* row to
  allow reuse — same table shape, one line of loop logic decides bounded vs. unbounded."

### Vocabulary Builder

- **subset-sum** (n. phrase) — the family of problems asking whether/how some subset of
  items meets a target; 0/1 Knapsack is the weighted, value-maximizing generalization of it.
- **bounded / unbounded** (adj.) — whether an item can be used at most once (bounded, this
  problem) or unlimited times (unbounded, e.g. Coin Change) — the single loop-order detail
  that distinguishes the two DP variants.
- **"…trades memory for speed"** — reusable phrase for justifying the O(n·W) table against
  the exponential brute force, and again when discussing the O(W) rolling-array reduction.
- **greedy fails here** (phrase) — worth stating explicitly: unlike fractional knapsack,
  where greedy-by-ratio is optimal, indivisible items break the greedy exchange argument,
  which is precisely why DP is required.
