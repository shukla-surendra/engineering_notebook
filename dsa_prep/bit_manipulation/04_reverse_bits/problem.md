# 4. Reverse Bits

**Difficulty:** Easy
**Topic:** Bit Manipulation
**Pattern:** Bit-by-bit extraction and reassembly

## Problem
Given a 32-bit unsigned integer `n`, return its bits reversed.

## Examples
```
Input: n = 00000010100101000001111010011100 (43261596)
Output:   00111001011110000010100101000000 (964176192)
```

## Approach
Build the result bit by bit: for each of the 32 positions, shift the result left to make
room, then OR in the lowest bit of `n` (`n & 1`), then shift `n` right to expose the next
bit. After 32 iterations, every bit of `n` has been extracted lowest-to-highest and placed
into `result` highest-to-lowest — exactly the reversal.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Bit-by-bit extraction and reassembly**, which
itself belongs to the broader **Bit Manipulation** family of techniques. If the specific
trick above feels like it came out of nowhere, that's the signal to step back and read
[`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of problems in
general (not just this one), the reusable template you can write from memory, the usual
variations, and the mistakes people make applying it. Coming back to re-read this
problem's approach afterward should make the specific choices here feel inevitable
rather than clever.

## Complexity
- Time: O(32) = O(1)
- Space: O(1)

## Solution
Runnable, with sample test cases at the bottom (`python3 bit_manipulation/04_reverse_bits/solution.py`):

```python
--8<-- "bit_manipulation/04_reverse_bits/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move, even though there's no real
  alternative here):** "There isn't a cleverer trick to reach for — the honest brute force
  *is* the optimal solution: touch each of the 32 bit positions once. I'd say that
  explicitly so it's clear I considered alternatives rather than assuming O(32) by
  default."
- **Invariant framing (good for narrating the loop precisely):** "The invariant each
  iteration maintains is: 'the bits pulled out of `n` so far have been appended to
  `result` in reverse order.' Shifting `result` left before OR-ing in the new bit is what
  keeps that invariant — get the order of those two operations backwards and the bits land
  in the wrong slot."
- **Pattern-recognition framing (good for naming the reusable shape):** "This is the
  extract-and-rebuild template — pull the lowest bit off one number, append it to a result
  being built up, repeat a fixed number of times. It shows up anywhere you need to process
  a fixed-width value bit by bit."

### Vocabulary Builder

- **fixed-width** (adj.) — bound to a specific bit count (32 here); calling this out
  explains why the loop bound is `range(32)` rather than something input-dependent.
- **two's complement** (n.) — the representation scheme for signed integers; worth
  mentioning if asked how this would change for signed vs. unsigned interpretations.
- **"…the crux of it is getting the shift direction and order right"** — a natural way to
  narrate a bit-assembly loop without getting lost in the mechanics.
- **mask** (n./v.) — a bit pattern used to isolate or clear specific bits (e.g.
  `& 0xFFFFFFFF`); relevant if the language's integers aren't natively fixed-width, as in
  Python.
