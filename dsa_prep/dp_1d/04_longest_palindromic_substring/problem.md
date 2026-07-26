# 4. Longest Palindromic Substring

**Difficulty:** Medium
**Topic:** 1-D Dynamic Programming
**Pattern:** Expand around center

## Problem
Given a string `s`, return the longest palindromic substring.

## Examples
```
Input: s = "babad" -> "bab" (or "aba", both valid)
Input: s = "cbbd"  -> "bb"
```

## Approach
Every palindrome has a center — either a single character (odd length) or a gap between
two characters (even length). For each of the `2n - 1` possible centers, expand outward
while both sides match, tracking the longest palindrome found. This avoids the O(n³) brute
force of checking every substring, and is simpler to implement correctly than the O(n) DP
table approach while still being efficient enough (O(n²)).

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Expand around center**, which itself belongs to
the broader **1-D Dynamic Programming** family of techniques. If the specific trick
above feels like it came out of nowhere, that's the signal to step back and read
[`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of problems in
general (not just this one), the reusable template you can write from memory, the usual
variations, and the mistakes people make applying it. Coming back to re-read this
problem's approach afterward should make the specific choices here feel inevitable
rather than clever.

## Complexity
- Time: O(n²)
- Space: O(1)

## Solution
Runnable, with sample test cases at the bottom (`python3 dp_1d/04_longest_palindromic_substring/solution.py`):

```python
--8<-- "dp_1d/04_longest_palindromic_substring/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The true brute force checks
  every substring for being a palindrome — O(n³). I'd name that, then note that a lot of
  that work is redundant: a palindrome's identity is fully determined by its center, which
  is the opening for expand-around-center."
- **Invariant framing (good for explaining the expansion loop precisely):** "The invariant
  each expansion step maintains is: 'the substring between the current left and right
  pointers is a palindrome.' The moment `s[left] != s[right]`, that invariant would break, so
  I stop immediately — that's why the loop condition checks equality before advancing."
- **Pattern-recognition framing (good for connecting to a simpler DP table alternative):**
  "I'd mention this is one of two standard approaches — expand-around-center or a 2-D `is
  palindrome` DP table — and that I'm choosing the center approach because it's O(1) extra
  space and easier to get right under pressure, even though both are O(n²) time."

### Vocabulary Builder

- **center** (n.) — for palindromes, either a single character (odd length) or the gap
  between two characters (even length); enumerating `2n - 1` centers is what makes the
  approach exhaustive without being O(n³).
- **monotonic** (adj.) — not directly used here, but worth contrasting: expand-around-center
  is *not* monotonic in the sense two-pointer problems often are — expansion can stop at any
  point, it's not a one-directional sweep.
- **"…trades an explicit DP table for a simpler O(n²) sweep"** — a reusable phrase for
  justifying expand-around-center over the table-based DP formulation of the same problem.
- **odd/even-length symmetry** (n. phrase) — the two cases a palindrome center-expansion
  must handle separately; forgetting the even case is a common bug.
