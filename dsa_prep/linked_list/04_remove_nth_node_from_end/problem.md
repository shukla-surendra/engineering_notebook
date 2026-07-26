# 4. Remove Nth Node From End of List

**Difficulty:** Medium
**Topic:** Linked List
**Pattern:** Two pointers with a fixed gap

## Problem
Given the head of a linked list, remove the `n`-th node from the end and return the head.
Do it in one pass.

## Examples
```
Input: head = 1->2->3->4->5, n = 2 -> 1->2->3->5
Input: head = 1, n = 1              -> []
Input: head = 1->2, n = 1            -> 1
```

## Approach
Use a dummy node before `head` (handles removing the head cleanly). Advance a `fast`
pointer `n` steps ahead first, then move `slow` (starting at dummy) and `fast` together
until `fast` reaches the last node. At that point `slow.next` is exactly the node to
remove — unlink it with `slow.next = slow.next.next`. One pass, no need to know the
length up front.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Two pointers with a fixed gap**, which itself
belongs to the broader **Linked List Pointer Manipulation** family of techniques. If the
specific trick above feels like it came out of nowhere, that's the signal to step back
and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of
problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(n)
- Space: O(1)

## Solution
Runnable, with sample test cases at the bottom (`python3 linked_list/04_remove_nth_node_from_end/solution.py`):

```python
--8<-- "linked_list/04_remove_nth_node_from_end/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The naive approach is two
  passes: count the list's length first, then walk again to the `(length - n)`-th node.
  That's still O(n), but it reads the list twice and the interviewer usually wants one
  pass — so I'd immediately pivot to the gap-pointer trick."
- **Invariant framing (good for explaining the two-pointer gap precisely):** "The
  invariant is that `fast` and `slow` stay exactly `n` nodes apart once the initial
  head-start finishes. When `fast` hits the last node, `slow` is guaranteed to be sitting
  right before the node I need to remove — that fixed gap is doing all the work."
- **Generalization framing (good for signaling pattern recognition):** "This is the
  'two pointers with a fixed gap' primitive from the pointer-manipulation family, and
  pairing it with a dummy head is the standard fix for 'what if I'm removing the head
  itself' — I'd name both pieces explicitly."

### Vocabulary Builder

- **one-pass** (adj. phrase) — an algorithm that reads its input exactly once; worth
  stating explicitly since it's often the actual ask, not just an optimization.
- **sentinel node** (n. phrase) — a dummy node preceding `head` that absorbs the
  "removing the first node" edge case so the main loop needs no special-casing.
- **"the naive approach breaks down when…"** — a reusable phrase for pivoting from a
  correct-but-suboptimal solution to the constraint that forces a better one (here: "when
  you're not allowed a second pass").
- **off-by-one** (n. phrase) — an error from advancing a pointer one step too many or too
  few; worth naming when you double-check whether `fast` should start `n` or `n+1` steps
  ahead.
