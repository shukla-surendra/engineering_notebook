# 1. Binary Search

**Difficulty:** Easy
**Topic:** Binary Search
**Pattern:** Classic binary search on a sorted array

## Problem
Given a sorted array of distinct integers `nums` and a `target`, return the index of
`target`, or `-1` if it isn't present. Must run in O(log n).

## Examples
```
Input: nums = [-1,0,3,5,9,12], target = 9 -> 4
Input: nums = [-1,0,3,5,9,12], target = 2 -> -1
```

## Approach
Maintain `left`/`right` bounds over the sorted array. At each step compare `nums[mid]` to
`target`: equal means found; if `nums[mid] < target` the answer must be to the right, so
move `left = mid + 1`; otherwise move `right = mid - 1`. Halving the search space each
step gives O(log n).

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Classic binary search on a sorted array**,
which itself belongs to the broader **Binary Search** family of techniques. If the
specific trick above feels like it came out of nowhere, that's the signal to step back
and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of
problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(log n)
- Space: O(1)

## Solution
Runnable, with sample test cases at the bottom (`python3 binary_search/01_binary_search/solution.py`):

```python
--8<-- "binary_search/01_binary_search/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Baseline framing (even for the "easy" version, name the alternative):** "A linear scan
  is O(n) and trivially correct — I'd mention it only to contrast it with binary search,
  which exploits the fact that the array is sorted to eliminate half the remaining
  candidates on every comparison."
- **Invariant framing (the habit worth building on the simplest case first):** "The
  invariant is: if the target exists, it's always within `[left, right]`. Every comparison
  either finds it or shrinks that range while preserving the invariant — that's the exact
  mental model I carry into every harder binary-search variant."
- **Boundary-condition framing (shows precision on loop bounds):** "I use `left <= right`
  because I'm looking for an exact match and returning -1 on failure — that's a different
  loop condition than the boundary-converging version I'd use for something like 'find the
  rotation point,' and mixing the two up is the most common bug in this family."

### Vocabulary Builder

- **search space** (n. phrase) — the current range of candidates still under
  consideration; binary search halves it every iteration. *"The search space shrinks by
  half on every comparison, giving O(log n)."*
- **invariant** (n.) — a condition guaranteed true throughout the loop's execution; here,
  "if the target exists, it's within `[left, right]`."
- **off-by-one** (n. phrase) — an error from an incorrect boundary (`mid+1` vs `mid`,
  `<=` vs `<`); binary search is notorious for these, so naming the risk shows awareness.
- **"halving the problem at each step"** — a natural, spoken-language way to describe
  O(log n) behavior without leaning on notation alone.
