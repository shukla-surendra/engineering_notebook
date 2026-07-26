# 2. Number of 1 Bits (Hamming Weight)

**Difficulty:** Easy
**Topic:** Bit Manipulation
**Pattern:** Brian Kernighan's bit trick (`n & (n-1)` clears the lowest set bit)

## Problem
Given an unsigned integer `n`, return the number of `1` bits in its binary representation
(its Hamming weight).

## Examples
```
Input: n = 11 (1011) -> 3
Input: n = 128 (10000000) -> 1
```

## Approach
The naive approach shifts and checks the lowest bit `32` times regardless of how many bits
are actually set. Brian Kernighan's trick is faster in practice: `n & (n - 1)` clears
exactly the lowest set bit of `n` (since `n - 1` flips all bits after and including the
lowest set bit). Repeating this and counting iterations until `n` becomes 0 counts exactly
the number of set bits, in a number of steps equal to the number of 1-bits (not 32).

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Brian Kernighan's bit trick (`n & (n-1)` clears
the lowest set bit)**, which itself belongs to the broader **Bit Manipulation** family
of techniques. If the specific trick above feels like it came out of nowhere, that's the
signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to
recognize this family of problems in general (not just this one), the reusable template
you can write from memory, the usual variations, and the mistakes people make applying
it. Coming back to re-read this problem's approach afterward should make the specific
choices here feel inevitable rather than clever.

## Complexity
- Time: O(k), k = number of set bits (at most 32)
- Space: O(1)

## Solution
Runnable, with sample test cases at the bottom (`python3 bit_manipulation/02_number_of_1_bits/solution.py`):

```python
--8<-- "bit_manipulation/02_number_of_1_bits/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The obvious approach shifts
  and checks 32 fixed positions — O(32), technically constant time but wasteful when most
  bits are 0. I'd mention that before reaching for Kernighan's trick, so the improvement
  reads as a refinement, not a memorized fact."
- **Invariant framing (good for explaining *why* the trick terminates correctly):** "Each
  iteration of `n &= n - 1` strictly clears exactly one set bit — that's the invariant. So
  the loop provably runs exactly (number of set bits) times, not a guess bounded by 32."
- **Pattern-recognition framing (good for connecting to related problems):** "This trick
  generalizes to anything asking 'is this a power of two' or 'clear the lowest set bit' —
  I'd flag that this is one identity out of a small toolbox covered in the bit-manipulation
  pattern, not an isolated party trick."

### Vocabulary Builder

- **Hamming weight** (n.) — the standard name for the count of set bits in a binary
  representation; using the term signals familiarity with the literature.
- **amortized** (adj.) — a cost averaged across operations; contrast with Kernighan's
  trick, whose per-call cost is *input-dependent* rather than amortized.
- **"…runs in proportion to the number of set bits, not the word size"** — a precise,
  reusable way to describe why this beats a fixed 32-iteration loop.
- **isolate the lowest set bit** (v. phrase) — a common bit-manipulation idiom (`n & -n`)
  worth knowing even when the problem at hand uses the related `n & (n-1)` instead.
