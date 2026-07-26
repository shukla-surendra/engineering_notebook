# 1. Single Number

**Difficulty:** Easy
**Topic:** Bit Manipulation
**Pattern:** XOR cancellation

## Problem
Given a non-empty array `nums` where every element appears exactly twice except for one,
find that single element. Must run in linear time with O(1) extra space.

## Examples
```
Input: nums = [2,2,1]     -> 1
Input: nums = [4,1,2,1,2] -> 4
```

## Approach
XOR is commutative, associative, `x ^ x = 0`, and `x ^ 0 = x`. XOR-ing every element
together cancels out every pair (they XOR to 0), leaving only the single unpaired element.
This rules out needing a hash set (which would use O(n) space).

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **XOR cancellation**, which itself belongs to the
broader **Bit Manipulation** family of techniques. If the specific trick above feels
like it came out of nowhere, that's the signal to step back and read
[`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of problems in
general (not just this one), the reusable template you can write from memory, the usual
variations, and the mistakes people make applying it. Coming back to re-read this
problem's approach afterward should make the specific choices here feel inevitable
rather than clever.

## Complexity
- Time: O(n)
- Space: O(1)

## Solution
Runnable, with sample test cases at the bottom (`python3 bit_manipulation/01_single_number/solution.py`):

```python
--8<-- "bit_manipulation/01_single_number/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The naive approach is a hash
  set — track what I've seen, O(n) space. I'd say that out loud first, then note the
  constraint 'O(1) space' is the tell that they want XOR: it lets me cancel pairs without
  storing anything."
- **Invariant framing (good for justifying why the trick is correct, not just fast):** "The
  invariant is that `result` always holds the XOR of everything processed so far. Since
  XOR is commutative and associative, order never breaks that invariant — which is exactly
  why I don't need to sort or group first."
- **Pattern-recognition framing (good for signaling this generalizes):** "This is the base
  case of XOR cancellation — anytime I hear 'everything appears twice except one,' that's
  my cue. I'd name the family, since Missing Number is the same identity applied over
  indices instead of just values."

### Vocabulary Builder

- **cancellation** (n.) — here, the algebraic fact that `x ^ x = 0` makes paired elements
  vanish under XOR. *"XOR cancellation is what lets duplicates zero themselves out."*
- **commutative / associative** (adj.) — order and grouping don't affect the result;
  worth stating explicitly since it's *why* a single linear pass suffices.
- **"the naive approach breaks down when…"** — a reusable phrase for pivoting from brute
  force to optimized: "...it breaks down when we're told we can't use extra space."
- **degenerate case** (n. phrase) — a trivially small but valid input (a single-element
  array here) — worth naming to show you've checked boundaries.
