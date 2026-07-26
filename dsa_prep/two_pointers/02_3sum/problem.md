# 2. 3Sum

**Difficulty:** Medium
**Topic:** Two Pointers
**Pattern:** Sort + fix one element + two pointers

## Problem
Given an integer array `nums`, return all unique triplets `[nums[i], nums[j], nums[k]]`
such that `i != j != k` and they sum to zero. The solution set must not contain duplicate
triplets.

## Examples
```
Input: nums = [-1,0,1,2,-1,-4] -> [[-1,-1,2],[-1,0,1]]
Input: nums = [0,1,1]          -> []
Input: nums = [0,0,0]          -> [[0,0,0]]
```

## Approach
Sort the array first. Fix each index `i` as a candidate first element, then use two
pointers (`left = i+1`, `right = n-1`) on the remaining sorted subarray to find pairs that
sum to `-nums[i]`, exactly like the classic "two sum on a sorted array" pattern. Skip
duplicate values for `i`, and after finding a valid triplet, skip duplicate `left`/`right`
values to avoid duplicate triplets in the output.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Sort + fix one element + two pointers**, which
itself belongs to the broader **Two Pointers** family of techniques. If the specific
trick above feels like it came out of nowhere, that's the signal to step back and read
[`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of problems in
general (not just this one), the reusable template you can write from memory, the usual
variations, and the mistakes people make applying it. Coming back to re-read this
problem's approach afterward should make the specific choices here feel inevitable
rather than clever.

## Complexity
- Time: O(n²)
- Space: O(1) extra (excluding output; O(n) if sort isn't in-place)

## Solution
Runnable, with sample test cases at the bottom (`python3 two_pointers/02_3sum/solution.py`):

```python
--8<-- "two_pointers/02_3sum/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The brute force is three
  nested loops checking every triplet — O(n³). I'd name that, then say the first real
  optimization is sorting, which turns the innermost loop into a two-pointer scan instead
  of a third loop, bringing it to O(n log n) + O(n²) overall — sorting isn't the whole
  optimization, it's what *enables* the two-pointer step."
- **Invariant framing (good for explaining the duplicate-skipping precisely):** "The
  invariant I have to protect is 'no duplicate triplets in the output,' and duplicates
  don't come from one place — they can come from repeating `i`, or from repeating `left`/
  `right` after a match. So I skip duplicates in three separate spots, not one, and I'd
  say that out loud because it's the detail most people get partially right."
- **Generalization framing (good for connecting to the underlying pattern):** "This is
  'fix one element, reduce to the two-pointer version of the problem on what's left' —
  Two Sum's converging-pointer idea nested inside a loop. I'd name that reduction
  explicitly, since it's the reusable trick for turning k-Sum problems into (k-1)-Sum ones
  recursively."

### Vocabulary Builder

- **reduction** (n.) — solving a new problem by transforming it into an already-solved
  one; here, 3Sum reduces to "two-sum on a sorted subarray" once one element is fixed.
- **monotonic** (adj.) — consistently increasing or decreasing; the sorted array's
  monotonic order is exactly what lets the two-pointer scan decide which side to move
  without backtracking.
- **"…sorting isn't the optimization, it's what enables the optimization"** — a precise
  phrase for explaining that the O(n log n) sort's value lies in unlocking a cheaper
  subsequent step, not in speeding up the problem by itself.
- **degenerate case** (n. phrase) — here, an array of all zeros (`[0,0,0]`) — a good input
  to mention aloud, since it stress-tests both the duplicate-skipping logic and the case
  where a single value satisfies the target sum on its own.
