# 3. House Robber II

**Difficulty:** Medium
**Topic:** 1-D Dynamic Programming
**Pattern:** Circular array reduced to two linear subproblems

## Problem
Same as House Robber, but the houses are arranged in a **circle** — the first and last
house are adjacent. Return the maximum amount you can rob without robbing two adjacent
houses.

## Examples
```
Input: nums = [2,3,2] -> 3   (can't rob both house 0 and 2 — they're adjacent in the circle)
Input: nums = [1,2,3,1] -> 4
```

## Approach
The circular constraint only matters for the pair (first house, last house). So the
answer is the max of two independent linear House-Robber subproblems: one that excludes
the last house (allows robbing the first), and one that excludes the first house (allows
robbing the last). Run the standard linear House Robber DP on both ranges and take the
max. (Special-case a single house directly.)

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Circular array reduced to two linear
subproblems**, which itself belongs to the broader **1-D Dynamic Programming** family of
techniques. If the specific trick above feels like it came out of nowhere, that's the
signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to
recognize this family of problems in general (not just this one), the reusable template
you can write from memory, the usual variations, and the mistakes people make applying
it. Coming back to re-read this problem's approach afterward should make the specific
choices here feel inevitable rather than clever.

## Complexity
- Time: O(n)
- Space: O(1)

## Solution
Runnable, with sample test cases at the bottom (`python3 dp_1d/03_house_robber_ii/solution.py`):

```python
--8<-- "dp_1d/03_house_robber_ii/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "A naive extension of House
  Robber would try to special-case the wraparound inside a single pass, but that gets messy
  fast — tracking whether house 0 was robbed as extra state. I'd name that complexity before
  proposing the cleaner reduction."
- **Invariant framing (good for justifying the reduction to two subproblems):** "The
  invariant is: the only constraint the circle adds is 'house 0 and house n-1 can't both be
  robbed.' So running linear House Robber on `[0..n-2]` and again on `[1..n-1]` and taking
  the max covers every valid configuration without ever needing extra state for the wrap."
- **Pattern-recognition framing (good for naming the reusable technique):** "This is the
  'reduce a circular constraint to two linear subproblems' trick — I'd say that explicitly,
  since it's a technique that generalizes to other circular-array problems, not just this
  one."

### Vocabulary Builder

- **wraparound** (n.) — the constraint created by the first and last elements being
  adjacent in a circular structure; naming it precisely clarifies what's actually different
  from the linear version.
- **reduction** (n.) — solving a problem by transforming it into one or more instances of a
  problem you've already solved; here, two linear House Robber calls.
- **"…the crux of it is that the circle only constrains one pair, not the whole structure"**
  — a compact way to justify why two linear subproblems suffice instead of a genuinely new
  algorithm.
- **edge case** (n. phrase) — here, a single house, where the circular constraint is
  vacuous; worth special-casing explicitly rather than trusting the general reduction to
  handle it silently.
