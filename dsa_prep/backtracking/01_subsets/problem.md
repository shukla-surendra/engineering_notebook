# 1. Subsets

**Difficulty:** Medium
**Topic:** Backtracking
**Pattern:** Include/exclude decision tree

## Problem
Given an array `nums` of unique integers, return all possible subsets (the power set), in
any order.

## Examples
```
Input: nums = [1,2,3]
Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
```

## Approach
At each index, backtrack over two choices: include `nums[i]` in the current subset, or
don't. Recurse to the next index either way, and record the current subset (a copy) at
every recursive call — every node of this decision tree is a valid subset, not just the
leaves. This "include/exclude" template generalizes to many other backtracking problems.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Include/exclude decision tree**, which itself
belongs to the broader **Backtracking** family of techniques. If the specific trick
above feels like it came out of nowhere, that's the signal to step back and read
[`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of problems in
general (not just this one), the reusable template you can write from memory, the usual
variations, and the mistakes people make applying it. Coming back to re-read this
problem's approach afterward should make the specific choices here feel inevitable
rather than clever.

## Complexity
- Time: O(2^n) — that many subsets exist
- Space: O(n) recursion depth (excluding output)

## Solution
Runnable, with sample test cases at the bottom (`python3 backtracking/01_subsets/solution.py`):

```python
--8<-- "backtracking/01_subsets/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Decision-tree framing (the default way to open this one):** "I'd draw the recursion as
  a binary decision tree — at each index, go left meaning 'exclude,' go right meaning
  'include.' There are n binary decisions total, so 2^n leaves, which is exactly the size
  of the power set."
- **Invariant framing (what makes every node valid, not just the leaves):** "The invariant
  is that `path` always represents a valid subset at *every* node of the recursion, not
  just at the leaves — so I record a copy of `path` on entry to every call, before making
  any further choice."
- **Pattern-recognition framing (names the reusable skeleton):** "This is the
  choose-explore-un-choose backtracking skeleton in its purest form — I'd say that
  explicitly, since recognizing it here is what makes Combination Sum and Permutations feel
  like variations rather than new problems."

### Vocabulary Builder

- **power set** (n.) — the set of all subsets of a set, including the empty set and the
  full set itself; has size 2^n for an n-element input.
- **decision tree** (n. phrase) — a way of visualizing recursive choices as branches in a
  tree; useful shorthand for narrating backtracking out loud without drawing it.
- **"the crux of it is…"** — a clean pivot phrase from restating the problem to stating the
  actual recursive insight.
- **shallow copy** (n. phrase) — copying a mutable list (`path[:]` or `list(path)`) before
  storing it, since appending the reference itself would let later mutations corrupt
  already-recorded answers.
