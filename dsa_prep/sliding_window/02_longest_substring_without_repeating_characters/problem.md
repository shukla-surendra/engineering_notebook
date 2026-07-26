# 2. Longest Substring Without Repeating Characters

**Difficulty:** Medium
**Topic:** Sliding Window
**Pattern:** Variable-size window + hash map of last-seen index

## Problem
Given a string `s`, find the length of the longest substring without repeating
characters.

## Examples
```
Input: s = "abcabcbb" -> 3  ("abc")
Input: s = "bbbbb"    -> 1  ("b")
Input: s = "pwwkew"   -> 3  ("wke")
```

## Approach
Expand a window's right edge one character at a time. Keep a hash map of each character's
most recent index. If the current character was seen before *and* its last index is
inside the current window, jump the left edge to just past that previous occurrence.
Track the max window size (`right - left + 1`) throughout.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Variable-size window + hash map of last-seen
index**, which itself belongs to the broader **Sliding Window** family of techniques. If
the specific trick above feels like it came out of nowhere, that's the signal to step
back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family
of problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(n)
- Space: O(min(n, alphabet size))

## Solution
Runnable, with sample test cases at the bottom (`python3 sliding_window/02_longest_substring_without_repeating_characters/solution.py`):

```python
--8<-- "sliding_window/02_longest_substring_without_repeating_characters/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The naive way checks every
  substring for uniqueness — O(n³) if you re-verify each one from scratch, or O(n²) with
  a smarter check. I'd name that, then say a sliding window gets to O(n) because most of
  the work checking one window is still valid for the next — I don't need to re-verify
  from scratch."
- **Invariant framing (good for explaining the jump-the-left-edge trick precisely):** "The
  invariant is that `[left, right]` never contains a duplicate. When I see a repeat, I
  don't shrink one step at a time — I jump `left` directly past the previous occurrence,
  but only if that occurrence is still inside the current window. Checking that condition
  is exactly what stops `left` from ever moving backward."
- **Generalization framing (good for signaling pattern recognition):** "This is the
  variable-size window template from the sliding-window family, specialized with a
  hash map of last-seen index instead of a frequency count. I'd point out the same
  amortized-O(n) argument applies: `left` only ever moves forward."

### Vocabulary Builder

- **amortized** (adj.) — a cost measured over the whole run rather than any single step;
  the `while`-loop shrink here looks expensive per-iteration but is O(n) in total because
  `left` never backtracks.
- **last-seen index** (n. phrase) — a hash map from value to its most recent position,
  used to jump a pointer directly rather than stepping one element at a time.
- **"trades a rescan for a lookup"** — a reusable phrase for describing how a hash map
  replaces an inner loop with an O(1) check.
- **monotonic** (adj.) — describing a pointer or quantity that moves in only one
  direction; `left` here is monotonic, which is precisely what makes the total work
  linear instead of quadratic.
