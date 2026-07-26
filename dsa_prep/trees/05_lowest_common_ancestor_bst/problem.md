# 5. Lowest Common Ancestor of a Binary Search Tree

**Difficulty:** Medium
**Topic:** Trees
**Pattern:** BST property-guided traversal

## Problem
Given a BST and two nodes `p` and `q` that exist in it, return their lowest common
ancestor (the deepest node that has both `p` and `q` as descendants, a node can be its own
descendant).

## Examples
```
Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 8 -> 6
Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 4 -> 2
```

## Approach
Exploit the BST ordering instead of general tree LCA: starting at the root, if both `p.val`
and `q.val` are less than the current node, the LCA must be in the left subtree; if both
are greater, it's in the right subtree. Otherwise (values on either side, or equal to the
current node) the current node is the split point — the LCA.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **BST property-guided traversal**, which itself
belongs to the broader **Tree Traversal (DFS & BFS)** family of techniques. If the
specific trick above feels like it came out of nowhere, that's the signal to step back
and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of
problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(h), h = tree height (O(log n) balanced, O(n) worst case)
- Space: O(1) iterative

## Solution
Runnable, with sample test cases at the bottom (`python3 trees/05_lowest_common_ancestor_bst/solution.py`):

```python
--8<-- "trees/05_lowest_common_ancestor_bst/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The general-tree brute force
  is: find the root-to-`p` and root-to-`q` paths, then walk both lists to find where they
  last agree — that works on any binary tree but costs O(n) and O(n) extra space. I'd name
  that first, then say the BST ordering makes it unnecessary."
- **Invariant framing (good for explaining why you don't need to search both sides):** "The
  invariant I'm exploiting is that a BST's ordering tells me, without looking, which side
  a value lives on. So the moment `p` and `q` fall on opposite sides of the current node —
  or one equals it — that node *is* the split point, and I can stop; I never need to
  explore both subtrees like a general-tree LCA would."
- **Generalization framing (good for signaling BST-specific thinking):** "I'd flag this is
  a 'let the ordering do the searching' problem — the same instinct that makes Kth Smallest
  cheap without a full traversal. Whenever a problem says BST specifically, not just binary
  tree, that's a cue to ask what the ordering buys me before reaching for general DFS."

### Vocabulary Builder

- **split point** (n. phrase) — the node where two paths diverge; here, the first node
  where `p` and `q` stop being on the same side, which is exactly the LCA by definition.
- **degenerate case** (n. phrase) — an edge case that's valid but easy to mishandle; here,
  when one of `p`/`q` *is* an ancestor of the other, so "the LCA" is literally one of the
  two input nodes.
- **"…let the ordering do the searching"** — a reusable phrase for describing any
  algorithm that uses sortedness to eliminate branches instead of exploring them.
- **prune** (v.) — to eliminate a branch of the search space without visiting it; the BST
  property prunes one entire subtree at every step, which is why this runs in O(h) instead
  of O(n).
