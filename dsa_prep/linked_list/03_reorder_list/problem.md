# 3. Reorder List

**Difficulty:** Medium
**Topic:** Linked List
**Pattern:** Fast/slow pointer split + reverse + merge

## Problem
Given a linked list `L0 -> L1 -> ... -> Ln-1 -> Ln`, reorder it in place to:
`L0 -> Ln -> L1 -> Ln-1 -> L2 -> Ln-2 -> ...`. You may not modify node values, only links.

## Examples
```
Input: 1->2->3->4   -> 1->4->2->3
Input: 1->2->3->4->5 -> 1->5->2->4->3
```

## Approach
Three steps, each a well-known sub-pattern: (1) find the middle with slow/fast pointers,
(2) reverse the second half in place, (3) merge the first half and reversed second half by
alternating nodes. Combining these three primitives is the whole trick.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Fast/slow pointer split + reverse + merge**,
which itself belongs to the broader **Linked List Pointer Manipulation** family of
techniques. If the specific trick above feels like it came out of nowhere, that's the
signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to
recognize this family of problems in general (not just this one), the reusable template
you can write from memory, the usual variations, and the mistakes people make applying
it. Coming back to re-read this problem's approach afterward should make the specific
choices here feel inevitable rather than clever.

## Complexity
- Time: O(n)
- Space: O(1)

## Solution
Runnable, with sample test cases at the bottom (`python3 linked_list/03_reorder_list/solution.py`):

```python
--8<-- "linked_list/03_reorder_list/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The easy version copies
  every node into an array, then uses two array pointers walking inward from both ends
  to rebuild the list — O(n) time, O(n) space. I'd name that first, then say I can drop
  the space to O(1) by doing the same 'walk from both ends' idea directly on the list."
- **Decomposition framing (good for explaining why this problem feels different):**
  "Rather than one clever trick, this is three known sub-routines chained together — find
  the middle, reverse the second half, merge by alternating. I'd say out loud that
  recognizing the decomposition *is* the problem; each piece individually is easy."
- **Generalization framing (good for signaling pattern recognition):** "Each of those
  three steps is a named primitive from the linked-list pointer-manipulation family —
  fast/slow for the middle, iterative reversal, two-pointer merge — so I'd frame this as
  composition rather than a new trick."

### Vocabulary Builder

- **compose** (v.) — to combine simpler operations into a larger solution; useful for
  describing multi-step approaches like this one. *"I'm composing three primitives I'd
  already use independently."*
- **interleave** (v.) — to alternate elements from two sequences into one combined
  sequence, as the final merge step does here.
- **"the difficulty here is decomposition, not a new trick"** — a reusable phrase for
  problems that are hard only because they chain familiar sub-problems together.
- **in-place** (adj.) — operating with O(1) extra memory by rewiring existing nodes
  instead of allocating a new structure.
