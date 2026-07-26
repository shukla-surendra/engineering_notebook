# 2. Combination Sum

**Difficulty:** Medium
**Topic:** Backtracking
**Pattern:** Backtracking with unlimited reuse of each candidate

## Problem
Given an array of distinct integers `candidates` and a `target`, return all unique
combinations where the chosen numbers sum to `target`. The same number may be reused an
unlimited number of times.

## Examples
```
Input: candidates = [2,3,6,7], target = 7 -> [[2,2,3],[7]]
Input: candidates = [2,3,5], target = 8   -> [[2,2,2,2],[2,3,3],[3,5]]
```

## Approach
Backtrack starting from index `i`, choosing whether to include `candidates[i]` (staying at
index `i` again, since reuse is allowed) or move on to `i + 1`. Subtract the chosen value
from the remaining target; a remaining target of 0 is a valid combination, and going
negative prunes that branch. Always advancing `i` (never revisiting earlier indices)
naturally avoids generating duplicate combinations in different orders.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Backtracking with unlimited reuse of each
candidate**, which itself belongs to the broader **Backtracking** family of techniques.
If the specific trick above feels like it came out of nowhere, that's the signal to step
back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family
of problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: exponential in the worst case, bounded by the number of valid combinations
- Space: O(target / min(candidates)) recursion depth

## Solution
Runnable, with sample test cases at the bottom (`python3 backtracking/02_combination_sum/solution.py`):

```python
--8<-- "backtracking/02_combination_sum/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Pruning-first framing (state the cutoff before the recursion):** "The moment the
  remaining target goes negative, that whole branch is dead — I prune it immediately
  instead of letting the recursion bottom out on its own. Pruning early is what keeps this
  from exploring far more of the tree than it needs to."
- **Invariant framing (why duplicates never appear):** "The invariant is: I never revisit
  an index smaller than where I currently am. That's what guarantees `[2,3]` and `[3,2]`
  never both get generated — order in the combination is fixed by the order I visit
  indices, not by the order I add numbers."
- **Contrast framing (distinguishes it from Permutations/Subsets by name):** "This is
  backtracking with reuse allowed — the recursive call stays at index `i` instead of
  advancing to `i+1` when I include a candidate. I'd contrast that explicitly with
  Permutations, where a `used` tracker forbids reuse."

### Vocabulary Builder

- **prune** (v.) — to abandon a branch of the recursion early because it provably cannot
  lead to a valid answer, without exploring it fully. *"Once remaining target goes
  negative, I prune that branch immediately."*
- **search space** (n. phrase) — the full set of possibilities a backtracking algorithm
  could in principle explore; pruning shrinks how much of it actually gets visited.
- **"the naive approach breaks down when…"** — useful for explaining why unconstrained
  reuse without the non-decreasing-index rule would generate the same combination multiple
  times in different orders.
- **base case** (n. phrase) — the termination condition of the recursion; here, remaining
  target exactly zero (record the combination) or negative (dead branch, return).
