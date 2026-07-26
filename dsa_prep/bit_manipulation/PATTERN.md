# Pattern: Bit Manipulation

## What problem does this solve?

A small set of bitwise identities let you solve certain counting/uniqueness/toggling
problems in O(1) space and often O(1) time per element, where a hash-set or extra-array
approach would cost O(n) space. Bit tricks show up less often than the other patterns in
this collection, but when they apply, they're usually dramatically simpler than the
alternative once you recognize which identity fits.

## How to recognize it

Signals that a bit trick applies:
- "Every element appears twice except one" (or similar parity statements) → XOR
  cancellation.
- "Count set bits," "is this a power of two," "clear/isolate the lowest set bit" → Brian
  Kernighan's `n & (n-1)` trick and its relatives.
- A DP-shaped counting problem where the *previous* answer for a related, smaller number is
  trivially derivable by looking at one bit (Counting Bits) — bit manipulation combined
  with DP.
- The problem explicitly asks for O(1) space and the naive solution wants a hash
  set/array — that's a strong hint a bitwise identity might replace it.
- Missing-number-style problems over a known, complete range `[0, n]` — arithmetic (sum
  formula) and XOR both solve these; recognizing *both* options is worth having.

## The core identities

**XOR cancellation** — `x ^ x = 0` and `x ^ 0 = x`, and XOR is commutative/associative, so
order doesn't matter:
```python
result = 0
for x in nums:
    result ^= x
# every value appearing an even number of times cancels to 0; the odd-one-out survives
```
This single identity solves Single Number directly, and (combined with also XOR-ing every
index in range) solves Missing Number too — XOR-ing `0..n` together with all the array
values cancels every present value against its matching index, leaving only the missing
one unpaired.

**Brian Kernighan's trick** — `n & (n - 1)` clears exactly the lowest set bit of `n`:
```python
count = 0
while n:
    n &= n - 1
    count += 1
```
Why it works: subtracting 1 from `n` flips every bit from the lowest set bit downward (that
bit becomes 0, everything below it becomes 1). ANDing with the original `n` keeps only the
bits *above* that point unchanged and zeroes everything from the lowest set bit down —
netting a single bit cleared per iteration, so the loop runs exactly (number of set bits)
times, not 32 times regardless of input.

**Extract-and-rebuild** (Reverse Bits) — process a fixed number of bit positions, pulling
the lowest bit off one number and appending it to a result being built up:
```python
result = 0
for _ in range(32):
    result = (result << 1) | (n & 1)
    n >>= 1
```

**DP built on a one-bit-shorter subproblem** (Counting Bits) — `ans[i] = ans[i >> 1] +
(i & 1)`: shifting right by 1 drops the lowest bit, and that smaller number's bit count is
already known from earlier in the same table — this is 1-D DP (see
`../dp_1d/PATTERN.md`) with the specific recurrence coming from a bitwise observation.

## Common pitfalls

- Reaching for a 32-iteration loop (checking each bit position one at a time) when Brian
  Kernighan's trick would finish in O(number of set bits) instead — both are technically
  O(32) = O(1), but the distinction matters if asked to reason about it precisely.
- Forgetting that Python integers are arbitrary-precision — bit tricks that assume a fixed
  32-bit width (like Reverse Bits) need an explicit loop bound (`range(32)`), since there's
  no automatic wraparound like in a fixed-width language.
- Sign/negative-number confusion: bitwise operations on negative numbers in Python use an
  infinite two's-complement representation conceptually, which can surprise you if a
  problem assumes unsigned 32-bit semantics — mask with `& 0xFFFFFFFF` when a problem
  explicitly wants unsigned 32-bit behavior.

## Complexity characteristics

O(1) or O(number of bits) time — effectively constant for fixed-width integers — and O(1)
space, which is usually the entire reason to reach for a bit trick over a hash-set/array
approach that would cost O(n) space instead.

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Recognition framing (how you'd explain *when* this pattern applies, before any
  specific problem):** "I listen for language like 'every element appears twice except
  one,' 'count set bits,' or 'O(1) space required' — those are the tells that a bitwise
  identity might replace a hash set. I'd say this out loud early, since it reframes the
  problem from 'find the trick' to 'match the phrasing to a known identity.'"
- **Mental-model framing (good for explaining why these tricks work, not just that they
  do):** "Every trick here boils down to one of three things: cancellation (XOR undoes
  pairs), subtraction-based bit clearing (`n & (n-1)`), or bit-by-bit extraction into a
  new value. I'd name which of the three a new problem needs before writing any code —
  it's a much smaller search space than 'try things.'"
- **Generalization framing (good for signaling you see bit tricks as one tool among
  several, not a silver bullet):** "I'd point out that Counting Bits isn't purely a bit
  trick — it's 1-D DP where the recurrence happens to come from a bitwise observation. That
  matters because it shows these categories compose rather than being mutually exclusive."

### Vocabulary Builder

- **identity** (n.) — a rule that's always true by definition (e.g. `x ^ x = 0`); precise
  word for what these tricks actually are, as opposed to heuristics.
- **two's complement** (n.) — the standard representation for signed integers; relevant
  whenever a problem's bitwise semantics depend on sign, which Python doesn't handle the
  same way fixed-width languages do.
- **"…trades an O(n) structure for an O(1) identity"** — a reusable phrase for justifying
  any bit-trick optimization over a hash-set or array-based approach.
- **fixed-width** (adj.) — bound to a specific number of bits (commonly 32); calling this
  out explains why some solutions need an explicit masking step in Python, which has
  arbitrary-precision integers by default.
