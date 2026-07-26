# 4. Meeting Rooms

**Difficulty:** Easy
**Topic:** Intervals
**Pattern:** Sort by start, check adjacent overlap

## Problem
Given an array of meeting time intervals, determine if a person could attend all of them
(i.e. none overlap).

## Examples
```
Input: intervals = [[0,30],[5,10],[15,20]] -> False
Input: intervals = [[7,10],[2,4]]           -> True
```

## Approach
Sort intervals by start time. Once sorted, any overlap must occur between two
*adjacent* intervals in the sorted order, so a single linear scan comparing each interval's
start to the previous interval's end suffices — if `intervals[i][0] < intervals[i-1][1]`,
there's a conflict.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Sort by start, check adjacent overlap**, which
itself belongs to the broader **Interval Scheduling** family of techniques. If the
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
Runnable, with sample test cases at the bottom (`python3 intervals/04_meeting_rooms/solution.py`):

```python
--8<-- "intervals/04_meeting_rooms/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "Checking every pair of
  meetings for overlap is O(n²). I'd name that, then note the fix is the same one that
  applies across this whole topic: sort by start time first, and overlap-checking reduces
  to comparing each interval only against its immediate predecessor."
- **Reduction framing (good for placing this problem relative to its harder sibling):**
  "I'd frame this explicitly as the yes/no version of Meeting Rooms II — instead of
  counting how many rooms are needed simultaneously, I only need to know if any overlap
  exists at all, which means I don't need a heap, just a single sorted pass checking
  adjacent pairs."
- **Generalization framing (good for signaling the broader pattern):** "This is the
  simplest member of the interval-scheduling family — sort by start, check adjacent
  overlap — and I'd mention it as the base case worth solving first before tackling Meeting
  Rooms II, since getting the sort-and-scan structure right here transfers directly."

### Vocabulary Builder

- **reduces to** (v. phrase) — describing one problem as a simplified special case of
  another; here, Meeting Rooms reduces to "does the max concurrent count in Meeting Rooms
  II ever exceed 1."
- **adjacent pair** (n. phrase) — two consecutive elements in sorted order; the only pairs
  that need checking once intervals are sorted by start time, since sorting makes any
  overlap detectable locally.
- **"…is the yes/no version of…"** — a reusable phrase for relating an easy problem to a
  harder sibling that generalizes it, useful for showing you see the family tree rather
  than treating each problem as isolated.
- **base case** (n. phrase) — the simplest instance of a pattern, worth solving cleanly
  first since its structure (sort, then linear scan) usually transfers to harder variants
  with more state to track.
