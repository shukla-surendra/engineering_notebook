# 3. Word Search II

**Difficulty:** Hard
**Topic:** Tries
**Pattern:** Trie of all target words + DFS/backtracking over the grid, pruned by the trie

## Problem
Given an `m x n` grid of characters `board` and a list of strings `words`, return all
words from `words` that can be formed by a path of adjacent cells (horizontally or
vertically), using each cell at most once per word.

## Examples
```
Input: board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]],
       words = ["oath","pea","eat","rain"]
Output: ["eat","oath"]
```

## Approach
Doing a separate DFS-from-every-cell for each word independently is too slow when there
are many words. Instead, build one Trie containing all of `words`. Then DFS from every
cell on the board **once**, walking the trie alongside the board path: only continue in a
direction if the current character exists as a child in the trie (this prunes dead paths
across all words simultaneously). When a trie node marked `is_end` is reached, record that
word (and clear its flag to avoid duplicate results). Mark visited cells during the DFS
and unmark on backtrack.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Trie of all target words + DFS/backtracking
over the grid, pruned by the trie**, which itself belongs to the broader **Trie (Prefix
Tree)** family of techniques. If the specific trick above feels like it came out of
nowhere, that's the signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it
covers how to recognize this family of problems in general (not just this one), the
reusable template you can write from memory, the usual variations, and the mistakes
people make applying it. Coming back to re-read this problem's approach afterward should
make the specific choices here feel inevitable rather than clever.

## Complexity
- Time: O(m·n·4^L) worst case, L = max word length, heavily pruned in practice by the trie
- Space: O(sum of word lengths) for the trie + O(L) recursion stack

## Solution
Runnable, with sample test cases at the bottom (`python3 tries/03_word_search_ii/solution.py`):

```python
--8<-- "tries/03_word_search_ii/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The brute force is: for
  every word in `words`, run a separate DFS from every cell on the board looking for that
  exact word — that's O(words · m · n · 4^L), and it re-explores the same board paths
  once per word. I'd name that redundancy explicitly, since it's exactly what the trie
  optimization removes."
- **Invariant framing (good for explaining the pruning precisely):** "The invariant is:
  I only continue the DFS in a direction if the path so far is a valid prefix of *some*
  word in the trie. That single check is what collapses 'one DFS per word' into 'one DFS
  total' — the trie node I'm currently at encodes exactly which words are still reachable
  from here, for all words simultaneously."
- **Generalization framing (good for the trades-memory-for-speed framing):** "This is the
  general move of 'build an index once, then reuse it across every query' — the same
  instinct behind Design Add and Search Words, just applied to board traversal instead of
  a single lookup. I'd also flag the visited-cell marking and unmarking as classic
  backtracking bookkeeping, distinct from the trie pruning itself."

### Vocabulary Builder

- **prune** (v.) — to cut off a branch of a search early because it cannot lead to a
  valid answer; here, the trie tells the board DFS which directions are dead ends before
  exploring them.
- **amortize** (v.) — to spread a fixed cost (building the trie once) across many
  subsequent operations (checking it against every board path) so the upfront cost pays
  for itself.
- **"…re-explores the same work once per word"** — a reusable phrase for diagnosing
  redundant brute-force work that a shared index (trie, hash map, memo table) eliminates.
- **mark-and-unmark** (v. phrase) — the standard backtracking idiom of temporarily
  marking a cell/state visited, recursing, then restoring it before returning — necessary
  here so a cell already used by an in-progress word doesn't block sibling search paths
  incorrectly.
