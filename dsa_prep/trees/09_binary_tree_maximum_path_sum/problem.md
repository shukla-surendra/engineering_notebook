# 9. Binary Tree Maximum Path Sum

**Difficulty:** Hard
**Topic:** Trees
**Pattern:** Post-order DFS returning "best downward path", tracking a global max

## Problem
A path is any sequence of nodes connected by edges, and need not pass through the root or
be a straight line to a leaf. Given the root of a binary tree, return the maximum path
sum any path in the tree can have.

## Examples
```
Input: root = [1,2,3]         -> 6   (path 2 -> 1 -> 3)
Input: root = [-10,9,20,null,null,15,7] -> 42  (path 15 -> 20 -> 7)
```

## Approach
The subtlety: a node can be the "peak" of a path that uses **both** its children, but if
that node is itself used by its parent's path, the parent can only extend through **one**
side (a path can't branch). So define a recursive helper `max_gain(node)` that returns the
best sum extending downward through one side only (clamped at 0 — negative branches are
skipped). At each node, before returning that one-sided value to the parent, also check
`node.val + left_gain + right_gain` (the "peak through this node" case) against a global
running maximum.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Post-order DFS returning "best downward path",
tracking a global max**, which itself belongs to the broader **Tree Traversal (DFS &
BFS)** family of techniques. If the specific trick above feels like it came out of
nowhere, that's the signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it
covers how to recognize this family of problems in general (not just this one), the
reusable template you can write from memory, the usual variations, and the mistakes
people make applying it. Coming back to re-read this problem's approach afterward should
make the specific choices here feel inevitable rather than clever.

## Complexity
- Time: O(n)
- Space: O(h) recursion stack

## Solution
Runnable, with sample test cases at the bottom (`python3 trees/09_binary_tree_maximum_path_sum/solution.py`):

```python
--8<-- "trees/09_binary_tree_maximum_path_sum/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The naive idea is enumerate
  every path between every pair of nodes — that's exponential and clearly not where I'm
  going, but I'd say it out loud for one sentence to establish the target I'm optimizing
  away from before jumping into the DFS."
- **Invariant framing (good for the part that actually trips people up):** "The subtlety
  is that a node can be the *peak* of a path using both children, but it can only report
  *one* side upward to its own parent, because a path can't branch. So the invariant of my
  helper is: it returns the best single-sided downward extension, while a separate global
  variable tracks the best two-sided peak seen at any node along the way — those are two
  different questions and conflating them is the classic bug."
- **Generalization framing (good for naming the technique abstractly):** "This is 'return
  one thing to the parent, but silently accumulate a different global answer while you
  recurse' — the same shape shows up anytime a problem asks for a maximum over all
  possible paths/subtrees but the recursive contract can only pass one value upward."

### Vocabulary Builder

- **clamp** (v.) — to constrain a value to a minimum or maximum bound; here, clamping a
  negative one-sided gain to 0 so a worthless branch is excluded rather than subtracted.
- **global state** (n. phrase) — a variable that persists and mutates across recursive
  calls rather than being passed as an argument or return value; the running maximum here
  is exactly this, and naming it explicitly avoids confusing it with the function's return
  value.
- **"…the crux of it is…"** — a good pivot phrase for signaling "I've stated the problem,
  now here's the actual insight," useful right before explaining the one-sided-vs-peak
  distinction.
- **degenerate case** (n. phrase) — here, a tree of all-negative values, where the answer
  is just the single largest (least negative) node — worth mentioning to show the clamp-
  to-zero logic doesn't accidentally force a positive-sum path that doesn't exist.
