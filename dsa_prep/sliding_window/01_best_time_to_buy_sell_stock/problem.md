# 1. Best Time to Buy and Sell Stock

**Difficulty:** Easy
**Topic:** Sliding Window
**Pattern:** Single pass, track running minimum

## Problem
Given an array `prices` where `prices[i]` is the stock price on day `i`, and you may buy
on one day and sell on a later day, return the maximum profit. Return 0 if no profit is
possible.

## Examples
```
Input: prices = [7,1,5,3,6,4] -> 5   (buy at 1, sell at 6)
Input: prices = [7,6,4,3,1]   -> 0   (prices only fall)
```

## Approach
This is a sliding window with a variable left edge: keep a running `min_price` seen so
far (the left/buy edge), and at each day compute the profit if selling today
(`price - min_price`), tracking the best. Update `min_price` whenever a new lower price
is seen. One pass, no extra space.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Single pass, track running minimum**, which
itself belongs to the broader **Sliding Window** family of techniques. If the specific
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
Runnable, with sample test cases at the bottom (`python3 sliding_window/01_best_time_to_buy_sell_stock/solution.py`):

```python
--8<-- "sliding_window/01_best_time_to_buy_sell_stock/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The obvious approach checks
  every buy/sell pair — O(n²). I'd name that first, then say the O(n) version notices you
  never need to re-check old pairs: at any given sell day, the only buy day that matters
  is the cheapest one seen *so far*, so I just track a running minimum instead."
- **Invariant framing (good for explaining the single pass precisely):** "The invariant
  is: `min_price` always equals the lowest price seen up to and including the current
  day. Because I update the best-profit-so-far *before* considering whether to lower
  `min_price`, I never accidentally sell before I buy — order of operations here is load-
  bearing."
- **Generalization framing (good for signaling pattern recognition):** "This is a
  degenerate sliding window — the left edge only ever moves to a strictly better position
  and never needs to shrink for validity, unlike the substring problems in this same
  folder. I'd mention that to show I see it as a family member, not a one-off trick."

### Vocabulary Builder

- **running minimum/maximum** (n. phrase) — a value updated incrementally to always
  reflect the best-so-far seen during a single pass, without rescanning prior elements.
- **monotonic** (adj.) — moving consistently in one direction; here, profit-so-far only
  ever increases, never needs revisiting once recorded.
- **"the naive approach breaks down when…"** — a reusable phrase for justifying an O(n)
  pivot: *"the O(n²) approach breaks down at scale because it repeats comparisons an O(n)
  scan makes unnecessary."*
- **degenerate case** (n. phrase) — here, both a trivial input (empty or single-element
  price array) and, more interestingly, a "degenerate" version of a general pattern —
  worth naming both senses if asked.
