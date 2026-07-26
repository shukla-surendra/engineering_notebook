# 3. Same Tree

**Difficulty:** Easy
**Topic:** Trees
**Pattern:** Recursive structural comparison

## Problem
Given the roots of two binary trees, return `True` if they are structurally identical and
their nodes have the same values.

## Examples
```
Input: p = [1,2,3], q = [1,2,3] -> True
Input: p = [1,2], q = [1,null,2] -> False
```

## Approach
Recursively compare: if both nodes are `None`, they match. If exactly one is `None`, or
their values differ, they don't. Otherwise recurse on left-vs-left and right-vs-right and
require both to match.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Recursive structural comparison**, which itself
belongs to the broader **Tree Traversal (DFS & BFS)** family of techniques. If the
specific trick above feels like it came out of nowhere, that's the signal to step back
and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of
problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(min(n, m))
- Space: O(min(h1, h2)) recursion stack

## Solution
Runnable, with sample test cases at the bottom (`python3 trees/03_same_tree/solution.py`):

```python
--8<-- "trees/03_same_tree/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "There's no meaningfully
  slower alternative to name here — you can't determine structural equality without
  visiting every node in at least one tree, so O(min(n, m)) with early exit is already the
  floor. I'd say that instead of manufacturing a fake brute force."
- **Invariant framing (good for the three-way branch in the recursion):** "The invariant
  I'm checking at every step is 'same shape, same value, so far.' The three-way branch —
  both `None`, exactly one `None`, or both present but different values — has to be
  exhaustive, because missing the 'exactly one is `None`' case is the classic silent bug
  that only shows up on asymmetric trees."
- **Generalization framing (good for connecting to Subtree of Another Tree):** "This is a
  structural-comparison DFS, and I'd flag it's about to become a subroutine — Subtree of
  Another Tree literally calls this same-tree check at every node of a bigger tree, so
  getting this one exactly right pays off twice."

### Vocabulary Builder

- **short-circuit** (v.) — to stop evaluating as soon as the result is determined; the
  `and` in the recursive call short-circuits so a left mismatch skips checking the right
  side entirely.
- **exhaustive** (adj.) — covering every possible case with no gaps; used to describe a
  set of conditional branches that leaves nothing unhandled.
- **structural equality** (n. phrase) — sameness of shape *and* values, as opposed to
  reference equality (same object in memory) — worth distinguishing explicitly since
  interviewers sometimes probe this distinction.
- **"…the classic silent bug"** — a reusable phrase for flagging an edge case that
  compiles and often passes casual testing but is wrong (e.g. comparing `.val` before
  confirming neither node is `None`).
