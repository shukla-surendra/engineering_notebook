# Pattern: Heap / Priority Queue

## What problem does this solve?

Any time you repeatedly need "the current min/max among a changing set of elements" —
without needing the *entire* set sorted, just the extreme — a heap gives O(log n) insert
and O(log n) extract-min/max, versus O(n log n) to re-sort from scratch every time the set
changes, or O(n) to linear-scan for the extreme every time. This is the standard tool for
"top k," "kth largest," "merge k sorted things," and "running median" style problems.

## How to recognize it

Signals that a heap applies:
- "Top k" / "k closest" / "k-th largest/smallest," especially with a *stream* (elements
  keep arriving) — a bounded heap avoids re-sorting on every new element.
- "Repeatedly take the two largest/smallest and combine them" (Last Stone Weight,
  Huffman-coding-style problems) — this is literally "simulate with a heap."
- "Merge k sorted structures" — keep one representative per structure in a heap, always
  advancing whichever one gets consumed.
- "Running median" / "running percentile" of a stream — the two-heap trick.
- You catch yourself wanting to `sort()` the whole collection just to look at the
  top/bottom few elements, or wanting to re-sort after every single insertion — that's the
  tell that a heap should replace the sort.

## The general template

**Top-k / kth-largest with a bounded min-heap** (keeps the heap small — O(log k) per op
instead of O(log n)):
```python
import heapq
heap = []
for x in stream:
    heapq.heappush(heap, x)
    if len(heap) > k:
        heapq.heappop(heap)     # evict the smallest — heap always holds the k largest
# heap[0] is now the k-th largest seen so far
```

**Max-heap via negation** (Python's `heapq` is min-heap only):
```python
heap = [-x for x in values]
heapq.heapify(heap)
biggest = -heapq.heappop(heap)
```

**Merge k sorted lists/streams:**
```python
heap = [(seq[0], i, 0) for i, seq in enumerate(sequences) if seq]
heapq.heapify(heap)
while heap:
    val, seq_i, elem_i = heapq.heappop(heap)
    emit(val)
    if elem_i + 1 < len(sequences[seq_i]):
        heapq.heappush(heap, (sequences[seq_i][elem_i + 1], seq_i, elem_i + 1))
```
The tie-breaker index (`seq_i`) in the tuple avoids Python trying to compare the next
element type directly if values tie (important for lists of custom objects).

**Two heaps for a running median:**
```python
small = []  # max-heap (negated) — holds the lower half
large = []  # min-heap — holds the upper half
def add(num):
    heapq.heappush(small, -num)
    heapq.heappush(large, -heapq.heappop(small))     # ensures small's max <= large's min
    if len(large) > len(small):
        heapq.heappush(small, -heapq.heappop(large))
def median():
    return -small[0] if len(small) > len(large) else (-small[0] + large[0]) / 2
```

## Common pitfalls

- Using an unbounded heap for "top k" instead of capping its size at k — works, but wastes
  O(log n) instead of O(log k) per operation, and misses the deliberate size-bound idiom.
- Forgetting Python's `heapq` is min-heap-only and needing negation for max-heap behavior —
  easy to get sign errors when popping/reporting the negated values.
- For the two-heap median trick, forgetting to *rebalance sizes* after every insertion —
  the invariant "sizes differ by at most 1" is what makes `find_median` O(1).

## Complexity characteristics

O(log n) per insert/extract (O(log k) for a bounded top-k heap), O(1) for peeking the
top. Building a heap from n elements up front is O(n) (`heapify`), not O(n log n).

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Recognition framing (how I'd open a discussion of this pattern):** "The tell for a
  heap is catching myself wanting to sort the whole collection just to look at a few
  extreme elements, or wanting to re-sort after every single insertion. Any time the
  question is phrased as 'top k,' 'kth largest,' 'merge k sorted things,' or 'running
  median,' a heap is almost always the right structure before I even think about the
  details."
- **Mental-model framing (good for explaining *why* it's the right tool, not just naming
  it):** "A heap gives you a partial order — it only guarantees the extreme element is
  accessible in O(log n), not that everything is sorted. That's exactly the right amount of
  structure when you only ever need the min or max repeatedly, and paying for a full sort
  would be solving a harder problem than you were asked."
- **Generalization framing (good for showing you can pick the right heap shape):** "There
  are really three templates here, not one: a size-bounded heap for top-k, a heap of 'next
  candidates' for merging k sorted sequences, and two heaps split at a boundary for running
  medians. I'd name which template applies before writing code, since picking the wrong one
  is the most common way this goes sideways."

### Vocabulary Builder

- **partial order** (n. phrase) — an ordering that only guarantees relationships between
  some elements (like "the root is smaller than its children") rather than a full sorted
  sequence; it's exactly the guarantee a heap provides, and exactly what most "top-k" or
  "extreme element" problems actually need.
- **heapify** (v.) — building a heap from an existing collection in O(n) time, faster than
  n individual O(log n) insertions would suggest; worth knowing so you don't accidentally
  claim O(n log n) for heap construction when it isn't necessary.
- **extract-min / extract-max** (n. phrase) — the operation of removing and returning the
  heap's root; the core operation that makes heaps useful whenever a problem repeatedly
  asks for "the current smallest/largest."
- **"…is exactly the right amount of structure"** — a reusable phrase for justifying a data
  structure choice by noting it provides no more (and no less) guarantee than the problem
  actually needs — useful pushback against over-engineering with a fully sorted structure.
- **tie-breaker** (n.) — an auxiliary value added to a heap entry (like an index) purely to
  make comparisons well-defined when primary keys are equal; worth mentioning when merging
  k sorted sequences of complex objects.
