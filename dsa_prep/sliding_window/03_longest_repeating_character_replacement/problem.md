# 3. Longest Repeating Character Replacement

**Difficulty:** Medium
**Topic:** Sliding Window
**Pattern:** Variable window with a validity check

## Problem
Given a string `s` of uppercase letters and an integer `k`, you may replace up to `k`
characters in the string. Return the length of the longest substring achievable that
contains only one repeating character after those replacements.

## Examples
```
Input: s = "ABAB", k = 2  -> 4
Input: s = "AABABBA", k = 1 -> 4
```

## Approach
Slide a window over `s`, keeping a count of each letter within the window and tracking
`max_freq` — the count of the most frequent letter in the current window. A window is
valid if `(window length - max_freq) <= k`, i.e. the number of characters that would need
replacing doesn't exceed `k`. When the window becomes invalid, shrink from the left. The
window size never needs to shrink below its historical max, so the answer is simply the
largest window length ever reached.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Variable window with a validity check**, which
itself belongs to the broader **Sliding Window** family of techniques. If the specific
trick above feels like it came out of nowhere, that's the signal to step back and read
[`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of problems in
general (not just this one), the reusable template you can write from memory, the usual
variations, and the mistakes people make applying it. Coming back to re-read this
problem's approach afterward should make the specific choices here feel inevitable
rather than clever.

## Complexity
- Time: O(n) (26-letter alphabet, effectively O(26n) = O(n))
- Space: O(1) (fixed 26-letter count array)

## Solution
Runnable, with sample test cases at the bottom (`python3 sliding_window/03_longest_repeating_character_replacement/solution.py`):

```python
--8<-- "sliding_window/03_longest_repeating_character_replacement/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The brute force checks every
  substring, counts letters, and verifies `length - max_freq <= k` from scratch each
  time — O(n²) or worse. I'd name that, then say I can maintain the count and `max_freq`
  incrementally as the window slides, getting to O(n)."
- **Invariant framing (good for explaining the non-shrinking window, which trips people
  up):** "The subtlety here is the window never needs to shrink below its historical max
  size — once I've achieved a window of size W, sliding both edges forward together
  keeps checking whether size W+1 is achievable, without ever needing to fall back below
  W. I'd say that explicitly, since it's the part that looks wrong at first glance."
- **Generalization framing (good for signaling pattern recognition):** "This is the
  'variable window with a validity check' shape, where validity comes from a derived
  quantity — `max_freq` — instead of a simple boolean. I'd mention that's the same family
  as Minimum Window Substring, just with a different validity condition."

### Vocabulary Builder

- **validity check** (n. phrase) — the condition a window must satisfy to count as a
  candidate answer; here, `window_length - max_freq <= k`.
- **derived quantity** (n. phrase) — a value computed from other tracked state rather than
  stored directly; `max_freq` is derived from the count array, not maintained independently
  from scratch each time.
- **"the crux of it is…"** — useful for isolating the one subtle design decision an
  interviewer is probing for, here: *"the crux of it is that `max_freq` never needs to
  decrease even when the window slides past its peak."*
- **monotonic** (adj.) — non-decreasing here: the answer (best window length) only ever
  grows or stays the same as the scan proceeds, never needs revisiting downward.
