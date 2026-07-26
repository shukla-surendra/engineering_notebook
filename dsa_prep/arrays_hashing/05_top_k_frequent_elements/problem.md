# 5. Top K Frequent Elements

**Difficulty:** Medium
**Topic:** Arrays & Hashing
**Pattern:** Bucket Sort by frequency (or heap)

## Problem
Given an integer array `nums` and an integer `k`, return the `k` most frequent elements.
Any order in the answer is fine.

## Examples
```
Input: nums = [1,1,1,2,2,3], k = 2 -> [1,2]
Input: nums = [1], k = 1           -> [1]
```

## Approach
A heap of size k gives O(n log k). The optimal O(n) approach: count frequencies with a hash
map, then bucket-sort — create `n+1` buckets indexed by frequency (frequency can't exceed
`n`), and drop each value into `buckets[freq]`. Walk buckets from high frequency to low,
collecting values until we have k.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Bucket Sort by frequency (or heap)**, which
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
Runnable, with sample test cases at the bottom (`python3 arrays_hashing/05_top_k_frequent_elements/solution.py`):

```python
--8<-- "arrays_hashing/05_top_k_frequent_elements/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Heap-first framing (a defensible, easy-to-derive starting point):** "The obvious answer
  is a min-heap of size k over frequency counts — O(n log k). I'd say that first, since
  it's easy to justify on the spot, then mention that this problem actually has an O(n)
  answer if I exploit the fact that frequency itself is a small, bounded number."
- **Invariant framing (for the bucket-sort optimization):** "Because frequency can never
  exceed n, I can index buckets directly by frequency instead of comparing them — that's
  the insight that turns a sort or heap into a direct array lookup, dropping the log
  factor entirely."
- **Trade-off framing (naming what you're giving up):** "Bucket sort trades a bit of extra
  space — n+1 buckets, even though most sit near the low end — for guaranteed O(n) time,
  which is worth it when the interviewer explicitly wants the optimal bound."

### Vocabulary Builder

- **bounded domain** (n. phrase) — when a value is known to fall within a small, fixed
  range (here, frequency can't exceed n); recognizing this is what unlocks bucket-sort-style
  O(n) tricks. *"Frequency is a bounded domain, so I can use it as an array index instead
  of sorting."*
- **min-heap** (n.) — a priority queue that keeps the smallest element accessible in O(1),
  with O(log k) insert/pop; useful for "top k" problems when k is much smaller than n.
- **"the naive approach breaks down when…"** — a reusable phrase for pivoting from a
  first-pass solution to why it's suboptimal, e.g. "…the interviewer wants strictly O(n)."
- **amortized** (adj.) — describing a cost averaged over a sequence of operations; worth
  invoking if asked why bucket-filling is still linear overall despite looking like nested
  work.
