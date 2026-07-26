# 5. Find Median from Data Stream

**Difficulty:** Hard
**Topic:** Heap / Priority Queue
**Pattern:** Two heaps (max-heap for the lower half, min-heap for the upper half)

## Problem
Design a data structure that supports `add_num(num)` and `find_median()`, efficiently
tracking the running median of a stream of numbers.

## Examples
```
add_num(1); add_num(2); find_median() -> 1.5
add_num(3); find_median() -> 2
```

## Approach
Split the stream into two halves via two heaps: a max-heap `small` holding the smaller
half, and a min-heap `large` holding the larger half, kept balanced in size (differing by
at most 1). On `add_num`, insert into `small` then move its top into `large` to keep
ordering correct, then rebalance sizes by moving the top of the larger heap back if it's
grown too big. The median is either the top of the larger-sized heap (odd total) or the
average of both tops (even total).

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Two heaps (max-heap for the lower half, min-
heap for the upper half)**, which itself belongs to the broader **Heap / Priority
Queue** family of techniques. If the specific trick above feels like it came out of
nowhere, that's the signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it
covers how to recognize this family of problems in general (not just this one), the
reusable template you can write from memory, the usual variations, and the mistakes
people make applying it. Coming back to re-read this problem's approach afterward should
make the specific choices here feel inevitable rather than clever.

## Complexity
- Time: O(log n) per `add_num`, O(1) per `find_median`
- Space: O(n)

## Solution
Runnable, with sample test cases at the bottom (`python3 heap_priority_queue/05_find_median_from_data_stream/solution.py`):

```python
--8<-- "heap_priority_queue/05_find_median_from_data_stream/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The naive approach
  re-sorts (or does an insertion into a sorted list) on every `add_num` — O(n) or O(n log n)
  per call. Two heaps get that down to O(log n) per insert and O(1) per median query, by
  never fully sorting — just keeping the boundary between the two halves accurate."
- **Invariant framing (good for explaining why two heaps work at all):** "I maintain two
  invariants together: every element in `small` is `<=` every element in `large`, and their
  sizes differ by at most one. Every `add_num` has to restore both — insert-then-shuffle-top
  keeps the ordering invariant true, and the size check afterward keeps the balance
  invariant true. The median falls straight out of those two invariants without any search."
- **Generalization framing (good for tying to the broader pattern):** "This is the two-heap
  pattern for streaming statistics — I'd name it as such, since the same 'split into a
  max-heap lower half and min-heap upper half, keep them balanced' idea generalizes to
  tracking any running percentile, not just the median."

### Vocabulary Builder

- **rebalance** (v.) — to restore a size or ordering invariant after an operation disturbs
  it; here, moving one element between `small` and `large` after every insertion.
- **balance invariant** (n. phrase) — the specific property "sizes differ by at most one,"
  which is what makes `find_median` an O(1) lookup instead of a search.
- **streaming statistic** (n. phrase) — a summary value (median, running average, top-k)
  maintained incrementally as data arrives, without ever materializing the full dataset in
  sorted form.
- **"…falls straight out of the invariant"** — a reusable phrase for describing a result
  that requires no extra computation because the maintained data structure already
  guarantees it — useful for signaling that a solution's simplicity is by design, not luck.
