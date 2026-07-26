# 7. Validate Binary Search Tree

**Difficulty:** Medium
**Topic:** Trees
**Pattern:** DFS with a propagated valid range

## Problem
Given the root of a binary tree, determine if it is a valid BST: every node's value must
be strictly greater than **all** values in its left subtree and strictly less than **all**
values in its right subtree — not just its immediate children.

## Examples
```
Input: root = [2,1,3]     -> True
Input: root = [5,1,4,null,null,3,6] -> False  (4 < 5 but sits in the right subtree)
```

## Approach
A common bug is only comparing a node to its direct children. Instead, pass down a valid
`(low, high)` range as you recurse: the root's range is `(-inf, +inf)`; going left tightens
the upper bound to the parent's value, going right tightens the lower bound. A node is
valid only if `low < node.val < high`. (An in-order-traversal-must-be-strictly-increasing
approach is an equally valid alternative.)

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **DFS with a propagated valid range**, which
itself belongs to the broader **Tree Traversal (DFS & BFS)** family of techniques. If
the specific trick above feels like it came out of nowhere, that's the signal to step
back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family
of problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(n)
- Space: O(h) recursion stack

## Solution
Runnable, with sample test cases at the bottom (`python3 trees/07_validate_binary_search_tree/solution.py`):

```python
--8<-- "trees/07_validate_binary_search_tree/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The tempting-but-wrong
  brute force is comparing each node only to its immediate children — I'd name that
  explicitly as the bug I'm avoiding, not just skip past it, since flagging the wrong
  approach before writing the right one shows I understand *why* it's wrong."
- **Invariant framing (good for the propagated-range approach):** "The invariant is: by
  the time I'm validating `node`, I already know the full open interval `(low, high)` its
  value must fall in, inherited from every ancestor's decision, not just its parent's. That
  invariant is exactly what a naive 'compare to children' check throws away."
- **Generalization framing (good for contrasting two valid strategies):** "There are two
  legitimate framings of this problem, and I'd name both: pre-order with a propagated
  range, or in-order traversal checked for strict monotonic increase. I'd pick the range
  version because it can short-circuit early without materializing a full traversal list."

### Vocabulary Builder

- **strictly increasing / monotonic** (adj.) — always increasing, never equal or
  decreasing, between consecutive elements; the property an in-order BST traversal must
  satisfy, and the basis of the alternative solution strategy.
- **propagate** (v.) — to pass information down through recursive calls as an argument
  rather than recomputing it; here, tightening the `(low, high)` bound at each level.
- **open interval** (n. phrase) — a bound that excludes its endpoints (`low < x < high`);
  worth naming precisely since "strictly greater/less than" is the exact requirement this
  problem states, and using `<=` anywhere is an easy off-by-one bug.
- **"…the crux of it is…"** — a general-purpose phrase for pivoting from problem
  restatement into the actual insight, useful whenever you want to flag "this next part is
  the part that matters."
