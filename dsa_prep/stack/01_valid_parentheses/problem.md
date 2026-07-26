# 1. Valid Parentheses

**Difficulty:** Easy
**Topic:** Stack
**Pattern:** Stack matching for nested/paired structures

## Problem
Given a string `s` containing just `(`, `)`, `{`, `}`, `[`, `]`, determine if the input is
valid: every open bracket is closed by the same type of bracket, and in the correct order.

## Examples
```
Input: s = "()[]{}" -> True
Input: s = "(]"      -> False
Input: s = "([)]"    -> False
Input: s = "{[]}"    -> True
```

## Approach
Push every opening bracket onto a stack. On a closing bracket, check the stack's top: it
must be the matching opening bracket, or the string is invalid — pop it if so. At the end,
the string is valid only if the stack is empty (every opener was matched).

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Stack matching for nested/paired structures**,
which itself belongs to the broader **Stack** family of techniques. If the specific
trick above feels like it came out of nowhere, that's the signal to step back and read
[`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of problems in
general (not just this one), the reusable template you can write from memory, the usual
variations, and the mistakes people make applying it. Coming back to re-read this
problem's approach afterward should make the specific choices here feel inevitable
rather than clever.

## Complexity
- Time: O(n)
- Space: O(n)

## Solution
Runnable, with sample test cases at the bottom (`python3 stack/01_valid_parentheses/solution.py`):

```python
--8<-- "stack/01_valid_parentheses/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "A naive approach might
  repeatedly scan for and remove adjacent matched pairs like `()` until nothing changes —
  correct, but O(n²) in the worst case since each removal can require a rescan. I'd name
  that, then say a stack gets to O(n) by resolving matches in a single pass instead of
  repeated scans."
- **Invariant framing (good for explaining the stack check precisely):** "The invariant
  is: the stack always holds exactly the currently-unmatched openers, in the order they
  were opened. A closer is only valid if it matches the top of the stack — checking the
  top, not just 'is there an opener somewhere,' is what enforces correct nesting order,
  not just correct counts."
- **Generalization framing (good for signaling pattern recognition):** "This is the
  canonical 'stack for matching/nesting' problem — the same LIFO discipline shows up
  anytime 'the most recently opened thing must be the first thing closed,' like nested
  tags or nested function calls."

### Vocabulary Builder

- **LIFO** (n., last-in-first-out) — the ordering discipline a stack enforces; naming it
  explicitly shows you know *why* a stack (not a queue) is the right structure here.
  *"LIFO order mirrors how nesting resolves — the innermost thing closes first."*
- **sentinel check** (n. phrase) — a guard like `if not stack` before popping, to avoid
  crashing on an unmatched closer with nothing to pop.
- **"resolves in a single pass"** — a reusable phrase for describing why a stack-based
  approach beats a naive rescan-based one.
- **degenerate case** (n. phrase) — an empty string (trivially valid) or a lone opener/
  closer with no match — worth naming as boundary conditions to verify explicitly.
