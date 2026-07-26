# 5. Missing Number

**Difficulty:** Easy
**Topic:** Bit Manipulation
**Pattern:** XOR cancellation over indices and values (or sum formula)

## Problem
Given an array `nums` containing `n` distinct numbers from the range `[0, n]`, return the
one number in that range missing from the array.

## Examples
```
Input: nums = [3,0,1] -> 2
Input: nums = [9,6,4,2,3,5,7,0,1] -> 8
```

## Approach
XOR every index `0..n` together with every value in `nums`. Every number that's actually
present cancels with its matching index (since indices only run 0..n-1 but values cover
0..n, exactly one value — the missing one — has no index partner and survives the
XOR). Equivalently, `expected_sum(0..n) - actual_sum(nums)` gives the same answer using
arithmetic instead of XOR — both are O(n) time, O(1) space, and avoid the O(n log n) sort
or O(n) hash-set approaches.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **XOR cancellation over indices and values (or
sum formula)**, which itself belongs to the broader **Bit Manipulation** family of
techniques. If the specific trick above feels like it came out of nowhere, that's the
signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to
recognize this family of problems in general (not just this one), the reusable template
you can write from memory, the usual variations, and the mistakes people make applying
it. Coming back to re-read this problem's approach afterward should make the specific
choices here feel inevitable rather than clever.

## Complexity
- Time: O(n)
- Space: O(1)

## Solution
Runnable, with sample test cases at the bottom (`python3 bit_manipulation/05_missing_number/solution.py`):

```python
--8<-- "bit_manipulation/05_missing_number/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "Sorting and scanning for a
  gap is O(n log n); a hash set of expected values is O(n) time but O(n) extra space. I'd
  mention both before landing on XOR or the sum formula, since naming two alternatives
  shows I actually compared trade-offs rather than jumping straight to the trick."
- **Invariant framing (good for explaining why XOR-ing indices *and* values works):** "The
  invariant is that every present value cancels exactly once against its matching index,
  because indices run 0..n-1 and values run over 0..n minus one missing entry — so
  whichever value has no index partner is what survives the XOR."
- **Pattern-recognition framing (good for showing you know there are two valid tools):**
  "I'd flag out loud that this is solvable two ways — XOR cancellation or
  `expected_sum - actual_sum` — and that they're interchangeable here; picking XOR avoids
  any concern about integer overflow in the sum, which matters more in fixed-width
  languages than in Python."

### Vocabulary Builder

- **closed range** (n. phrase) — `[0, n]` inclusive on both ends; precisely stating the
  range up front avoids an off-by-one when pairing indices to values.
- **arithmetic series** (n.) — the sum formula `n(n+1)/2` underlying the subtraction
  alternative to XOR; useful to have in your back pocket as a second approach.
- **"…both are O(n) time, O(1) space — the choice is really about robustness to overflow"**
  — a reusable way to compare two equally valid approaches instead of presenting only one.
- **unpaired** (adj.) — the element left without a match after cancellation; precise
  vocabulary for describing what XOR-ing indices against values actually isolates.
