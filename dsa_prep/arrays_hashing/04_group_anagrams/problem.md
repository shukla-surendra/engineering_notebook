# 4. Group Anagrams

**Difficulty:** Medium
**Topic:** Arrays & Hashing
**Pattern:** Hash Map with a canonical key

## Problem
Given an array of strings `strs`, group the anagrams together. You can return the answer
in any order.

## Examples
```
Input: strs = ["eat","tea","tan","ate","nat","bat"]
Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
```

## Approach
Two strings are anagrams iff they share the same sorted form (or the same 26-length letter
count signature). Use that canonical form as a hash map key and bucket every string into
the list for its key. Using a count-tuple signature instead of `sorted(word)` avoids the
O(k log k) sort per word, giving O(k) per word instead.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Hash Map with a canonical key**, which itself
belongs to the broader **Hashing for O(1) Lookups** family of techniques. If the
specific trick above feels like it came out of nowhere, that's the signal to step back
and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of
problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(n·k) where n = number of strings, k = max string length (using count signature)
- Space: O(n·k)

## Solution
Runnable, with sample test cases at the bottom (`python3 arrays_hashing/04_group_anagrams/solution.py`):

```python
--8<-- "arrays_hashing/04_group_anagrams/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Canonical-key framing (the core idea to state first):** "The question is really 'which
  strings collapse to the same identity once you ignore ordering' — so I need a canonical
  form that's identical for every anagram and different for everything else. `sorted(word)`
  is the obvious canonical key; a 26-length count tuple is the faster one."
- **Complexity-refinement framing (shows you iterate on your own answer):** "I'd start with
  sorting each word as the key — simple, O(k log k) per word — then say: since I only care
  about letter counts, not order, I can replace the sort with a count signature and drop the
  log factor, giving O(k) per word instead."
- **Generalization framing (names the family):** "This is 'bucket by a derived key,' the
  same shape as grouping by any computed property — the only thing that changes between
  problems in this family is what `derive_canonical_key` actually computes."

### Vocabulary Builder

- **canonical form** (n. phrase) — a standardized representation such that equivalent
  inputs map to the identical output; the whole trick here is picking the right one.
  *"Sorted letters is one canonical form for anagrams; a count signature is another."*
- **hashable** (adj.) — usable as a dict/set key, meaning immutable and consistently
  hashable; a `tuple` of counts is hashable, a `list` is not.
- **"the crux of it is…"** — useful for pivoting straight from problem restatement to your
  actual insight (here: choosing the key function) without meandering.
- **signature** (n.) — a compact fingerprint derived from an object that's sufficient to
  test equivalence, without comparing the full objects directly.
