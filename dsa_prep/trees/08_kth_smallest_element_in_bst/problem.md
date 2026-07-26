# 8. Kth Smallest Element in a BST

**Difficulty:** Medium
**Topic:** Trees
**Pattern:** In-order traversal (yields sorted order for a BST)

## Problem
Given the root of a BST and an integer `k`, return the `k`-th smallest value in the tree
(1-indexed).

## Examples
```
Input: root = [3,1,4,null,2], k = 1 -> 1
Input: root = [5,3,6,2,4,null,null,1], k = 3 -> 3
```

## Approach
An in-order traversal (left, node, right) of a BST visits nodes in strictly ascending
order. Do an iterative in-order traversal with an explicit stack, decrementing a counter
each time a node is visited; stop and return as soon as the counter hits `k`. This avoids
building the full sorted list when `k` is small.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **In-order traversal (yields sorted order for a
BST)**, which itself belongs to the broader **Tree Traversal (DFS & BFS)** family of
techniques. If the specific trick above feels like it came out of nowhere, that's the
signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to
recognize this family of problems in general (not just this one), the reusable template
you can write from memory, the usual variations, and the mistakes people make applying
it. Coming back to re-read this problem's approach afterward should make the specific
choices here feel inevitable rather than clever.

## Complexity
- Time: O(h + k) where h = tree height
- Space: O(h)

## Solution
Runnable, with sample test cases at the bottom (`python3 trees/08_kth_smallest_element_in_bst/solution.py`):

```python
--8<-- "trees/08_kth_smallest_element_in_bst/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The brute force is: do a full
  in-order traversal, collect every value into a list, index into `list[k-1]`. That's
  O(n) time and space regardless of `k`. I'd name that first, then say the optimization is
  stopping the traversal early instead of avoiding it entirely."
- **Invariant framing (good for the iterative early-exit version):** "The invariant is
  that an iterative in-order walk with an explicit stack visits nodes in strictly ascending
  order, one at a time, so I can decrement a counter as I go and return the instant it hits
  zero — I never need to materialize the rest of the tree past the k-th node."
- **Generalization framing (good for the trades-memory-for-speed framing):** "This is the
  'let sortedness do the work' idea again — an in-order BST walk is a sorted stream for
  free, so any 'k-th smallest/largest' question on a BST should make me reach for an
  early-exit traversal before reaching for sorting or a heap."

### Vocabulary Builder

- **in-order traversal** (n. phrase) — visiting left subtree, then node, then right
  subtree; the specific traversal order that yields sorted output on a BST, unlike
  pre-order or post-order.
- **early exit** (n. phrase) — returning as soon as the answer is known rather than
  finishing a full computation; here, stopping the moment the k-th node is popped instead
  of building the whole sorted list.
- **"…avoids materializing the full result"** — a reusable phrase for justifying an
  early-exit or streaming approach over one that builds an entire intermediate structure
  just to throw most of it away.
- **explicit stack** (n. phrase) — a manually managed stack used to simulate recursion
  iteratively; worth naming when asked to convert a recursive traversal into an iterative
  one, since it's the standard technique for that conversion.
