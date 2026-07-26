# 1. Valid Palindrome

**Difficulty:** Easy
**Topic:** Two Pointers
**Pattern:** Two Pointers (converging from both ends)

## Problem
Given a string `s`, return `True` if it is a palindrome after: converting all uppercase
letters to lowercase, and removing all non-alphanumeric characters.

## Examples
```
Input: s = "A man, a plan, a canal: Panama" -> True
Input: s = "race a car"                      -> False
Input: s = " "                                -> True
```

## Approach
Use two pointers, one from the start and one from the end. Skip non-alphanumeric
characters at each pointer. Compare the lowercase versions of the characters at both
pointers; mismatch means not a palindrome. Move both pointers inward until they cross.
This avoids building a cleaned copy of the string.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Two Pointers (converging from both ends)**,
which itself belongs to the broader **Two Pointers** family of techniques. If the
specific trick above feels like it came out of nowhere, that's the signal to step back
and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of
problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(n)
- Space: O(1)

## Solution
Runnable, with sample test cases at the bottom (`python3 two_pointers/01_valid_palindrome/solution.py`):

```python
--8<-- "two_pointers/01_valid_palindrome/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The brute force is: build a
  cleaned, lowercased copy of the string, then compare it to its reverse — O(n) time but
  also O(n) extra space for the copy. I'd name that first, then say two pointers gets the
  same time complexity down to O(1) space by never materializing the cleaned string at
  all."
- **Invariant framing (good for explaining the skip-then-compare order):** "The invariant
  is: at every step, both pointers are sitting on alphanumeric characters before I compare
  them. That's why the skip-while-not-alphanumeric step has to happen *before* the
  comparison, not after — comparing first would silently let punctuation influence the
  result."
- **Generalization framing (good for naming the family):** "This is the simplest form of
  converging two pointers — no target sum, no sorting required, just 'walk inward and bail
  on the first mismatch.' I'd connect it to Container With Most Water and 3Sum as the same
  inward-converging skeleton with a different stopping/matching condition."

### Vocabulary Builder

- **alphanumeric** (adj.) — consisting of letters and digits only, excluding punctuation
  and whitespace; the exact filter this problem requires before comparing characters.
- **converge** (v.) — to move toward each other and eventually meet or cross; describes
  how the two pointers close the gap from opposite ends of the string.
- **"…trades a data copy for a pair of indices"** — a reusable phrase for describing the
  space optimization when two pointers replace an approach that would otherwise build a
  new string/array.
- **degenerate case** (n. phrase) — an edge case that's valid but trivial, like an empty
  string or one with no alphanumeric characters at all (`" "`), which should trivially
  return True — worth stating that the pointers simply cross without ever comparing
  anything.
