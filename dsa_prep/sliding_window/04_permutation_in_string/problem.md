# 4. Permutation in String

**Difficulty:** Medium
**Topic:** Sliding Window
**Pattern:** Fixed-size window with a frequency-count comparison

## Problem
Given strings `s1` and `s2`, return `True` if `s2` contains a permutation of `s1` as a
contiguous substring.

## Examples
```
Input: s1 = "ab", s2 = "eidbaooo" -> True  ("ba" is a permutation of "ab")
Input: s1 = "ab", s2 = "eidboaoo" -> False
```

## Approach
A permutation of `s1` occurring in `s2` is just a fixed-size window of length `len(s1)`
whose letter-count matches `s1`'s letter-count exactly. Slide a window of that fixed size
across `s2`, maintaining a running count array, incrementing on the new right character and
decrementing on the character leaving the left. Compare count arrays (or track a
"matches" counter to avoid O(26) comparisons each step).

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Fixed-size window with a frequency-count
comparison**, which itself belongs to the broader **Sliding Window** family of
techniques. If the specific trick above feels like it came out of nowhere, that's the
signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to
recognize this family of problems in general (not just this one), the reusable template
you can write from memory, the usual variations, and the mistakes people make applying
it. Coming back to re-read this problem's approach afterward should make the specific
choices here feel inevitable rather than clever.

## Complexity
- Time: O(n) where n = len(s2)
- Space: O(1) (26-letter count arrays)

## Solution
Runnable, with sample test cases at the bottom (`python3 sliding_window/04_permutation_in_string/solution.py`):

```python
--8<-- "sliding_window/04_permutation_in_string/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The naive approach generates
  every permutation of `s1` and searches for each in `s2` — factorial time, clearly a
  non-starter. I'd immediately reframe: a permutation is just 'same letters, same
  counts,' so this becomes a fixed-size window comparison instead of a combinatorics
  problem, which is O(n)."
- **Invariant framing (good for explaining the fixed-size window precisely):** "The
  invariant is: the window is always exactly `len(s1)` characters wide. Each slide adds
  one character on the right and removes exactly one on the left, so I maintain the
  count array incrementally rather than recomputing it — and I track a single 'matches'
  counter instead of comparing two 26-length arrays every step."
- **Generalization framing (good for signaling pattern recognition):** "This is the
  fixed-size-window variant of the sliding-window family — contrast it with the variable-
  size windows in Longest Substring or Minimum Window Substring, where the window's size
  itself is what you're solving for."

### Vocabulary Builder

- **fixed-size window** (n. phrase) — a sliding window whose length is constant
  throughout, as opposed to one that grows/shrinks to satisfy a condition.
- **frequency count** (n. phrase) — an array or map tracking how many times each
  character appears; comparing two frequency counts is how you check "same letters,
  regardless of order."
- **"reframe the problem as…"** — a reusable phrase for the moment you notice a
  combinatorial-looking problem ("check every permutation") is secretly a simpler
  structural check ("same character counts").
- **amortized** (adj.) — describing the O(1) per-step cost of the matches-counter trick,
  which avoids an O(26) full-array comparison at every window position.
