# 1. Implement Trie (Prefix Tree)

**Difficulty:** Medium
**Topic:** Tries
**Pattern:** Nested hash-map tree, one node per character

## Problem
Implement a Trie with `insert(word)`, `search(word)` (exact match), and
`starts_with(prefix)` (any word has this prefix).

## Examples
```
insert("apple")
search("apple")   -> True
search("app")     -> False
starts_with("app") -> True
insert("app")
search("app")     -> True
```

## Approach
Each `TrieNode` holds a dict mapping character -> child `TrieNode`, plus a boolean
`is_end` flag. `insert` walks/creates a child node per character and marks `is_end` on the
final node. `search` walks the same way but requires `is_end` to be true at the end.
`starts_with` walks the same way but doesn't check `is_end` — just that the path exists.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Nested hash-map tree, one node per character**,
which itself belongs to the broader **Trie (Prefix Tree)** family of techniques. If the
specific trick above feels like it came out of nowhere, that's the signal to step back
and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of
problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(L) per operation, L = length of the word/prefix
- Space: O(total characters inserted)

## Solution
Runnable, with sample test cases at the bottom (`python3 tries/01_implement_trie/solution.py`):

```python
--8<-- "tries/01_implement_trie/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The brute force is a hash
  set of full words — O(1) exact match, but it can't answer 'does any word start with this
  prefix' without scanning everything. I'd name that gap explicitly, since it's the whole
  reason a trie exists rather than a simpler structure."
- **Invariant framing (good for explaining `is_end` precisely):** "The invariant that
  makes this correct is: a root-to-node path existing in the trie means that path is a
  *prefix* of some inserted word, but only a node with `is_end = True` means a word ends
  exactly there. Conflating 'path exists' with 'word exists' is the single most common bug
  — inserting 'apple' makes 'app' a valid path but not a valid search hit."
- **Generalization framing (good for signaling this is a building block):** "This is the
  base trie — insert and two flavors of lookup that differ only in whether I check
  `is_end`. I'd flag that the next two problems in this set just bolt DFS/backtracking or a
  second trie of query words onto this exact same node structure, so getting this
  foundation right pays off downstream."

### Vocabulary Builder

- **prefix tree** (n. phrase) — another name for a trie, emphasizing that every
  root-to-node path spells out a prefix shared by all words below it.
- **shared prefix / prefix sharing** (n. phrase) — the space-saving property where words
  with common beginnings ("app", "apple", "apply") reuse the same nodes instead of storing
  each string separately.
- **"…conflates path-exists with word-exists"** — a precise phrase for naming the specific
  bug of skipping the `is_end` check in a trie lookup.
- **setdefault** (v., Python-specific) — a dict method that inserts a default value only
  if the key is missing, otherwise returns the existing value; the idiomatic one-liner for
  "get or create" during trie insertion.
