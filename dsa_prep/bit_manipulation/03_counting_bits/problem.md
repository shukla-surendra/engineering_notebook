# 3. Counting Bits

**Difficulty:** Easy
**Topic:** Bit Manipulation
**Pattern:** DP building on `i >> 1` (reuse of the previous answer)

## Problem
Given an integer `n`, return an array `ans` of length `n+1` where `ans[i]` is the number
of 1 bits in the binary representation of `i`, for every `i` from 0 to `n`. Aim for a
linear-time solution.

## Examples
```
Input: n = 2 -> [0,1,1]
Input: n = 5 -> [0,1,1,2,1,2]
```

## Approach
Recomputing Hamming weight from scratch for every `i` costs O(n log n) overall. Instead,
build a DP: `ans[i] = ans[i >> 1] + (i & 1)`. Right-shifting `i` by 1 drops its lowest bit,
so its bit count is already known from an earlier (smaller) index; just add back 1 if the
dropped bit was a 1 (`i & 1`). This computes every answer in O(1) using previously
computed results.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **DP building on `i >> 1` (reuse of the previous
answer)**, which itself belongs to the broader **Bit Manipulation** family of
techniques. If the specific trick above feels like it came out of nowhere, that's the
signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to
recognize this family of problems in general (not just this one), the reusable template
you can write from memory, the usual variations, and the mistakes people make applying
it. Coming back to re-read this problem's approach afterward should make the specific
choices here feel inevitable rather than clever.

## Complexity
- Time: O(n)
- Space: O(n) for the output (O(1) extra)

## Solution
Runnable, with sample test cases at the bottom (`python3 bit_manipulation/03_counting_bits/solution.py`):

```python
--8<-- "bit_manipulation/03_counting_bits/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "Running Kernighan's trick or
  a naive bit-check independently for every `i` from 0 to `n` gives O(n log n) overall —
  I'd state that total before pivoting, since 'linear time' in the prompt is the signal
  that I should reuse work across iterations instead of recomputing."
- **Invariant framing (good for explaining the recurrence precisely):** "The invariant is
  that `ans[j]` is already correct for every `j < i` by the time I compute `ans[i]`. That
  lets me write `ans[i] = ans[i >> 1] + (i & 1)` and trust the smaller answer instead of
  recomputing it — it's DP dressed up as a bit trick."
- **Pattern-recognition framing (good for placing this in the DP family):** "I'd name this
  explicitly as 1-D DP where the recurrence happens to come from a bitwise observation
  rather than an array of costs — same shape as Climbing Stairs, different source for the
  transition."

### Vocabulary Builder

- **memoization** (n.) — caching subproblem results so they're computed once; here it's
  implicit in the array itself rather than an explicit cache.
- **recurrence** (n.) — the rule relating `ans[i]` to earlier entries; stating it in words
  before code ("shift right, drop the low bit, add it back if it was 1") shows you derived
  it rather than memorized it.
- **"…trades recomputation for reuse"** — a compact phrase for describing any DP speedup
  over independent per-element brute force.
- **bottom-up** (adj.) — building the table from the smallest index upward, as opposed to
  top-down recursion with memoization; worth naming which direction you're building in.
