# 2. House Robber

**Difficulty:** Medium
**Topic:** 1-D Dynamic Programming
**Pattern:** Include/exclude running maximum

## Problem
Given an array `nums` of non-negative integers representing money in houses arranged in a
row, you cannot rob two adjacent houses (triggers an alarm). Return the maximum amount you
can rob.

## Examples
```
Input: nums = [1,2,3,1] -> 4  (rob house 0 and 2: 1+3)
Input: nums = [2,7,9,3,1] -> 12  (rob houses 0,2,4: 2+9+1)
```

## Approach
At each house, decide: skip it (carry forward the best total without it) or rob it (best
total two houses back, plus this house's value). Track two running values: `rob_prev` (best
total including the previous house) and `skip_prev` (best total excluding it). At each
step, the new "best including or up to this house" is
`max(skip_prev + nums[i], rob_prev)`. Only two variables needed, no array.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Include/exclude running maximum**, which itself
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
Runnable, with sample test cases at the bottom (`python3 dp_1d/02_house_robber/solution.py`):

```python
--8<-- "dp_1d/02_house_robber/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The brute force tries every
  subset of non-adjacent houses — exponential. I'd say that, then point out the optimal
  choice at each house only depends on the best totals from the two houses before it, which
  is the opening for DP."
- **Invariant framing (good for explaining the include/exclude recurrence precisely):** "At
  every house I maintain two invariants: `rob_prev` is the best total *if* I robbed the
  previous house, `skip_prev` is the best total if I didn't. The new best is always
  `max(skip_prev + nums[i], rob_prev)` — stating both invariants out loud is what keeps the
  transition from getting muddled."
- **Pattern-recognition framing (good for naming the reusable shape):** "This is the
  classic include/exclude recurrence — 'take it or leave it' — and I'd flag that it
  generalizes directly to House Robber II with one extra twist for the circular
  constraint."

### Vocabulary Builder

- **include/exclude** (adj. phrase) — a recurrence structured around 'take this element or
  skip it'; precise vocabulary for the shape of this DP.
- **optimal substructure** (n. phrase) — the property that the best overall answer is built
  from best answers to smaller subproblems; the formal reason this greedy-looking choice is
  actually safe to make locally.
- **"…the naive approach breaks down when adjacency constraints interact across choices"** —
  a reusable way to explain why a simple greedy scan (always take the bigger house) fails
  here.
- **rolling variable** (n. phrase) — a variable reused across iterations instead of stored
  in an array, enabling the O(1)-space version once the array version is verified correct.
