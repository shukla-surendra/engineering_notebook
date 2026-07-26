# 2. Maximum Depth of Binary Tree

**Difficulty:** Easy
**Topic:** Trees
**Pattern:** Recursive DFS

## Problem
Given the root of a binary tree, return its maximum depth (number of nodes along the
longest path from root to the farthest leaf).

## Examples
```
Input: root = [3,9,20,null,null,15,7] -> 3
Input: root = [] -> 0
```

## Approach
The depth of a tree is `1 + max(depth(left), depth(right))`, with `None` having depth 0.
This recurrence maps directly onto a recursive DFS. A BFS level-by-level count works
equally well if an iterative solution is preferred.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Recursive DFS**, which itself belongs to the
broader **Tree Traversal (DFS & BFS)** family of techniques. If the specific trick above
feels like it came out of nowhere, that's the signal to step back and read
[`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of problems in
general (not just this one), the reusable template you can write from memory, the usual
variations, and the mistakes people make applying it. Coming back to re-read this
problem's approach afterward should make the specific choices here feel inevitable
rather than clever.

## Complexity
- Time: O(n)
- Space: O(h) recursion stack

## Solution
Runnable, with sample test cases at the bottom (`python3 trees/02_maximum_depth_of_binary_tree/solution.py`):

```python
--8<-- "trees/02_maximum_depth_of_binary_tree/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The naive idea and the
  optimal idea are basically the same here — there's no wasteful version to name and
  discard. I'd say that explicitly rather than pretending to 'optimize' a trivial O(n)
  problem, since inventing a fake brute force wastes interview time."
- **Invariant framing (good for justifying the recurrence):** "The invariant is that
  `depth(node)` always equals `1 + max(depth(left), depth(right))`, with depth of `None`
  defined as 0 so the recurrence doesn't need a special case for leaves — a leaf just falls
  out of the general formula naturally."
- **Generalization framing (good for connecting to the wider pattern):** "This is the
  textbook post-order DFS: recurse into both children first, then combine their results.
  I'd flag that the exact same skeleton, with a different `combine` function, solves Same
  Tree and Validate BST too."

### Vocabulary Builder

- **recurrence (relation)** (n.) — an equation defining a value in terms of the same
  function applied to smaller inputs; here, depth in terms of children's depths.
- **post-order** (adj.) — visiting a node *after* its children have already been
  processed; the natural order when a result is built bottom-up from children.
- **"…falls out of the general formula naturally"** — a useful phrase for showing an edge
  case (like a leaf, or an empty tree) doesn't need special-casing because the base case
  already handles it.
- **recursion stack** (n. phrase) — the implicit call stack consuming O(h) space during
  recursive DFS; worth naming when asked about space complexity, and worth contrasting with
  the O(w) a BFS/queue approach would use instead.
