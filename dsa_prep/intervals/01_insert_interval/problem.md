# 1. Insert Interval

**Difficulty:** Medium
**Topic:** Intervals
**Pattern:** Single pass, split into before/overlapping/after

## Problem
Given a list of non-overlapping intervals `intervals` sorted by start time, and a new
`newInterval`, insert it and merge as necessary so the result is still sorted and
non-overlapping.

## Examples
```
Input: intervals = [[1,3],[6,9]], newInterval = [2,5] -> [[1,5],[6,9]]
Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
Output: [[1,2],[3,10],[12,16]]
```

## Approach
Single pass through the sorted intervals, split into three phases: (1) intervals ending
entirely before `newInterval` starts — copy as-is; (2) intervals overlapping
`newInterval` (start <= newInterval's end and end >= newInterval's start) — merge them
into `newInterval` by expanding its bounds (`min` of starts, `max` of ends); (3) intervals
starting entirely after `newInterval` ends — copy as-is. Insert the (possibly expanded)
`newInterval` between phases 1 and 3.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Single pass, split into
before/overlapping/after**, which itself belongs to the broader **Interval Scheduling**
family of techniques. If the specific trick above feels like it came out of nowhere,
that's the signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it covers how
to recognize this family of problems in general (not just this one), the reusable
template you can write from memory, the usual variations, and the mistakes people make
applying it. Coming back to re-read this problem's approach afterward should make the
specific choices here feel inevitable rather than clever.

## Complexity
- Time: O(n)
- Space: O(n) for the output

## Solution
Runnable, with sample test cases at the bottom (`python3 intervals/01_insert_interval/solution.py`):

```python
--8<-- "intervals/01_insert_interval/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The brute force appends
  `newInterval` to the list and re-runs a general merge-intervals sort — O(n log n). The
  crux of the better approach is that the input is *already* sorted, so re-sorting throws
  away information I already have; a single linear pass exploiting that existing order gets
  this to O(n)."
- **Phase framing (good for narrating the three-way split cleanly):** "I think of the scan
  as three phases in order: intervals entirely before the new one, intervals overlapping
  it, and intervals entirely after. I'd say that structure out loud before coding, since it
  turns a fiddly merge into three simple, sequential loops instead of one loop with
  tangled conditionals."
- **Generalization framing (good for connecting to the family):** "This is a variant of
  interval scheduling that exploits an already-sorted, non-overlapping precondition — I'd
  name that precondition explicitly, since it's what distinguishes this from Merge
  Intervals, which can't assume the input arrives pre-sorted."

### Vocabulary Builder

- **precondition** (n.) — an assumption the algorithm relies on being true before it runs
  (here, that `intervals` is already sorted and non-overlapping); naming it explicitly
  clarifies why this problem doesn't need its own sort step.
- **"…exploiting existing structure"** — a reusable phrase for justifying why an approach
  that looks specialized is actually a legitimate optimization: it's using information
  (sortedness) the problem statement already hands you, not a hack.
- **phase** (n.) — a distinct stage of a single pass with its own simple rule; describing
  an algorithm as having phases is a clean way to narrate multi-part linear scans without
  the logic sounding tangled.
- **in-place** (adj.) — modifying a structure without allocating a new one; worth
  contrasting with this solution's approach of building a fresh result list, and explaining
  why in-place insertion into an array is awkward here (shifting costs).
