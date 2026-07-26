# 3. Permutations

**Difficulty:** Medium
**Topic:** Backtracking
**Pattern:** Backtracking with a "used" tracker (or swap-in-place)

## Problem
Given an array `nums` of distinct integers, return all possible permutations, in any order.

## Examples
```
Input: nums = [1,2,3]
Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
```

## Approach
Build permutations one position at a time: at each step, try every not-yet-used number as
the next element, recurse, then backtrack (unmark it as used) before trying the next
candidate. A boolean `used` array (or a set) tracks which numbers are already placed in the
current path. A full path (length == len(nums)) is a complete permutation.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Backtracking with a "used" tracker (or swap-in-
place)**, which itself belongs to the broader **Backtracking** family of techniques. If
the specific trick above feels like it came out of nowhere, that's the signal to step
back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family
of problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(n! · n) — n! permutations, each built in O(n)
- Space: O(n) recursion depth (excluding output)

## Solution
Runnable, with sample test cases at the bottom (`python3 backtracking/03_permutations/solution.py`):

```python
--8<-- "backtracking/03_permutations/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **State-tracking framing (the detail that differentiates this from Subsets):** "Unlike
  Subsets, position matters here and every element must appear exactly once per
  permutation — so I need a `used` tracker to know, at each recursive level, which numbers
  are still available to place next."
- **Invariant framing (what the backtrack step must undo):** "The invariant is: `used`
  always reflects exactly what's currently in `path`, nothing more, nothing less. That's
  why unmarking a number as used has to happen right after the recursive call returns —
  skip that line and the invariant breaks for every sibling branch that runs afterward."
- **Complexity framing (justifying the n! term without hand-waving):** "There are n!
  orderings total, and building each one costs O(n) to construct the path, so I'd state the
  bound as O(n! · n) rather than just O(n!), since the interviewer may probe on that
  distinction."

### Vocabulary Builder

- **used tracker** (n. phrase) — a boolean array or set marking which elements are already
  placed in the current path, checked before considering an element as the next choice.
- **backtrack** (v.) — to undo the most recent choice after exploring its consequences, so
  sibling branches start from a clean state. *"I backtrack by popping the last element and
  unmarking it as used."*
- **"state leaks between branches"** — the standard phrase for describing the bug that
  happens when you forget to backtrack (undo a choice) before trying the next option.
- **factorial growth** (n. phrase) — describing complexity that scales as n!, which grows
  faster than exponential (2^n) — worth naming to show you know it's a harder ceiling than
  typical subset/combination problems.
