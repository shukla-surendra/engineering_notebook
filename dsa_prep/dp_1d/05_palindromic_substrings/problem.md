# 5. Palindromic Substrings

**Difficulty:** Medium
**Topic:** 1-D Dynamic Programming
**Pattern:** Expand around center, counting instead of tracking longest

## Problem
Given a string `s`, return the number of palindromic substrings (different positions
counted separately even if the substrings have the same characters).

## Examples
```
Input: s = "abc" -> 3   ("a","b","c")
Input: s = "aaa" -> 6   ("a","a","a","aa","aa","aaa")
```

## Approach
Same "expand around center" technique as Longest Palindromic Substring, but every
successful expansion step is itself a valid palindrome, so just increment a counter each
time the expansion condition holds, for both odd-length and even-length centers, across
all `2n - 1` centers.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Expand around center, counting instead of
tracking longest**, which itself belongs to the broader **1-D Dynamic Programming**
family of techniques. If the specific trick above feels like it came out of nowhere,
that's the signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it covers how
to recognize this family of problems in general (not just this one), the reusable
template you can write from memory, the usual variations, and the mistakes people make
applying it. Coming back to re-read this problem's approach afterward should make the
specific choices here feel inevitable rather than clever.

## Complexity
- Time: O(n²)
- Space: O(1)

## Solution
Runnable, with sample test cases at the bottom (`python3 dp_1d/05_palindromic_substrings/solution.py`):

```python
--8<-- "dp_1d/05_palindromic_substrings/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "Checking every one of the O(n²)
  substrings individually for being a palindrome costs O(n³) total. I'd name that, then
  point out this is nearly identical machinery to Longest Palindromic Substring — same
  centers, different bookkeeping."
- **Invariant framing (good for explaining why counting instead of tracking-longest is
  simpler):** "The invariant is: every time the expansion condition holds — `s[left] ==
  s[right]` — that span is, by definition, a palindrome, so I just increment a counter right
  there. There's no need to compare against a running maximum like the 'longest' version
  requires."
- **Pattern-recognition framing (good for showing you see the relationship between the two
  problems):** "I'd explicitly connect this to problem 4 — same expand-around-center
  traversal, but the aggregation changes from 'track the max span seen' to 'count every
  valid span.' Recognizing that the traversal is reusable and only the aggregation differs
  is the generalizable insight."

### Vocabulary Builder

- **aggregation** (n.) — how individual results are combined into a final answer (count vs.
  max here); useful vocabulary for describing what changes between two structurally
  identical algorithms.
- **exhaustive** (adj.) — covering every possible case without omission; the `2n - 1`
  centers are exhaustive over all possible palindrome midpoints, both odd and even length.
- **"…same traversal, different aggregation"** — a precise, reusable phrase for describing
  problems that share a scanning strategy but differ in what they compute from each step.
- **double-counting** (n.) — a bug risk worth naming: confirming odd- and even-length
  centers are counted from genuinely distinct starting points, not overlapping ranges.
