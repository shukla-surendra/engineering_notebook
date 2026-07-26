# 6. Decode Ways

**Difficulty:** Medium
**Topic:** 1-D Dynamic Programming
**Pattern:** Fibonacci-style recurrence with validity checks

## Problem
A message of digits (`'A'`->`1` ... `'Z'`->`26`) was encoded. Given the encoded string
`s`, return the number of ways it can be decoded. (Leading zeros make a substring invalid,
e.g. "06" is not decodable.)

## Examples
```
Input: s = "12"   -> 2   ("AB" or "L")
Input: s = "226"  -> 3   ("BZ", "VF", "BBF"... actually "B Z", "V F", "B B F" -> 3 ways)
Input: s = "06"   -> 0   (leading zero, invalid)
```

## Approach
`ways(i)` = number of ways to decode `s[i:]`. It depends on: can `s[i]` alone be decoded
(non-zero) -> add `ways(i+1)`; can `s[i:i+2]` be decoded as a two-digit letter (10-26) ->
add `ways(i+2)`. Build this bottom-up from the end of the string using two running
variables (like climbing stairs, but with validity gating each transition).

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Fibonacci-style recurrence with validity
checks**, which itself belongs to the broader **1-D Dynamic Programming** family of
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
Runnable, with sample test cases at the bottom (`python3 dp_1d/06_decode_ways/solution.py`):

```python
--8<-- "dp_1d/06_decode_ways/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The brute force recursively
  branches at every position into 'take one digit' or 'take two digits' — exponential in the
  worst case. I'd name that before pointing out it's Fibonacci-shaped once you memoize, just
  with extra validity checks gating each branch."
- **Invariant framing (good for explaining why validity checks come first):** "The invariant
  is that `ways(i)` only counts transitions that are actually legal decodings. That's why I
  check 'is `s[i]` non-zero' and 'is `s[i:i+2]` between 10 and 26' *before* adding the
  corresponding `ways(i+1)` or `ways(i+2)` — skip that check and I'd silently count invalid
  decodings like ones with leading zeros."
- **Pattern-recognition framing (good for connecting to Climbing Stairs):** "Structurally
  this is Climbing Stairs with gated transitions — I'd say that explicitly, since it shows
  the recurrence itself isn't novel, only the extra validity logic layered on top of it."

### Vocabulary Builder

- **gated transition** (n. phrase) — a recurrence step that only applies conditionally
  (here, only if the digit span decodes to a valid letter); precise language for describing
  validity-checked DP.
- **degenerate case** (n. phrase) — a leading zero like `"06"`, which is syntactically
  present but decodes to nothing; naming it shows you've considered invalid-but-plausible
  inputs, not just the happy path.
- **"…the crux of it is that not every transition is legal, so I have to check before I
  add"** — a reusable phrase for narrating any DP recurrence with validity gating.
- **two-digit window** (n. phrase) — the `s[i:i+2]` slice checked against 10-26; useful
  shorthand when talking through the recurrence out loud without re-deriving indices each
  time.
