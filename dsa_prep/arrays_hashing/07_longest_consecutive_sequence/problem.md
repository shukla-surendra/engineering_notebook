# 7. Longest Consecutive Sequence

**Difficulty:** Medium
**Topic:** Arrays & Hashing
**Pattern:** Hash Set, sequence-start detection

## Problem
Given an unsorted array of integers `nums`, return the length of the longest run of
consecutive integers (e.g. `[100,4,200,1,3,2]` contains the run `1,2,3,4`). Must run in
O(n) time — sorting first would be O(n log n).

## Examples
```
Input: nums = [100,4,200,1,3,2] -> 4   (the sequence 1,2,3,4)
Input: nums = [0,3,7,2,5,8,4,6,0,1] -> 9
```

## Approach
Put all numbers in a hash set. For each number, only start counting a sequence if
`num - 1` is **not** in the set (i.e. it's the start of a run) — this guarantees each run
is only walked once in total, not once per element, giving true O(n) overall despite the
inner while loop.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Hash Set, sequence-start detection**, which
itself belongs to the broader **Hashing for O(1) Lookups** family of techniques. If the
specific trick above feels like it came out of nowhere, that's the signal to step back
and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of
problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(n)
- Space: O(n)

## Solution
Runnable, with sample test cases at the bottom (`python3 arrays_hashing/07_longest_consecutive_sequence/solution.py`):

```python
--8<-- "arrays_hashing/07_longest_consecutive_sequence/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Constraint-driven framing (rules out the obvious answer first):** "Sorting first gets
  you there in O(n log n) easily, but the problem explicitly asks for O(n), so I need
  something that doesn't order the data at all — that's my cue to reach for a hash set."
- **Invariant framing (the subtlety that makes it truly O(n)):** "I only start counting a
  run from a number whose predecessor isn't in the set — that's the invariant that
  guarantees each run gets walked exactly once in total, not once per element inside it.
  Without that check, this degrades to O(n²) on a fully consecutive input."
- **Amortized-cost framing (for defending the true complexity under questioning):** "The
  inner while loop looks like it could be O(n) per outer iteration, worst case, but because
  every element is only ever visited as part of exactly one run's expansion, the total work
  across all outer iterations is still O(n) — that's an amortized argument, not a per-call
  one."

### Vocabulary Builder

- **amortized** (adj.) — a cost averaged over a whole sequence of operations rather than
  bounded per individual call; the reason the nested-looking loop here is still O(n)
  overall. *"The while loop is amortized O(n) across the full array, even though it's
  not O(1) per outer step."*
- **run / consecutive run** (n.) — a maximal stretch of values with no gaps, e.g. 1,2,3,4;
  the object this problem is measuring the length of.
- **"the naive approach breaks down when…"** — useful here for explaining why starting an
  expansion from *every* element (not just run-starts) turns O(n) into O(n²) on adversarial
  input.
- **sequence-start detection** (n. phrase) — checking `num - 1 not in seen` before
  expanding; the specific technique that prevents redundant work.
