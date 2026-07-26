# 3. Edit Distance

**Difficulty:** Hard
**Topic:** 2-D Dynamic Programming
**Pattern:** 2-D DP table with insert/delete/replace transitions

## Problem
Given two strings `word1` and `word2`, return the minimum number of single-character
edits (insert, delete, replace) required to transform `word1` into `word2`.

## Examples
```
Input: word1 = "horse", word2 = "ros" -> 3
Input: word1 = "intention", word2 = "execution" -> 5
```

## Approach
`dp[i][j]` = min edits to convert `word1[:i]` into `word2[:j]`. Base cases: converting an
empty prefix into a `j`-length prefix takes `j` insertions (and symmetrically `i`
deletions). If the last characters match, no edit needed there:
`dp[i][j] = dp[i-1][j-1]`. Otherwise take the best of three operations:
`1 + min(dp[i-1][j]` (delete from word1), `dp[i][j-1]` (insert into word1),
`dp[i-1][j-1])` (replace).

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **2-D DP table with insert/delete/replace
transitions**, which itself belongs to the broader **2-D Dynamic Programming** family of
techniques. If the specific trick above feels like it came out of nowhere, that's the
signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to
recognize this family of problems in general (not just this one), the reusable template
you can write from memory, the usual variations, and the mistakes people make applying
it. Coming back to re-read this problem's approach afterward should make the specific
choices here feel inevitable rather than clever.

## Complexity
- Time: O(n·m)
- Space: O(n·m) (reducible to O(min(n,m)))

## Solution
Runnable, with sample test cases at the bottom (`python3 dp_2d/03_edit_distance/solution.py`):

```python
--8<-- "dp_2d/03_edit_distance/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The naive recursion at each
  step tries three operations — insert, delete, replace — and recurses on each, giving an
  exponential branching factor of 3. I'd flag that immediately, then say the fix is the
  same one as always for overlapping subproblems: cache on `(i, j)`, or tabulate bottom-up."
- **Invariant framing (good for making the three-way min defensible):** "`dp[i][j]` is
  'minimum edits to turn the first `i` characters of word1 into the first `j` of word2.'
  When the trailing characters already match, there's nothing to pay for, so
  `dp[i][j] = dp[i-1][j-1]`; otherwise I have to try all three operations and take the
  cheapest, because none of them is provably dominant in general — that's why it's a min
  over three, not a single formula."
- **Generalization framing (good for connecting it to LCS in the same folder):** "This is
  the same two-string prefix-DP template as Longest Common Subsequence, just with a
  different combine function — LCS maximizes a count on mismatch, Edit Distance minimizes
  a cost plus one. Naming that connection is a fast way to show pattern recognition rather
  than memorized solutions."

### Vocabulary Builder

- **Levenshtein distance** (n.) — the formal name for edit distance under
  insert/delete/replace, each costing 1; useful to drop if asked whether you know the
  named version of the problem.
- **base case** (n.) — the boundary condition anchoring the recursion; here, transforming
  an empty prefix into a `j`-length prefix costs exactly `j` insertions.
- **degenerate case** (n. phrase) — an edge case that's trivial but must still be handled
  correctly, such as one string being empty from the start.
- **"the naive approach breaks down when…"** — a reusable phrase for transitioning from
  brute force to DP: "...breaks down when the same subproblem recurs across multiple
  recursive paths, which is exactly what happens here."
