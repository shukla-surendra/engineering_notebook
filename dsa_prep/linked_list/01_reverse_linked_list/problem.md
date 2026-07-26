# 1. Reverse Linked List

**Difficulty:** Easy
**Topic:** Linked List
**Pattern:** Iterative pointer reversal

## Problem
Given the head of a singly linked list, reverse it and return the new head.

## Examples
```
Input: 1->2->3->4->5 -> 5->4->3->2->1
Input: [] -> []
```

## Approach
Walk the list with `prev = None` and `curr = head`. At each node, save `curr.next` before
overwriting it to point backward at `prev`, then advance both `prev` and `curr` forward.
When `curr` becomes `None`, `prev` is the new head. (A recursive version is equally
common: recurse to the tail, then rewire `next.next = curr` on the way back up.)

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Iterative pointer reversal**, which itself
belongs to the broader **Linked List Pointer Manipulation** family of techniques. If the
specific trick above feels like it came out of nowhere, that's the signal to step back
and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of
problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(n)
- Space: O(1) iterative (O(n) recursion stack if done recursively)

## Solution
Runnable, with sample test cases at the bottom (`python3 linked_list/01_reverse_linked_list/solution.py`):

```python
--8<-- "linked_list/01_reverse_linked_list/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The naive way is to dump
  every value into an array, reverse the array, and rebuild a new list from it — O(n)
  time but O(n) extra space. I'd name that, then say the in-place version does the same
  job with O(1) space by rewiring pointers as I walk instead of copying values."
- **Invariant framing (good for narrating the loop precisely):** "The invariant I hold at
  the top of every iteration is: `prev` is the head of the fully-reversed portion so far,
  and `curr` is the next unprocessed node. I have to save `curr.next` before I overwrite
  it, or I lose the rest of the list — that ordering is the whole trick."
- **Generalization framing (good for signaling pattern recognition):** "This is the base
  case of the broader linked-list pointer-manipulation family — the same 'save-before-
  overwrite' discipline shows up in Reorder List and Merge k Sorted Lists, so I'd mention
  I'm reaching for a known template, not improvising."

### Vocabulary Builder

- **in-place** (adj.) — modifying a structure using O(1) extra space rather than building
  a new one. *"I can do this in-place since I only need three pointer variables."*
- **invariant** (n.) — a condition that holds true at a fixed point in every loop
  iteration; naming it out loud proves you understand *why* the code is correct, not just
  that it happens to work.
- **"the crux of it is…"** — a reusable phrase for pointing at the one line or decision
  that makes the whole approach work, right after you've walked through the mechanics.
- **degenerate case** (n. phrase) — a trivially small or empty input (empty list, single
  node) that's still technically valid — mentioning it shows you check boundaries by
  default.
