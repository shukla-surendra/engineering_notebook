# 5. Linked List Cycle

**Difficulty:** Easy
**Topic:** Linked List
**Pattern:** Floyd's Cycle Detection (slow/fast pointers)

## Problem
Given the head of a linked list, determine if it has a cycle (some node's `next` points
back to an earlier node in the list).

## Examples
```
Input: 3->2->0->-4 with tail connecting back to node at index 1 -> True
Input: 1->2 with no cycle -> False
```

## Approach
Floyd's Tortoise and Hare: move `slow` one step and `fast` two steps at a time. If there's
a cycle, `fast` will eventually lap `slow` and they'll meet inside the loop. If `fast`
(or `fast.next`) hits `None`, there's no cycle. Uses no extra data structure, unlike the
hash-set-of-visited-nodes alternative (O(n) space).

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Floyd's Cycle Detection (slow/fast pointers)**,
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
Runnable, with sample test cases at the bottom (`python3 linked_list/05_linked_list_cycle/solution.py`):

```python
--8<-- "linked_list/05_linked_list_cycle/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The straightforward version
  is a hash set of visited nodes — the moment I revisit a node I've seen, there's a
  cycle. O(n) time and O(n) space. I'd name that, then say Floyd's algorithm gets the
  same O(n) time with O(1) space by using a second pointer instead of memory."
- **Invariant/proof framing (good for showing you understand *why* it works, not just
  that it does):** "The invariant is that the gap between `fast` and `slow` shrinks by
  exactly one node every step once both are inside the loop, so they're mathematically
  guaranteed to collide — it's not a heuristic. I'd say that out loud so it's clear I'm
  not just reciting the trick."
- **Generalization framing (good for signaling pattern recognition):** "This is the
  canonical use of fast/slow pointers, the same primitive that finds a list's midpoint in
  Reorder List — I'd mention that a two-speed pointer pair is a general tool for 'detect a
  repeating structure without extra memory.'"

### Vocabulary Builder

- **trades memory for speed** (phrase, inverted here) — actually this problem trades
  memory *away*: *"Floyd's algorithm trades the hash set's O(n) space for O(1), at no
  cost to time — that's a strict win, not really a trade-off."*
- **converge** (v.) — for two things approaching the same state or value; useful for
  describing how `fast` and `slow` inevitably meet inside a cycle.
- **"guaranteed, not just likely"** — a reusable phrase for distinguishing a proven
  invariant from a solution that merely works on the examples you tried.
- **auxiliary space** (n. phrase) — extra memory used beyond the input itself, e.g. the
  hash set in the brute-force version; contrasting it against Floyd's O(1) is the whole
  pitch for this technique.
