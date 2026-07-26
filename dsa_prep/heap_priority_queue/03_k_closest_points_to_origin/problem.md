# 3. K Closest Points to Origin

**Difficulty:** Medium
**Topic:** Heap / Priority Queue
**Pattern:** Max-heap of size k (or quickselect)

## Problem
Given an array of `points` where `points[i] = [xi, yi]`, return the `k` points closest to
the origin `(0, 0)` (Euclidean distance). Any order is acceptable.

## Examples
```
Input: points = [[1,3],[-2,2]], k = 1 -> [[-2,2]]
Input: points = [[3,3],[5,-1],[-2,4]], k = 2 -> [[3,3],[-2,4]]
```

## Approach
Maintain a max-heap (negate distances) capped at size `k`, keyed by squared distance
(avoids an unnecessary sqrt). Push each point; if the heap exceeds size `k`, pop the
farthest. What remains are the k closest. (Quickselect gives O(n) average time if that's
preferred over the O(n log k) heap approach.)

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Max-heap of size k (or quickselect)**, which
itself belongs to the broader **Heap / Priority Queue** family of techniques. If the
specific trick above feels like it came out of nowhere, that's the signal to step back
and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of
problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(n log k)
- Space: O(k)

## Solution
Runnable, with sample test cases at the bottom (`python3 heap_priority_queue/03_k_closest_points_to_origin/solution.py`):

```python
--8<-- "heap_priority_queue/03_k_closest_points_to_origin/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The simplest correct answer
  is: compute every distance, sort all n points by it, take the first k — O(n log n). I'd
  name that first, then note it does more work than necessary since it fully orders points
  we don't care about the relative rank of beyond position k."
- **Trade-off framing (good for comparing the two real candidate solutions):** "There are
  two solutions worth naming and comparing out loud: a bounded max-heap gives O(n log k)
  with O(k) space, guaranteed; quickselect gives O(n) average time but O(n²) worst case and
  is more fiddly to implement correctly live. I'd pick the heap by default and mention
  quickselect as the follow-up if asked to optimize further."
- **Generalization framing (good for connecting to the pattern library):** "This is the
  same 'bounded heap of size k' shape as Kth Largest Element in a Stream, just keyed by
  squared distance instead of raw value — naming that connection up front signals I'm
  reusing a template, not improvising from scratch."

### Vocabulary Builder

- **squared distance** (n. phrase) — comparing `x² + y²` instead of `√(x² + y²)`; since
  square root is monotonic for non-negative values, it doesn't change the ordering, so
  skipping it avoids unnecessary floating-point work.
- **quickselect** (n.) — a partition-based selection algorithm (a relative of quicksort)
  that finds the k-th smallest/largest element in expected O(n) time without fully sorting.
- **worst-case vs. average-case** (n. phrase) — quickselect's O(n) is an average-case
  guarantee (it degrades to O(n²) on adversarial pivots), whereas the heap's O(n log k) is
  a worst-case guarantee — a distinction worth stating explicitly when comparing the two.
- **"…does more work than necessary"** — a reusable phrase for critiquing a correct-but-
  wasteful brute force before proposing the bounded alternative.
