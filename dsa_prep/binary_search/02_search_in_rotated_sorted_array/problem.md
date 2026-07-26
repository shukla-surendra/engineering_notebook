# 2. Search in Rotated Sorted Array

**Difficulty:** Medium
**Topic:** Binary Search
**Pattern:** Modified binary search using "which half is sorted"

## Problem
An ascending array `nums` (distinct values) has been rotated at an unknown pivot. Given
`target`, return its index, or `-1` if absent. Must run in O(log n).

## Examples
```
Input: nums = [4,5,6,7,0,1,2], target = 0 -> 4
Input: nums = [4,5,6,7,0,1,2], target = 3 -> -1
```

## Approach
Even rotated, at least one half of `[left, mid]` or `[mid, right]` is always properly
sorted. Check which half is sorted by comparing `nums[left]` to `nums[mid]`. Then check
whether `target` falls within that sorted half's range — if so, recurse/iterate into it;
otherwise the target must be in the other half. This keeps halving the search space.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Modified binary search using "which half is
sorted"**, which itself belongs to the broader **Binary Search** family of techniques.
If the specific trick above feels like it came out of nowhere, that's the signal to step
back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family
of problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(log n)
- Space: O(1)

## Solution
Runnable, with sample test cases at the bottom (`python3 binary_search/02_search_in_rotated_sorted_array/solution.py`):

```python
--8<-- "binary_search/02_search_in_rotated_sorted_array/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Structural-insight framing (the one sentence that unlocks the whole problem):** "Even
  after rotation, at least one of the two halves around `mid` is still properly sorted —
  that single observation is what lets me keep applying binary search instead of falling
  back to a linear scan."
- **Invariant framing (how I decide which half to recurse into):** "At each step, I first
  figure out which half is sorted by comparing `nums[left]` to `nums[mid]`, then check
  whether `target` could actually live inside that sorted half's range. If it can't, it
  must be in the other half — that two-step check is the whole algorithm."
- **Contrast framing (distinguishes this from vanilla binary search by name):** "This is
  binary search with an extra classification step bolted on — 'which half is sorted' —
  rather than a fundamentally different algorithm. I'd say that explicitly to show I'm
  extending the base template, not inventing something new."

### Vocabulary Builder

- **pivot** (n.) — the index where the rotation occurs, the boundary between the two
  originally-contiguous sorted segments. *"The pivot splits the array into two sorted
  halves, even though the array as a whole isn't sorted."*
- **properly sorted** (adj. phrase) — a subrange with no rotation break in it, i.e. it
  satisfies the normal sorted-order invariant end to end.
- **"degrades gracefully to…"** — useful for noting that when there's no rotation at all
  (pivot = 0), this algorithm behaves identically to plain binary search.
- **classification step** (n. phrase) — an extra piece of logic (here, "which half is
  sorted") layered on top of a base algorithm to handle a modified problem shape.
