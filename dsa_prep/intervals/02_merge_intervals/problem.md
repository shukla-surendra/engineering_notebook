# 2. Merge Intervals

**Difficulty:** Medium
**Topic:** Intervals
**Pattern:** Sort by start, then merge in a single pass

## Problem
Given an array of `intervals`, merge all overlapping intervals and return the resulting
non-overlapping set (sorted by start).

## Examples
```
Input: intervals = [[1,3],[2,6],[8,10],[15,18]] -> [[1,6],[8,10],[15,18]]
Input: intervals = [[1,4],[4,5]] -> [[1,5]]
```

## Approach
Sort intervals by start time. Walk through them keeping a `merged` list; if the current
interval's start is `<=` the last merged interval's end, they overlap — extend the last
merged interval's end to `max(last.end, current.end)`. Otherwise, append the current
interval as a new entry. Sorting first is what makes a single linear pass sufficient.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Sort by start, then merge in a single pass**,
which itself belongs to the broader **Interval Scheduling** family of techniques. If the
specific trick above feels like it came out of nowhere, that's the signal to step back
and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of
problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(n log n) (dominated by the sort)
- Space: O(n) for the output

## Solution
Runnable, with sample test cases at the bottom (`python3 intervals/02_merge_intervals/solution.py`):

```python
--8<-- "intervals/02_merge_intervals/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "Without sorting, checking
  whether any two intervals overlap means comparing every pair — O(n²). The crux of it is
  that sorting by start time first turns 'any pair could overlap' into 'only adjacent
  intervals in sorted order can possibly overlap,' which is what lets a single linear pass
  replace the pairwise comparison."
- **Invariant framing (good for explaining the merge step precisely):** "The invariant is:
  by the time I look at interval i, `merged[-1]` already reflects the correctly merged
  result of everything before it. So I only ever need to compare the current interval
  against the *last* merged one, not against every previous interval — extend it if they
  overlap, otherwise append a new entry."
- **Generalization framing (good for signaling the pattern family):** "This is the
  canonical 'sort by start, then merge in a single pass' template for interval problems —
  I'd name it as the base case that Meeting Rooms also uses, just answering a yes/no
  question instead of building the merged list."

### Vocabulary Builder

- **adjacency** (n.) — the property of being next to each other in sorted order; the key
  realization here is that overlap-checking reduces to checking adjacent pairs once
  sorted, not all pairs.
- **extend** (v.) — to widen an existing interval's bound (here, its end) rather than
  creating a new one; the natural verb for what happens when the current interval overlaps
  the last merged one.
- **"the crux of it is…"** — a reusable phrase for naming the single realization (sorting
  removes the need for pairwise comparison) that collapses an O(n²) approach to O(n log n).
- **base case** (n. phrase) — here used loosely to mean "the simplest/most fundamental
  member of a family of related problems"; Merge Intervals is a natural one to reach for
  when explaining the sort-by-start template before layering on variations.
