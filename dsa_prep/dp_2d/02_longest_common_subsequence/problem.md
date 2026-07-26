# 2. Longest Common Subsequence

**Difficulty:** Medium
**Topic:** 2-D Dynamic Programming
**Pattern:** Classic 2-D DP table over two strings

## Problem
Given two strings `text1` and `text2`, return the length of their longest common
subsequence (not necessarily contiguous), or 0 if none exists.

## Examples
```
Input: text1 = "abcde", text2 = "ace" -> 3   ("ace")
Input: text1 = "abc", text2 = "abc"   -> 3
Input: text1 = "abc", text2 = "def"   -> 0
```

## Approach
Build a 2-D table `dp[i][j]` = LCS length of `text1[:i]` and `text2[:j]`. If the last
characters match (`text1[i-1] == text2[j-1]`), extend the LCS found without those
characters: `dp[i][j] = dp[i-1][j-1] + 1`. Otherwise take the best of dropping a character
from either string: `dp[i][j] = max(dp[i-1][j], dp[i][j-1])`. The answer is
`dp[len(text1)][len(text2)]`.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Classic 2-D DP table over two strings**, which
itself belongs to the broader **2-D Dynamic Programming** family of techniques. If the
specific trick above feels like it came out of nowhere, that's the signal to step back
and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of
problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(n·m)
- Space: O(n·m) (reducible to O(min(n,m)) with a rolling array)

## Solution
Runnable, with sample test cases at the bottom (`python3 dp_2d/02_longest_common_subsequence/solution.py`):

```python
--8<-- "dp_2d/02_longest_common_subsequence/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The naive recursion tries, at
  each position, either matching the two current characters or skipping a character from
  one string — that branches into roughly 2^n calls. I'd name that cost out loud, then say
  the overlapping subproblems (same `(i, j)` pair reached multiple ways) are exactly what
  a `dp[i][j]` table eliminates."
- **Invariant framing (good for justifying the two cases precisely):** "`dp[i][j]` means
  'LCS length using only the first `i` characters of text1 and first `j` of text2.' When
  the last characters match, I extend the diagonal answer by one; when they don't, the
  invariant forces me to take the best of dropping a character from either side — I can't
  skip that comparison, because I don't yet know which string 'wastes' the mismatched char."
- **Generalization framing (good for showing this isn't memorized):** "This is the two-string
  comparison DP template — `dp[i][j]` over prefixes, with a match-diagonal case and a
  combine-alternatives case. Edit Distance is the same table shape with a different combine
  function, which is the detail I'd point to if asked how the two problems relate."

### Vocabulary Builder

- **subsequence** (n.) — elements in relative order but not necessarily contiguous;
  contrast with **substring**, which must be contiguous — worth stating explicitly since
  interviewers sometimes probe this distinction directly.
- **prefix** (n.) — the first `i` characters of a string; the natural sub-problem boundary
  for `dp[i][j]` in two-string DP.
- **overlapping subproblems** (n. phrase) — when a recursive call re-derives the same
  sub-answer along multiple recursion paths; the property that makes memoization pay off.
- **"…same template with a different combine function"** — a reusable phrase for
  connecting two DP problems that look different on the surface but share a recurrence
  shape (here, LCS and Edit Distance).
