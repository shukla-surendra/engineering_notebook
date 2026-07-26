# 2. Valid Anagram

**Difficulty:** Easy
**Topic:** Arrays & Hashing
**Pattern:** Hash Map / Frequency Count

## Problem
Given two strings `s` and `t`, return `True` if `t` is an anagram of `s`, and `False`
otherwise. An anagram uses exactly the same letters with the same frequency, rearranged.

## Examples
```
Input: s = "anagram", t = "nagaram" -> True
Input: s = "rat", t = "car"         -> False
```

## Approach
If the lengths differ, they can't be anagrams. Otherwise count letter frequencies of `s`
in a hash map (or a fixed-size array of 26 for lowercase-only inputs), then decrement while
scanning `t`. If every count returns to zero, they're anagrams. Sorting both strings and
comparing (O(n log n)) is a simpler but slower alternative.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Hash Map / Frequency Count**, which itself
belongs to the broader **Hashing for O(1) Lookups** family of techniques. If the
specific trick above feels like it came out of nowhere, that's the signal to step back
and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of
problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(n)
- Space: O(1) (bounded alphabet) or O(n) for a general hash map

## Solution
Runnable, with sample test cases at the bottom (`python3 arrays_hashing/02_valid_anagram/solution.py`):

```python
--8<-- "arrays_hashing/02_valid_anagram/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Early-exit framing (shows you check cheap conditions first):** "Before touching any
  data structure, I check lengths — mismatched lengths are an instant `False` and save me
  the trouble of building a frequency map at all. Cheap checks first is a habit, not just
  luck."
- **Invariant framing (for the frequency-count approach):** "The invariant is: after
  incrementing for every character in `s` and decrementing for every character in `t`,
  every count should land back at exactly zero. If any count is nonzero at the end, the
  multisets of characters differ."
- **Trade-off framing (for comparing to the sorting alternative):** "Sorting both strings
  and comparing them is the more obvious approach, at O(n log n) — I'd mention it, then say
  the frequency-count version trades the log factor away by counting in a single pass
  instead of ordering."

### Vocabulary Builder

- **multiset** (n.) — a set where an element can appear more than once and its count
  matters; anagram-checking is really multiset equality. *"Two strings are anagrams iff
  their character multisets are equal."*
- **bounded alphabet** (n. phrase) — a fixed, small set of possible characters (like
  lowercase a–z); lets you use a fixed-size array instead of a general hash map for O(1)
  space.
- **"trades a log factor for a linear pass"** — a precise way to describe swapping a
  sort-based solution for a counting-based one.
- **symmetric check** (adj. phrase) — describing an approach that treats both inputs
  identically (increment on one, decrement on the other) rather than comparing asymmetrically.
