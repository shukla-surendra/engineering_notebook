# 2. Merge Two Sorted Lists

**Difficulty:** Easy
**Topic:** Linked List
**Pattern:** Dummy head + two-pointer merge

## Problem
Merge two sorted linked lists `list1` and `list2` into one sorted list by splicing
together their nodes, and return the head.

## Examples
```
Input: list1 = 1->2->4, list2 = 1->3->4 -> 1->1->2->3->4->4
```

## Approach
Use a dummy head node and a `tail` pointer. Repeatedly compare the current heads of both
lists, attach the smaller to `tail.next`, and advance that list's pointer and `tail`. When
one list is exhausted, attach the remainder of the other directly (no need to keep
comparing one at a time). Return `dummy.next`.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Dummy head + two-pointer merge**, which itself
belongs to the broader **Linked List Pointer Manipulation** family of techniques. If the
specific trick above feels like it came out of nowhere, that's the signal to step back
and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of
problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(n + m)
- Space: O(1) (reuses existing nodes)

## Solution
Runnable, with sample test cases at the bottom (`python3 linked_list/02_merge_two_sorted_lists/solution.py`):

```python
--8<-- "linked_list/02_merge_two_sorted_lists/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The naive route is to
  collect both lists' values, sort the combined array, and build a new list from it —
  O((n+m) log(n+m)). I'd point out that's wasteful because both inputs are already
  sorted; a linear merge exploits that instead of throwing the ordering away."
- **Invariant framing (good for explaining the dummy-head merge precisely):** "The
  invariant is: everything already attached to `dummy` is fully sorted and final. Each
  step I just compare the two current heads and attach the smaller — I never have to
  revisit a decision, which is what makes the single pass valid."
- **Generalization framing (good for signaling pattern recognition):** "This two-pointer
  merge is the primitive Merge k Sorted Lists scales up with a heap, and it's the same
  merge step from merge sort — I'd name that connection to show I see the family, not
  just this instance."

### Vocabulary Builder

- **sentinel node** (n. phrase) — a placeholder node (here, `dummy`) that simplifies edge
  cases by giving you something to attach to before you know the real head.
  *"Using a sentinel node means I never special-case 'what if the result starts with
  list2.'"*
- **splice** (v.) — to join or interleave two sequences by relinking pointers rather than
  copying values.
- **"trades a full sort for a linear scan"** — a compact phrase for justifying why
  exploiting existing sortedness beats a generic sort-based approach.
- **exhausted** (adj.) — describing a pointer/list that has been fully consumed; useful
  for narrating the loop-termination condition ("once one list is exhausted, I just
  attach the remainder of the other").
