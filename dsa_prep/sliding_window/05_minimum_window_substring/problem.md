# 5. Minimum Window Substring

**Difficulty:** Hard
**Topic:** Sliding Window
**Pattern:** Variable window with a "have vs need" counter

## Problem
Given strings `s` and `t`, return the minimum-length substring of `s` that contains every
character of `t` (including duplicates). Return `""` if no such substring exists.

## Examples
```
Input: s = "ADOBECODEBANC", t = "ABC" -> "BANC"
Input: s = "a", t = "aa"              -> ""
```

## Approach
Build a frequency map `need` for `t`. Expand a right pointer over `s`, maintaining a
`window` frequency map and a counter `have` of how many distinct characters currently meet
their required count in `need`. Once `have == len(need)` (window contains everything
needed), try shrinking from the left as much as possible while it stays valid, recording
the best (smallest) valid window found. This is the standard "grow to find validity,
shrink to minimize" sliding window template.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Variable window with a "have vs need"
counter**, which itself belongs to the broader **Sliding Window** family of techniques.
If the specific trick above feels like it came out of nowhere, that's the signal to step
back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family
of problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(n + m) where n = len(s), m = len(t)
- Space: O(n + m)

## Solution
Runnable, with sample test cases at the bottom (`python3 sliding_window/05_minimum_window_substring/solution.py`):

```python
--8<-- "sliding_window/05_minimum_window_substring/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The brute force checks every
  substring of `s` and verifies it contains all of `t` — O(n² · m) or worse depending on
  the verification cost. I'd name that, then pivot to sliding window since 'contains all
  of `t`' is a property that changes predictably as the window grows or shrinks by one
  character."
- **Invariant framing (good for explaining the have/need mechanic precisely):** "The
  invariant is: `have` always equals the number of distinct required characters whose
  count in the window currently meets or exceeds what `need` demands. That turns an O(26)
  or O(distinct chars) recheck into an O(1) increment/decrement — I only update `have`
  when a count crosses a threshold, not on every single character."
- **Generalization framing (good for signaling pattern recognition):** "This is the
  'grow until valid, then shrink to minimize' shape of sliding window — the inverse of
  the more common 'grow, and shrink while invalid' template — I'd name both shapes to
  show I know this is a variant, not a special case."

### Vocabulary Builder

- **have/need counters** (n. phrase) — a pair of trackers comparing how many required
  conditions are currently satisfied versus how many are needed; turns a full-map
  comparison into an O(1) check.
- **threshold** (n.) — the exact count a character must reach to satisfy its requirement;
  crossing it is what should trigger updating `have`, not the raw counts changing.
- **"grow until valid, then shrink to minimize"** — a reusable phrase distinguishing this
  window shape from the more common "grow, shrink while invalid" template used elsewhere
  in this folder.
- **greedily** (adv.) — making the locally best choice at each step without
  backtracking; the left-edge shrink here is greedy — it keeps removing characters as
  long as the window stays valid, trusting that's optimal.
