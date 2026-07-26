# 2. Last Stone Weight

**Difficulty:** Easy
**Topic:** Heap / Priority Queue
**Pattern:** Max-heap simulation

## Problem
You have stones with weights `stones`. Repeatedly smash the two heaviest stones together:
if they're equal weight, both are destroyed; otherwise the lighter is destroyed and the
heavier becomes `heavy - light`. Return the weight of the last remaining stone, or 0 if
none remain.

## Examples
```
Input: stones = [2,7,4,1,8,1] -> 1
```

## Approach
Python's `heapq` is a min-heap, so negate values to simulate a max-heap. Repeatedly pop
the two largest (most negative), and if they differ, push the difference back (re-negated).
Continue until at most one stone remains.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Max-heap simulation**, which itself belongs to
the broader **Heap / Priority Queue** family of techniques. If the specific trick above
feels like it came out of nowhere, that's the signal to step back and read
[`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of problems in
general (not just this one), the reusable template you can write from memory, the usual
variations, and the mistakes people make applying it. Coming back to re-read this
problem's approach afterward should make the specific choices here feel inevitable
rather than clever.

## Complexity
- Time: O(n log n)
- Space: O(n)

## Solution
Runnable, with sample test cases at the bottom (`python3 heap_priority_queue/02_last_stone_weight/solution.py`):

```python
--8<-- "heap_priority_queue/02_last_stone_weight/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The brute force re-sorts the
  whole array after every smash to find the two heaviest — O(n log n) per smash, O(n²
  log n) overall. A heap avoids re-sorting: it keeps the two heaviest accessible in O(log n)
  each, without re-ordering everything else."
- **Simulation framing (good for explaining the negation trick cleanly):** "I'm directly
  simulating the process the problem describes — pop the two largest, push back the
  difference, repeat — and the only wrinkle is that Python's `heapq` is min-heap-only, so I
  negate on the way in and out. I'd say that out loud early, since sign errors here are the
  most common way this goes wrong live."
- **Generalization framing (good for connecting to a broader family):** "This is 'max-heap
  simulation' — repeatedly combine the two extreme elements and reinsert the result — which
  is the same shape as Huffman coding's merge step, just with subtraction instead of
  addition. Naming that connection shows I recognize the structural similarity, not just
  the specific rule."

### Vocabulary Builder

- **negation trick** (n. phrase) — storing `-x` instead of `x` to simulate a max-heap using
  a min-heap-only implementation; worth naming explicitly since it's a recurring idiom, not
  a one-off hack.
- **simulate** (v.) — to directly model a process step-by-step as described, rather than
  finding a closed-form shortcut; the honest description of this approach, since there's no
  cleverer trick here beyond picking the right data structure.
- **sign error** (n. phrase) — a bug from mishandling negation; worth flagging as the
  specific failure mode to watch for when narrating heap-via-negation code live.
- **"…combine the two extremes and reinsert"** — a reusable phrase for describing the
  general shape of problems like this and Huffman coding, useful for signaling the pattern
  without walking through every step.
