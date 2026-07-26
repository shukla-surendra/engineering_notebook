# 4. Subtree of Another Tree

**Difficulty:** Easy
**Topic:** Trees
**Pattern:** Recursive DFS + "same tree" as a subroutine

## Problem
Given the roots of two binary trees `root` and `subRoot`, return `True` if there's a node
in `root` such that the subtree rooted at that node is identical to `subRoot`.

## Examples
```
Input: root = [3,4,5,1,2], subRoot = [4,1,2] -> True
Input: root = [3,4,5,1,2,null,null,null,null,0], subRoot = [4,1,2] -> False
```

## Approach
Reuse the "Same Tree" check as a subroutine. Walk `root` with DFS; at every node, check if
the subtree rooted there is identical to `subRoot`. If any node passes, return `True`.
Worst case compares at every node, each comparison bounded by `subRoot`'s size.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Recursive DFS + "same tree" as a subroutine**,
which itself belongs to the broader **Tree Traversal (DFS & BFS)** family of techniques.
If the specific trick above feels like it came out of nowhere, that's the signal to step
back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family
of problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(n · m) worst case (n = size of root, m = size of subRoot)
- Space: O(h) recursion stack

## Solution
Runnable, with sample test cases at the bottom (`python3 trees/04_subtree_of_another_tree/solution.py`):

```python
--8<-- "trees/04_subtree_of_another_tree/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The brute force is: at every
  node in `root`, run a full same-tree check against `subRoot`. That's already what I'd
  ship — the 'optimization' conversation here is really about whether that's acceptably
  bounded, which it is, at O(n·m) worst case, since `subRoot`'s size caps each individual
  check."
- **Invariant framing (good for explaining the composition of two traversals):** "I'm
  composing two traversals: an outer DFS over `root` that asks 'could this node be the
  match point,' and an inner same-tree check that verifies it once we guess yes. Keeping
  those as two separate functions — rather than tangling the logic into one recursive
  case — is what keeps the invariant of each one clean and independently correct."
- **Generalization framing (good for the 'reuse a solved subproblem' habit):** "This is
  the pattern of reusing an already-solved building block — same-tree — as a subroutine
  inside a bigger traversal. I'd call that out explicitly, since spotting 'I already solved
  a smaller version of this' is a transferable interview skill, not just a trick for this
  problem."

### Vocabulary Builder

- **subroutine** (n.) — a self-contained function invoked as a step within a larger
  algorithm; here, the same-tree check called from inside a bigger DFS.
- **composability** (n.) — the property of building a solution by combining smaller,
  independently-correct pieces rather than one monolithic block of logic.
- **"…bounded by the smaller of the two"** — a compact phrase for explaining why a nested
  comparison's cost is capped by the size of the thing you're checking against, not the
  thing you're searching through.
- **worst case** (n. phrase) — the input that maximizes cost, as opposed to average case;
  here, a `root` that's mostly-but-not-quite matching `subRoot` at every node forces the
  full O(n·m) bound, worth naming when asked "can this be improved?"
