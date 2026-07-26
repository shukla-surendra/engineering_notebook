# 7. Word Ladder

**Difficulty:** Hard
**Topic:** Graphs
**Pattern:** BFS shortest path over an implicit graph (words as nodes)

## Problem
Given `beginWord`, `endWord`, and a dictionary `wordList`, return the length of the
shortest transformation sequence from `beginWord` to `endWord`, changing exactly one
letter at a time, with every intermediate word required to be in `wordList`. Return 0 if
no such sequence exists.

## Examples
```
Input: beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
Output: 5   (hit -> hot -> dot -> dog -> cog)
```

## Approach
Model each word as a graph node, with edges between words differing by exactly one letter.
BFS from `beginWord` gives the shortest path in an unweighted graph. Rather than comparing
every pair of words directly (slow), generate all single-letter-wildcard patterns for the
current word (e.g. `"h*t"`, `"*it"`, `"hi*"`) and look them up in a precomputed map from
pattern -> list of matching words — this makes neighbor generation fast. Track visited
words to avoid revisiting.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **BFS shortest path over an implicit graph (words
as nodes)**, which itself belongs to the broader **Graph Traversal (BFS, DFS, Union-
Find, Topological Sort)** family of techniques. If the specific trick above feels like
it came out of nowhere, that's the signal to step back and read
[`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of problems in
general (not just this one), the reusable template you can write from memory, the usual
variations, and the mistakes people make applying it. Coming back to re-read this
problem's approach afterward should make the specific choices here feel inevitable
rather than clever.

## Complexity
- Time: O(N · L²) where N = number of words, L = word length (building the pattern map)
- Space: O(N · L²)

## Solution
Runnable, with sample test cases at the bottom (`python3 graphs/07_word_ladder/solution.py`):

```python
--8<-- "graphs/07_word_ladder/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "First I'd name the implicit
  graph: words are nodes, and an edge exists between two words differing by exactly one
  letter. The naive way to find neighbors — comparing the current word against every other
  word in the list — is O(N·L) per word, O(N²·L) overall; I'd say that cost out loud before
  introducing the wildcard-pattern trick that gets neighbor lookup down near O(L²) per
  word."
- **Invariant framing (good for explaining why BFS specifically, not DFS):** "Since every
  transformation costs exactly one step, this is an unweighted shortest-path question, and
  BFS's invariant — it explores nodes in strictly non-decreasing distance order — is what
  guarantees the *first* time I reach `endWord`, that's the shortest possible sequence. DFS
  would find *a* path, not provably the shortest one."
- **Generalization framing (good for naming the reusable trick):** "This is BFS shortest
  path over a graph that's never built explicitly — neighbors are generated on the fly via
  wildcard patterns rather than stored in an adjacency list. I'd flag that as the reusable
  idea: whenever a graph is too large or awkward to materialize, generate adjacency lazily
  and cache lookups instead."

### Vocabulary Builder

- **implicit graph** (n. phrase) — a graph whose nodes/edges are defined by a rule rather
  than stored explicitly; here, words are nodes and edges are computed via one-letter
  differences rather than precomputed pairwise.
- **wildcard pattern** (n. phrase) — a word with one letter replaced by a placeholder
  (e.g. `"h*t"`), used as a bucket key so all one-letter-away words can be found by a
  single hash lookup instead of a pairwise scan.
- **level-order** (adj. phrase) — processing a BFS queue one full layer (distance) at a
  time before advancing to the next, which is what makes tracking "distance" as a simple
  counter correct.
- **"…trades memory for speed"** — reusable phrase for justifying the precomputed
  pattern-to-words map: it costs O(N·L²) space up front to avoid O(N) comparisons per
  neighbor lookup during the search.
