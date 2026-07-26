# 1. Kth Largest Element in a Stream

**Difficulty:** Easy
**Topic:** Heap / Priority Queue
**Pattern:** Fixed-size min-heap

## Problem
Design a class that, given an integer `k` and an initial stream of numbers, supports `add(val)`
which adds `val` to the stream and returns the k-th largest element in the stream so far.

## Examples
```
KthLargest(3, [4,5,8,2]); add(3) -> 4; add(5) -> 5; add(10) -> 5; add(9) -> 8; add(4) -> 8
```

## Approach
Maintain a min-heap capped at size `k`. It always holds the `k` largest elements seen so
far, with the smallest of those `k` sitting at the heap's top — that top is exactly the
answer. On `add`, push the new value, and if the heap grows past size `k`, pop the
smallest. This is far cheaper than re-sorting the whole stream on every call.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Fixed-size min-heap**, which itself belongs to
the broader **Heap / Priority Queue** family of techniques. If the specific trick above
feels like it came out of nowhere, that's the signal to step back and read
[`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of problems in
general (not just this one), the reusable template you can write from memory, the usual
variations, and the mistakes people make applying it. Coming back to re-read this
problem's approach afterward should make the specific choices here feel inevitable
rather than clever.

## Complexity
- Time: O(log k) per `add`
- Space: O(k)

## Solution
Runnable, with sample test cases at the bottom (`python3 heap_priority_queue/01_kth_largest_element_in_a_stream/solution.py`):

```python
--8<-- "heap_priority_queue/01_kth_largest_element_in_a_stream/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The naive approach re-sorts
  the entire stream on every `add` call — O(n log n) per call. I'd name that, then say the
  fix: I don't need the whole stream sorted, only the k largest values, and a heap capped
  at size k gets each `add` down to O(log k)."
- **Invariant framing (good for explaining why the top of the heap is the answer):** "The
  invariant I maintain is: the min-heap always holds exactly the k largest elements seen so
  far, and its root is the smallest of those k — which by definition is the k-th largest
  overall. Popping whenever the heap exceeds size k is what keeps that invariant true after
  every insertion."
- **Generalization framing (good for signaling you know the family):** "This is the
  bounded min-heap pattern for streaming top-k — I'd name it as such, since the same shape
  reappears in K Closest Points and any 'maintain the top k of an unbounded stream'
  problem, just with a different comparison key."

### Vocabulary Builder

- **bounded heap** (n. phrase) — a heap capped at a fixed size k, used to track "the top k"
  without paying for a full sort; the size cap is what turns O(log n) into O(log k).
- **streaming** (adj.) — describing data that arrives incrementally rather than all at
  once; it's the detail that rules out "just sort everything" as a viable repeated
  operation.
- **"…trades a full sort for a bounded structure"** — a reusable phrase for justifying
  heap-based top-k solutions over repeatedly sorting.
- **invariant** (n.) — a property maintained across calls; here, "the heap always contains
  exactly the k largest elements so far" is what makes `heap[0]` a valid answer at any
  point in the stream, not just at the end.
