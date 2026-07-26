# Pattern: 2-D Dynamic Programming

## What problem does this solve?

2-D DP is what you reach for when the state needs **two** independent indices to describe
it — most commonly "a position in each of two strings" (comparing/transforming two
sequences) or "a position in a grid" (row and column). The core DP discipline (define the
state, find the recurrence, identify base cases) is identical to 1-D DP — see
`../dp_1d/PATTERN.md` for that process in detail — the only new wrinkle is that the
recurrence now reads from up to three neighboring cells instead of one or two prior values.

## How to recognize it

Signals that 2-D DP applies:
- The problem involves **two strings/sequences** and asks about a relationship between
  them ("longest common subsequence," "minimum edits to transform one into the other") —
  the natural state is `dp[i][j]` = answer considering the first `i` characters of one and
  the first `j` of the other.
- The problem involves a **grid** and asks "how many paths," "min/max cost path," or
  "reachability" — the natural state is `dp[row][col]`.
- You'd need two nested indices to even describe "which subproblem am I solving" in
  words — that's the tell it's 2-D, not 1-D.

## The general template

**Two-string comparison DP (most common shape):**
```python
n, m = len(s1), len(s2)
dp = [[base_value] * (m + 1) for _ in range(n + 1)]

# base cases: dp[i][0] and dp[0][j] usually represent "one string is empty"
for i in range(n + 1): dp[i][0] = base_case_for_row(i)
for j in range(m + 1): dp[0][j] = base_case_for_col(j)

for i in range(1, n + 1):
    for j in range(1, m + 1):
        if s1[i-1] == s2[j-1]:
            dp[i][j] = extend(dp[i-1][j-1])              # characters match — extend the diagonal
        else:
            dp[i][j] = combine(dp[i-1][j], dp[i][j-1], dp[i-1][j-1])  # they don't — try alternatives
return dp[n][m]
```
This single shape covers Longest Common Subsequence (`extend` = `+1`, `combine` = `max`)
and Edit Distance (`extend` = "no-op, same cost as `dp[i-1][j-1]`", `combine` = `1 + min(...)`
over insert/delete/replace) — recognizing they're the *same template* with different
combine functions is the key insight.

**Grid DP:**
```python
dp = [[0] * cols for _ in range(rows)]
dp[0][0] = base_value
for r in range(rows):
    for c in range(cols):
        if (r, c) == (0, 0): continue
        from_above = dp[r-1][c] if r > 0 else invalid_value
        from_left  = dp[r][c-1] if c > 0 else invalid_value
        dp[r][c] = combine(from_above, from_left, grid[r][c])
return dp[rows-1][cols-1]
```
This is Unique Paths (`combine` = sum, since you're counting ways) — and generalizes
directly to "minimum path sum" style problems (`combine` = `min(...) + cost`).

**0/1 Knapsack (item-by-item, capacity-by-capacity):**
```python
dp = [[0] * (capacity + 1) for _ in range(n + 1)]
for i in range(1, n + 1):
    weight, value = weights[i-1], values[i-1]
    for w in range(capacity + 1):
        dp[i][w] = dp[i-1][w]                              # option 1: skip item i
        if weight <= w:
            dp[i][w] = max(dp[i][w], value + dp[i-1][w - weight])  # option 2: take item i
return dp[n][capacity]
```
The critical detail that distinguishes this from Coin Change's *unbounded* knapsack: each
row only reads from the *previous* row (`dp[i-1][...]`), never the current row, which is
exactly what enforces "each item usable at most once."

## Common pitfalls

- Confusing bounded (0/1, each item once — read from the previous row only) with unbounded
  (unlimited reuse — read from the current row/same item's earlier state) knapsack
  variants; mixing them up silently allows/disallows reuse incorrectly.
- Off-by-one between the DP table (padded with an extra row/column for the empty-prefix
  base case, size `(n+1) x (m+1)`) and the original strings/grid — always double check
  whether you're indexing `s[i-1]` or `s[i]` inside the loop.
- Not row/column-reducing when memory matters: most 2-D DP tables only ever read from the
  *immediately previous* row, so a rolling 1-D array can replace the full 2-D table when
  space is a concern (as done in Unique Paths).

## Complexity characteristics

O(n·m) time and space for the full table; often reducible to O(min(n,m)) space with a
rolling array, since most recurrences only look at the previous row.

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Recognition framing (how you'd name the pattern out loud, fast):** "The moment I
  notice I need *two* indices to describe 'which subproblem am I solving' — a position in
  each of two strings, or a row and a column in a grid — I say '2-D DP' and reach for a
  `dp[i][j]` table instead of trying to force a 1-D array to do double duty."
- **Mental-model framing (good for explaining the unifying structure across variants):**
  "I think of 2-D DP as three canonical shapes: two-string comparison (diagonal-extend on
  match, combine-alternatives on mismatch), grid traversal (combine the cell above and the
  cell to the left), and item-by-item knapsack (skip-or-take, reading only the previous
  row). Naming which shape I'm in tells me the recurrence before I've derived it."
- **Generalization/teaching framing (good for demonstrating depth beyond one problem):**
  "The deepest insight isn't any single recurrence — it's that Longest Common Subsequence
  and Edit Distance are *literally the same template* with different `extend`/`combine`
  functions. I'd walk an interviewer through that unification to show I understand the
  family, not just two isolated solutions I memorized."

### Vocabulary Builder

- **state space** (n.) — the set of all `(i, j)` pairs the DP table could represent;
  thinking in terms of state space is what makes "how many indices do I need" a
  mechanical recognition question rather than a guess.
- **memoization vs. tabulation** (n. phrase) — top-down caching of recursive calls
  (memoization) versus bottom-up iterative filling of the table (tabulation); both compute
  the same values, but tabulation avoids recursion-depth limits and is usually what you'd
  write once the recurrence is nailed down.
- **monotonic** (adj.) — non-decreasing or non-increasing as an index advances; not every
  2-D DP table is monotonic, but checking whether one is can unlock further space
  optimizations (or rule out a proposed shortcut).
- **"…read from the previous row only"** — the diagnostic phrase for bounded/bookkeeping
  correctness: if your recurrence for row `i` never touches row `i` itself, you've encoded
  "used at most once"; if it does, you've encoded "reusable."
