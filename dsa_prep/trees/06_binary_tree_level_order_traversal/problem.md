# 6. Binary Tree Level Order Traversal

**Difficulty:** Medium
**Topic:** Trees
**Pattern:** BFS with level-size snapshotting

## Problem
Given the root of a binary tree, return the values of its nodes grouped level by level
(left to right within each level).

## Examples
```
Input: root = [3,9,20,null,null,15,7] -> [[3],[9,20],[15,7]]
```

## Approach
Standard BFS with a queue. The key trick for grouping by level: before draining the queue
for the next level, snapshot its current size (`level_size = len(queue)`) and process
exactly that many nodes, pushing their children for the next round. This cleanly separates
levels without needing sentinel markers.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **BFS with level-size snapshotting**, which
itself belongs to the broader **Tree Traversal (DFS & BFS)** family of techniques. If
the specific trick above feels like it came out of nowhere, that's the signal to step
back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family
of problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(n)
- Space: O(n) (queue + output)

## Solution
Runnable, with sample test cases at the bottom (`python3 trees/06_binary_tree_level_order_traversal/solution.py`):

```python
--8<-- "trees/06_binary_tree_level_order_traversal/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "A tempting-but-clunky
  alternative is a DFS that tracks depth and appends to `result[depth]`, creating new lists
  as needed — that works, but it means depth bookkeeping travels through every recursive
  call. I'd name that, then say BFS is the more natural fit since 'level' is literally
  what a queue processes in order."
- **Invariant framing (good for explaining the level-size trick precisely):** "The
  invariant is: at the top of each `while` iteration, the queue contains exactly the nodes
  of the current level and nothing from the next one yet. Snapshotting `len(queue)` before
  I start popping is what preserves that invariant — if I checked the length mid-loop
  instead, I'd be measuring a queue that already has next-level children mixed in."
- **Generalization framing (good for connecting to the wider BFS family):** "This is the
  canonical level-order BFS template — the same snapshot-then-drain trick shows up anytime
  a problem needs 'shortest path' or 'level-by-level' answers on an unweighted graph, not
  just trees."

### Vocabulary Builder

- **snapshot** (v./n.) — to capture a value at a specific moment before it can change;
  here, capturing `len(queue)` before the inner loop starts mutating the queue.
- **sentinel** (n.) — a special marker value used to signal a boundary (e.g. a `None`
  pushed between levels); worth naming as the alternative to level-size snapshotting, and
  why the snapshot approach avoids needing one.
- **"…measuring a moving target"** — a useful phrase for describing a bug where you check
  a value (like queue length) after it's already been mutated by the same loop.
- **unweighted graph** (n. phrase) — a graph where BFS guarantees shortest paths because
  every edge costs the same; worth mentioning since level-order traversal is BFS's
  tree-specific special case.
