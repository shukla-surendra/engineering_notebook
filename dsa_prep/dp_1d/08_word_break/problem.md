# 8. Word Break

**Difficulty:** Medium
**Topic:** 1-D Dynamic Programming
**Pattern:** Bottom-up reachability DP over string prefixes

## Problem
Given a string `s` and a dictionary `wordDict`, return `True` if `s` can be segmented into
a space-separated sequence of one or more dictionary words. Words may be reused.

## Examples
```
Input: s = "leetcode", wordDict = ["leet","code"] -> True
Input: s = "catsandog", wordDict = ["cats","dog","sand","and","cat"] -> False
```

## Approach
Let `dp[i]` mean "`s[:i]` can be fully segmented using dictionary words". `dp[0] = True`
(empty prefix trivially works). For each `i`, check every `j < i` where `dp[j]` is true
and `s[j:i]` is in the dictionary — if found, `dp[i] = True`. The final answer is
`dp[len(s)]`. Using a set for `wordDict` keeps substring lookups O(1).

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Bottom-up reachability DP over string
prefixes**, which itself belongs to the broader **1-D Dynamic Programming** family of
techniques. If the specific trick above feels like it came out of nowhere, that's the
signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to
recognize this family of problems in general (not just this one), the reusable template
you can write from memory, the usual variations, and the mistakes people make applying
it. Coming back to re-read this problem's approach afterward should make the specific
choices here feel inevitable rather than clever.

## Complexity
- Time: O(n²) (or O(n² · maxWordLen) depending on substring slicing cost)
- Space: O(n)

## Solution
Runnable, with sample test cases at the bottom (`python3 dp_1d/08_word_break/solution.py`):

```python
--8<-- "dp_1d/08_word_break/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The brute force recursively
  tries every split point — exponential, and it re-checks whether the same suffix is
  segmentable over and over across different branches. I'd name that repetition explicitly,
  since it's the tell for a reachability DP over prefixes."
- **Invariant framing (good for explaining what `dp[i]` actually means):** "The invariant is
  precise: `dp[i]` is true exactly when `s[:i]` — the first `i` characters — can be fully
  segmented. I'd say that definition out loud before coding, since 'first i characters' vs.
  'characters ending at i' is an easy thing to conflate and it changes the indexing."
- **Pattern-recognition framing (good for distinguishing this from count/optimize DP):** "This
  is a *boolean* reachability DP, not a min/max one — I'd flag that distinction, since it
  changes the recurrence from combining numeric values to combining truth values with OR."

### Vocabulary Builder

- **reachability** (n.) — whether a state can be reached at all, as opposed to optimizing
  a cost; precise term for what this DP computes (segmentable or not) versus, say, Coin
  Change's minimization.
- **prefix** (n.) — `s[:i]`, the first `i` characters of the string; stating "prefix" out
  loud instead of re-describing the slice each time keeps the explanation fluent.
- **"…the naive approach breaks down when the same suffix gets re-validated across
  branches"** — a reusable phrase for justifying memoized/bottom-up DP over recursive
  brute force.
- **amortized** (adj.) — using a set for `wordDict` amortizes substring membership checks
  to O(1) each, which matters since the DP already does O(n²) substring slices.
