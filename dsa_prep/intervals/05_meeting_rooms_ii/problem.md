# 5. Meeting Rooms II

**Difficulty:** Medium
**Topic:** Intervals
**Pattern:** Min-heap of active end times (or separate sorted start/end sweep)

## Problem
Given an array of meeting time intervals, return the minimum number of conference rooms
required to hold all meetings.

## Examples
```
Input: intervals = [[0,30],[5,10],[15,20]] -> 2
Input: intervals = [[7,10],[2,4]]           -> 1
```

## Approach
Sort meetings by start time. Use a min-heap of end times for currently "in progress"
meetings. For each meeting in order, if the heap's smallest end time is `<=` this
meeting's start, that room has freed up — pop it (reuse the room). Push the current
meeting's end time either way. The heap's size at any point represents rooms in
simultaneous use; its **maximum size over the whole sweep** is the answer. (An alternative
"sweep line" approach separately sorts all start times and end times, and tracks a running
counter — both approaches are O(n log n).)

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Min-heap of active end times (or separate
sorted start/end sweep)**, which itself belongs to the broader **Interval Scheduling**
family of techniques. If the specific trick above feels like it came out of nowhere,
that's the signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it covers how
to recognize this family of problems in general (not just this one), the reusable
template you can write from memory, the usual variations, and the mistakes people make
applying it. Coming back to re-read this problem's approach afterward should make the
specific choices here feel inevitable rather than clever.

## Complexity
- Time: O(n log n)
- Space: O(n)

## Solution
Runnable, with sample test cases at the bottom (`python3 intervals/05_meeting_rooms_ii/solution.py`):

```python
--8<-- "intervals/05_meeting_rooms_ii/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The brute force checks, for
  each meeting, how many others it overlaps with — O(n²). The reframe that unlocks the
  better solution is realizing this isn't really about pairwise overlap at all — it's
  asking for the *peak* number of meetings active at the same instant, which is a
  concurrency question, not a comparison question."
- **Sweep-line framing (good for explaining the heap mechanically):** "I sort by start
  time and sweep through, using a min-heap of end times to represent 'meetings currently in
  progress.' Whenever the earliest-ending active meeting has already finished by the time
  the next one starts, I free that room by popping it. The heap's maximum size across the
  whole sweep — not its final size — is the answer."
- **Generalization framing (good for connecting to the broader family):** "This is a sweep-
  line / heap-of-active-intervals pattern — I'd name it as the 'concurrent usage' variant of
  interval scheduling, distinct from the sort-by-start (any overlap) and sort-by-end
  (maximum compatible subset) variants, since it's answering a fundamentally different
  question: how many, not whether."

### Vocabulary Builder

- **sweep line** (n. phrase) — an algorithmic technique that processes events (starts and
  ends) in time order while maintaining running state; the heap here is the running state
  of "currently active" meetings as the sweep progresses.
- **concurrency** (n.) — multiple things being active/in-progress at the same time; this
  problem is fundamentally a concurrency question dressed up as a scheduling one.
- **peak / maximum concurrent usage** (n. phrase) — the largest number of overlapping
  intervals at any single instant; the actual quantity being computed, worth naming
  explicitly since it clarifies why you track the heap's *maximum* size, not its size at
  the end of the sweep.
- **"…is a concurrency question, not a comparison question"** — a reusable phrase for
  reframing a problem away from pairwise thinking (which suggests O(n²)) toward a
  sweep/counting approach (which suggests O(n log n)).
