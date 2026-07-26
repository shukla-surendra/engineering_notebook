# 4. Word Search

**Difficulty:** Medium
**Topic:** Backtracking
**Pattern:** Grid DFS/backtracking with in-place visited marking

## Problem
Given an `m x n` grid of characters `board` and a string `word`, return `True` if `word`
exists as a path of adjacent cells (horizontally or vertically), using each cell at most
once.

## Examples
```
Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
Output: True
```

## Approach
Try starting the DFS from every cell that matches `word[0]`. During the DFS, if the
current cell matches `word[i]`, temporarily mark it visited (e.g. overwrite with a
sentinel character) and recurse into all 4 neighbors looking for `word[i+1]`; restore the
cell afterward (backtrack) whether or not that branch succeeded. Reaching `i == len(word)`
means the whole word was matched.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Grid DFS/backtracking with in-place visited
marking**, which itself belongs to the broader **Backtracking** family of techniques. If
the specific trick above feels like it came out of nowhere, that's the signal to step
back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family
of problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(m·n·4^L), L = len(word) — heavily pruned in practice
- Space: O(L) recursion stack

## Solution
Runnable, with sample test cases at the bottom (`python3 backtracking/04_word_search/solution.py`):

```python
--8<-- "backtracking/04_word_search/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Grid-DFS framing (how to open the whiteboard walkthrough):** "I'd treat this as DFS
  over a grid, with backtracking baked in — from any matching cell, try all four
  neighbors, and if a branch fails, undo whatever I marked and try the next direction. The
  grid version of choose-explore-un-choose."
- **In-place-marking framing (why I don't need a separate visited set):** "Rather than
  allocating a separate visited grid, I overwrite the current cell with a sentinel
  character while exploring from it, then restore the original character on the way back
  up — that's cheaper than a second data structure and just as correct, as long as I
  restore unconditionally."
- **Pruning framing (why this beats brute-force path enumeration):** "The brute-force
  version would enumerate every possible path from every cell; instead I bail out the
  instant a cell doesn't match `word[i]`, so most branches die in O(1) rather than
  continuing to explore dead ends."

### Vocabulary Builder

- **sentinel value** (n. phrase) — a placeholder value (like `'#'`) used to mark a cell as
  "currently in use," distinguishable from any real input character.
- **in-place** (adj.) — modifying existing memory directly rather than allocating a
  parallel structure; here, marking visited cells on the board itself instead of a separate
  visited set.
- **"restore unconditionally"** — a precise phrase for describing that backtracking must
  undo a mark whether the recursive branch succeeded or failed, not only on failure.
- **branching factor** (n. phrase) — the number of choices available at each step of the
  search (up to 4 neighbors here); explains why the complexity bound includes a `4^L` term.
