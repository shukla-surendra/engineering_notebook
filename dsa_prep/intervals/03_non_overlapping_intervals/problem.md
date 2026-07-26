# 3. Non-overlapping Intervals

**Difficulty:** Medium
**Topic:** Intervals
**Pattern:** Sort by end time, greedy interval scheduling

## Problem
Given an array of `intervals`, return the minimum number of intervals to remove so the
rest are non-overlapping.

## Examples
```
Input: intervals = [[1,2],[2,3],[3,4],[1,3]] -> 1  (remove [1,3])
Input: intervals = [[1,2],[1,2],[1,2]]        -> 2
```

## Approach
This is the classic "activity selection" greedy problem: sort intervals by **end** time
(not start). Walk through them keeping track of the end time of the last kept interval; if
the current interval's start is before that end time, it overlaps — remove it (increment a
counter) and keep the previously kept interval's end (it ends earlier, so it's strictly
better to keep for future compatibility). Otherwise keep the current interval and update
the tracked end time. Sorting by end time greedily maximizes the number of non-overlapping
intervals kept, which minimizes removals.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Sort by end time, greedy interval scheduling**,
which itself belongs to the broader **Interval Scheduling** family of techniques. If the
specific trick above feels like it came out of nowhere, that's the signal to step back
and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of
problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(n log n)
- Space: O(1) extra (excluding sort)

## Solution
Runnable, with sample test cases at the bottom (`python3 intervals/03_non_overlapping_intervals/solution.py`):

```python
--8<-- "intervals/03_non_overlapping_intervals/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The brute force considers
  every subset of intervals to find the largest non-overlapping one — O(2ⁿ). I'd name that,
  then pivot immediately to the classic activity-selection greedy: sort by end time, and
  greedily keep whatever's compatible with what's already kept, which is O(n log n)."
- **Exchange-argument framing (good for justifying the sort key, not just naming it):**
  "The choice to sort by *end* time, not start, is the actual insight, and I'd justify it
  with an exchange argument: whichever interval finishes earliest among compatible
  candidates leaves the most room for everything after it, so swapping any other choice for
  the earliest-finishing one can never make the final count worse."
- **Generalization framing (good for connecting to the greedy pattern library):** "This is
  activity selection, a named instance of greedy interval scheduling — I'd reference
  `../PATTERN.md` and note this is the canonical example the exchange-argument proof
  technique is usually taught with."

### Vocabulary Builder

- **activity selection** (n. phrase) — the classical name for "select the maximum number
  of mutually compatible intervals from a set"; worth naming explicitly since it signals
  you recognize a textbook problem rather than improvising.
- **exchange argument** (n. phrase) — the proof technique showing any optimal solution can
  be transformed into your greedy choice without losing value; here, it justifies sorting
  by end time specifically.
- **sort key** (n. phrase) — the field used to order elements before a greedy pass; calling
  out that the sort key itself *is* the greedy insight (end time, not start time) is often
  the single most interview-relevant thing to say about this problem.
- **"…leaves the most room for everything after it"** — a reusable phrase for explaining
  greedy interval-selection intuitively before formalizing it as an exchange argument.
