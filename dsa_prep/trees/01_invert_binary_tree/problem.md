# 1. Invert Binary Tree

**Difficulty:** Easy
**Topic:** Trees
**Pattern:** Recursive DFS (swap children)

## Problem
Given the root of a binary tree, invert it (mirror left and right children at every node)
and return the root.

## Examples
```
Input: root = [4,2,7,1,3,6,9] -> [4,7,2,9,6,3,1]
```

## Approach
Classic recursive DFS: swap a node's left and right children, then recurse into both
(now-swapped) subtrees. Base case: `None` returns `None`. An iterative BFS/DFS with an
explicit queue/stack works identically well.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Recursive DFS (swap children)**, which itself
belongs to the broader **Tree Traversal (DFS & BFS)** family of techniques. If the
specific trick above feels like it came out of nowhere, that's the signal to step back
and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of
problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(n)
- Space: O(h) recursion stack, h = tree height

## Solution
Runnable, with sample test cases at the bottom (`python3 trees/01_invert_binary_tree/solution.py`):

```python
--8<-- "trees/01_invert_binary_tree/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "There isn't really a slower
  brute force here — the honest starting point is just 'walk every node once and swap its
  children,' so I'd say that out loud and move straight to whether I do it recursively or
  with an explicit stack, since that's the actual decision point."
- **Invariant framing (good for narrating the recursion cleanly):** "The invariant is: by
  the time I return from a call on `node`, its entire subtree is already mirrored. That's
  why I swap the children *and then* recurse into them — swap first so the recursive calls
  are operating on the already-swapped pointers, not the original ones."
- **Generalization framing (good for signaling pattern recognition):** "This is the
  simplest possible post-order-shaped DFS — combine-then-recurse-or-recurse-then-combine
  barely matters here since the operation is symmetric, but I'd name it as the same
  traversal skeleton I'd reuse for max depth or same-tree."

### Vocabulary Builder

- **mirror** (v.) — to reflect a structure so left and right are swapped at every level, not
  just the top. *"Inverting the tree means mirroring it recursively, not just swapping the
  root's two children."*
- **base case** (n.) — the recursion's stopping condition; here, a `None` node returns
  `None` immediately. Naming it out loud prevents an infinite-recursion bug.
- **"…the traversal skeleton is the same either way"** — a reusable phrase for pointing out
  that recursive and iterative (BFS/DFS-with-stack) solutions differ only in bookkeeping,
  not in the underlying idea.
- **idempotent** (adj.) — an operation that produces the same result if applied twice;
  worth mentioning that inverting twice returns the original tree, which is a quick
  self-check of correctness.
