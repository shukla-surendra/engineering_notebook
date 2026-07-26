# 1. Contains Duplicate

**Difficulty:** Easy
**Topic:** Arrays & Hashing
**Pattern:** Hash Set

## Problem
Given an integer array `nums`, return `True` if any value appears **at least twice**, and
`False` if every element is distinct.

## Examples
```
Input: nums = [1,2,3,1]        -> True
Input: nums = [1,2,3,4]        -> False
Input: nums = [1,1,1,3,3,4,3,2,4,2] -> True
```

## Approach
Walk the array once, adding each value to a hash set. If a value is already in the set when
we see it, we found a duplicate. A sort-based approach works too (O(n log n), O(1) extra
space) but the hash-set approach is the standard O(n) answer.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Hash Set**, which itself belongs to the broader
**Hashing for O(1) Lookups** family of techniques. If the specific trick above feels
like it came out of nowhere, that's the signal to step back and read
[`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of problems in
general (not just this one), the reusable template you can write from memory, the usual
variations, and the mistakes people make applying it. Coming back to re-read this
problem's approach afterward should make the specific choices here feel inevitable
rather than clever.

## Complexity
- Time: O(n)
- Space: O(n)

## Solution
Runnable, with sample test cases at the bottom (`python3 arrays_hashing/01_contains_duplicate/solution.py`):

```python
--8<-- "arrays_hashing/01_contains_duplicate/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (a safe default opener):** "The naive approach is a nested
  loop comparing every pair, O(n²). I'll immediately flag that as the fallback and reach
  for a hash set instead — one pass, O(n) time, O(n) space, since I only need membership,
  not counts."
- **Invariant framing (for defending correctness under pressure):** "At any point mid-loop,
  `seen` holds exactly the distinct elements visited so far. I test membership before
  adding the current element, so the invariant never gets contaminated by an element
  matching itself."
- **Pattern-recognition framing (for signaling breadth, not just this answer):** "This is
  the simplest member of the hashing-for-lookups family — I'd call out that the same
  seen-before check, just parameterized differently, is what powers Two Sum and Longest
  Consecutive Sequence."

### Vocabulary Builder

- **membership check** (n. phrase) — testing whether a value exists in a collection;
  the operation a hash set makes O(1) instead of O(n). *"A membership check against a set
  is constant time on average."*
- **degenerate case** (n. phrase) — a technically valid but trivial input, like an
  empty array or single-element array; worth naming to show you checked boundaries.
- **"the crux of it is…"** — a clean way to pivot from restating the problem into stating
  your actual insight, without rambling.
- **sort-based alternative** (n. phrase) — mentioning the O(n log n), O(1)-extra-space
  sort-and-scan approach shows you know the space/time trade-off exists, even when you
  don't choose it.
