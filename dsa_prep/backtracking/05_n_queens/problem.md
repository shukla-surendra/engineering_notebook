# 5. N-Queens

**Difficulty:** Hard
**Topic:** Backtracking
**Pattern:** Row-by-row placement with O(1) column/diagonal conflict tracking

## Problem
Place `n` queens on an `n x n` chessboard so that no two queens attack each other (same
row, column, or diagonal). Return all distinct board configurations, each represented as a
list of strings.

## Examples
```
Input: n = 4
Output: [
 [".Q..","...Q","Q...","..Q."],
 ["..Q.","Q...","...Q",".Q.."]
]
```

## Approach
Place one queen per row, choosing a column for each row in turn (this alone guarantees no
two queens share a row). Track occupied columns and both diagonal directions with three
sets: columns, `row - col` (identifies a "/" diagonal), and `row + col` (identifies a "\"
diagonal) — both diagonal identities are invariant along their diagonal, giving O(1)
conflict checks instead of scanning the board. Backtrack: try a column, recurse to the
next row, then undo before trying the next column.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Row-by-row placement with O(1) column/diagonal
conflict tracking**, which itself belongs to the broader **Backtracking** family of
techniques. If the specific trick above feels like it came out of nowhere, that's the
signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to
recognize this family of problems in general (not just this one), the reusable template
you can write from memory, the usual variations, and the mistakes people make applying
it. Coming back to re-read this problem's approach afterward should make the specific
choices here feel inevitable rather than clever.

## Complexity
- Time: O(n!) worst case, pruned heavily by the conflict checks
- Space: O(n) for the tracking sets + recursion stack

## Solution
Runnable, with sample test cases at the bottom (`python3 backtracking/05_n_queens/solution.py`):

```python
--8<-- "backtracking/05_n_queens/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Decomposition framing (reduce the constraint before coding):** "Placing one queen per
  row automatically satisfies the 'no two queens share a row' constraint for free — that
  lets me focus entirely on choosing a column per row, which shrinks the problem from
  'place n queens anywhere' to 'pick n columns, one per row.'"
- **Invariant framing (why O(1) conflict checks are possible):** "The invariant I lean on
  is that every cell on the same '/' diagonal shares the same `row - col` value, and every
  cell on the same '\\' diagonal shares the same `row + col` value. Tracking those two
  values as sets turns an O(n) board scan per placement into an O(1) lookup."
- **Trade-off framing (naming what the optimization costs):** "I trade a bit of extra
  bookkeeping — three sets instead of zero — for pruning that makes this tractable in
  practice, even though the worst-case bound is still n! before pruning kicks in."

### Vocabulary Builder

- **diagonal identity** (n. phrase) — a derived value (`row - col` or `row + col`) that's
  constant for every cell on a given diagonal; the trick that makes diagonal conflict
  checks O(1) instead of a scan.
- **conflict check** (n. phrase) — testing whether a proposed placement violates a
  constraint against existing placements, ideally in O(1) via precomputed sets.
- **"pruned heavily in practice"** — a fair, honest way to describe an exponential-
  worst-case algorithm that performs far better than its bound suggests on typical inputs.
- **backtrack** (v.) — undoing a placement (removing the column and both diagonal values
  from their tracking sets) after exploring it, so the next candidate column starts clean.
