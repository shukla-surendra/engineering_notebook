# 7. Coin Change

**Difficulty:** Medium
**Topic:** 1-D Dynamic Programming
**Pattern:** Unbounded knapsack (bottom-up min-coins table)

## Problem
Given coin denominations `coins` and a `target amount`, return the fewest number of coins
needed to make that amount (unlimited supply of each coin). Return `-1` if it can't be
made.

## Examples
```
Input: coins = [1,2,5], amount = 11 -> 3   (5+5+1)
Input: coins = [2], amount = 3       -> -1
Input: coins = [1], amount = 0       -> 0
```

## Approach
Build a bottom-up DP array `dp[a]` = minimum coins to make amount `a`, with `dp[0] = 0` and
everything else initialized to infinity. For each amount from 1 to target, try every coin
`c <= a`: `dp[a] = min(dp[a], dp[a - c] + 1)`. This is the unbounded-knapsack pattern
(each coin can be reused, so iterate amounts in the outer loop, coins in the inner loop,
without an "already used" restriction).

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Unbounded knapsack (bottom-up min-coins
table)**, which itself belongs to the broader **1-D Dynamic Programming** family of
techniques. If the specific trick above feels like it came out of nowhere, that's the
signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to
recognize this family of problems in general (not just this one), the reusable template
you can write from memory, the usual variations, and the mistakes people make applying
it. Coming back to re-read this problem's approach afterward should make the specific
choices here feel inevitable rather than clever.

## Complexity
- Time: O(amount · len(coins))
- Space: O(amount)

## Solution
Runnable, with sample test cases at the bottom (`python3 dp_1d/07_coin_change/solution.py`):

```python
--8<-- "dp_1d/07_coin_change/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The brute force tries every
  combination of coins recursively — exponential, and it re-explores the same remaining
  amount over and over from different coin orderings. I'd name that redundancy before
  introducing the DP table, since it's the actual justification for memoizing on amount."
- **Invariant framing (good for explaining the bottom-up fill order):** "The invariant is
  that by the time I compute `dp[a]`, every `dp[a - c]` for coin `c <= a` is already final.
  That's only true because I iterate amounts from 0 upward — get that order backwards and
  the recurrence reads uninitialized values."
- **Pattern-recognition framing (good for naming the family and its variant):** "This is
  unbounded knapsack — each coin can be reused arbitrarily many times, which is why coins
  go in the inner loop without any 'already used' tracking. I'd contrast that out loud with
  0/1 knapsack, where each item can only be used once, to show I know which variant applies
  here."

### Vocabulary Builder

- **unbounded knapsack** (n. phrase) — the knapsack variant where items can be reused
  without limit; the term itself signals you recognize the family, not just this instance.
- **sentinel value** (n. phrase) — here, initializing `dp[a]` to infinity to represent
  "not yet reachable," letting `min()` work correctly before any real value is known.
- **"…trades exponential branching for a table filled once, amount by amount"** — a
  reusable way to justify the DP table over recursive brute force.
- **unreachable** (adj.) — describing an amount no combination of coins can sum to exactly;
  the reason the final check for `dp[amount] == infinity` (returning -1) matters.
