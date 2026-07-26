# 6. Merge k Sorted Lists

**Difficulty:** Hard
**Topic:** Linked List
**Pattern:** Min-heap of current heads (or divide & conquer pairwise merge)

## Problem
Given an array of `k` linked lists, each sorted ascending, merge them into one sorted
linked list and return its head.

## Examples
```
Input: lists = [[1,4,5],[1,3,4],[2,6]] -> [1,1,2,3,4,4,5,6]
```

## Approach
Push the head node of every non-empty list onto a min-heap keyed by value (with a tie
breaker index to avoid comparing `ListNode` objects directly). Repeatedly pop the smallest,
append it to the result, and if it has a `next`, push that onto the heap. This is
O(N log k) where N is the total number of nodes. (An alternative divide-and-conquer
approach pairwise-merges the lists, also O(N log k), without needing a heap.)

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Min-heap of current heads (or divide & conquer
pairwise merge)**, which itself belongs to the broader **Linked List Pointer
Manipulation** family of techniques. If the specific trick above feels like it came out
of nowhere, that's the signal to step back and read [`../PATTERN.md`](../PATTERN.md) —
it covers how to recognize this family of problems in general (not just this one), the
reusable template you can write from memory, the usual variations, and the mistakes
people make applying it. Coming back to re-read this problem's approach afterward should
make the specific choices here feel inevitable rather than clever.

## Complexity
- Time: O(N log k) where N = total nodes, k = number of lists
- Space: O(k) for the heap

## Solution
Runnable, with sample test cases at the bottom (`python3 linked_list/06_merge_k_sorted_lists/solution.py`):

```python
--8<-- "linked_list/06_merge_k_sorted_lists/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The naive route merges the
  lists one at a time — merge list 1 into the result, then list 2, then list 3, and so
  on. That's O(N·k) in the worst case because each merge re-scans the growing result. I'd
  name that before jumping to the heap, since it's the natural first instinct."
- **Invariant framing (good for explaining the heap approach precisely):** "The invariant
  is: the heap always holds exactly the current smallest unconsumed node from every
  still-active list. Popping the global min and pushing its successor preserves that
  invariant, which is why popping k times in a row always gives you the next k smallest
  values in order."
- **Generalization framing (good for signaling pattern recognition):** "This generalizes
  Merge Two Sorted Lists — same 'always take the smallest available head' idea, just with
  a heap standing in for the two manual pointers once k grows past two. I'd also mention
  the divide-and-conquer alternative, pairwise-merging lists, as the same complexity
  without a heap."

### Vocabulary Builder

- **min-heap** (n.) — a tree-based structure giving O(log k) access to the current
  smallest of k elements; the right tool whenever you need repeated 'give me the min'
  queries with insertions interleaved.
- **tie-breaker** (n.) — a secondary comparison key (here, an index) used to avoid
  comparing incomparable objects directly. *"I add a tie-breaker index so the heap never
  tries to compare two `ListNode` objects when values are equal."*
- **"generalizes the two-list case"** — a reusable phrase for framing a k-way problem as
  a scaled-up version of a simpler two-way problem you already know.
- **divide and conquer** (n. phrase) — an alternative strategy here: pairwise-merge lists
  and repeat, halving the count each round — worth naming as the heap-free alternative
  with the same O(N log k) bound.
