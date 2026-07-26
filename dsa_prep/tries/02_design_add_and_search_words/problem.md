# 2. Design Add and Search Words Data Structure

**Difficulty:** Medium
**Topic:** Tries
**Pattern:** Trie + DFS/backtracking for wildcard `.` matching

## Problem
Design a data structure supporting `add_word(word)` and `search(word)`, where `search`'s
query may contain `.` as a wildcard matching any single letter.

## Examples
```
add_word("bad"); add_word("dad"); add_word("mad")
search("pad") -> False
search("bad") -> True
search(".ad") -> True
search("b..") -> True
```

## Approach
Same Trie structure as a standard prefix tree. `add_word` is identical to `insert`.
`search` needs DFS instead of a simple walk: at each character, if it's a literal letter,
follow that single child (or fail); if it's `.`, try **every** child at that level
recursively and succeed if any path succeeds. This is a backtracking search over the trie.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Trie + DFS/backtracking for wildcard `.`
matching**, which itself belongs to the broader **Trie (Prefix Tree)** family of
techniques. If the specific trick above feels like it came out of nowhere, that's the
signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to
recognize this family of problems in general (not just this one), the reusable template
you can write from memory, the usual variations, and the mistakes people make applying
it. Coming back to re-read this problem's approach afterward should make the specific
choices here feel inevitable rather than clever.

## Complexity
- Time: O(L) for a word with no wildcards; O(26^d · L) worst case with d dots
- Space: O(total characters inserted)

## Solution
Runnable, with sample test cases at the bottom (`python3 tries/02_design_add_and_search_words/solution.py`):

```python
--8<-- "tries/02_design_add_and_search_words/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The brute force is: store
  every word in a list, and for a wildcard query, regex-match or manually compare against
  every stored word — O(number of words · L). I'd name that, then say the trie version is
  really about pruning the search early rather than changing the asymptotic worst case
  outright, since a query like '...' still has to explore broadly."
- **Invariant framing (good for explaining why `.` needs recursion, not a loop):** "A
  plain letter narrows me to exactly one child, so a simple `while` loop suffices — that's
  the base trie. The moment I hit a `.`, I no longer have one path to follow, I have up to
  26, so the invariant shifts from 'walk a single path' to 'succeed if *any* branch
  succeeds' — that's exactly what recursion with an `any(...)` expresses, and a loop
  can't express it as cleanly."
- **Generalization framing (good for naming the technique):** "This is backtracking
  layered on top of a trie walk: try a branch, and if it doesn't pan out, that recursive
  call just returns False and I've implicitly backtracked with no extra bookkeeping. I'd
  connect that to the exact same 'try each option, recurse, let failure propagate up'
  shape used in general backtracking problems."

### Vocabulary Builder

- **backtracking** (n.) — a search strategy that tries a choice, recurses, and
  automatically abandons that choice (returns to the previous state) if it doesn't lead to
  a solution; here, trying each of up to 26 children for a wildcard.
- **worst case** (n. phrase) — here, a query of all dots (`"..."`) forces exploring every
  path of that length in the trie, giving the O(26^d · L) bound — worth stating precisely
  rather than just saying "it's slower with dots."
- **"…pruning the search early rather than changing the worst case"** — an honest,
  reusable phrase for describing an optimization that helps the common case a lot without
  improving the theoretical worst-case bound.
- **branching factor** (n. phrase) — the number of choices available at each step of a
  search (here, up to 26 possible children per trie node); a higher branching factor
  directly inflates a backtracking search's cost.
