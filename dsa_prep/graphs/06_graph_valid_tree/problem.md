# 6. Graph Valid Tree

**Difficulty:** Medium
**Topic:** Graphs
**Pattern:** Union-Find, checking edge count and connectivity/no-cycle

## Problem
Given `n` nodes labeled `0` to `n-1` and a list of undirected `edges`, determine if these
edges form a valid tree (connected, and acyclic).

## Examples
```
Input: n = 5, edges = [[0,1],[0,2],[0,3],[1,4]] -> True
Input: n = 5, edges = [[0,1],[1,2],[2,3],[1,3],[1,4]] -> False (cycle)
```

## Approach
A graph with `n` nodes is a tree iff it is connected **and** has exactly `n - 1` edges
(any fewer can't connect everything; any more forces a cycle given connectivity). Use
Union-Find: for each edge, if both endpoints are already in the same set, adding this edge
would create a cycle — fail immediately. Otherwise union them. At the end, check that
there's exactly one connected component left (equivalent to checking edge count == n-1
given no cycle was found along the way).

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Union-Find, checking edge count and
connectivity/no-cycle**, which itself belongs to the broader **Graph Traversal (BFS,
DFS, Union-Find, Topological Sort)** family of techniques. If the specific trick above
feels like it came out of nowhere, that's the signal to step back and read
[`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of problems in
general (not just this one), the reusable template you can write from memory, the usual
variations, and the mistakes people make applying it. Coming back to re-read this
problem's approach afterward should make the specific choices here feel inevitable
rather than clever.

## Complexity
- Time: O(n + E · α(n))
- Space: O(n)

## Solution
Runnable, with sample test cases at the bottom (`python3 graphs/06_graph_valid_tree/solution.py`):

```python
--8<-- "graphs/06_graph_valid_tree/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "I'd first restate the
  definition precisely: a tree on `n` nodes is connected *and* acyclic, and those two
  conditions together are equivalent to 'connected with exactly `n-1` edges.' Naming that
  equivalence up front is what turns this from 'write some graph traversal' into 'check two
  cheap conditions,' which is the actual efficient solution, not an afterthought
  optimization."
- **Invariant framing (good for explaining the early-exit cycle check):** "The invariant
  Union-Find gives me is: two nodes share a root if and only if they're already connected
  by edges processed so far. So when I process an edge and both endpoints already share a
  root, that edge is redundant — adding it can only create a cycle, so I can fail
  immediately without even finishing the edge list."
- **Generalization framing (good for connecting it to the sibling problem):** "This uses
  the identical Union-Find machinery as Number of Connected Components, just checked
  against a stricter condition — one component at the end, with the cycle check happening
  inline during the unions rather than only counting components after the fact. I'd name
  that shared machinery to show these aren't two separate tricks."

### Vocabulary Builder

- **acyclic** (adj.) — containing no cycles; combined with connectivity, the defining
  property of a tree.
- **redundant edge** (n. phrase) — an edge connecting two nodes already in the same
  component; its presence alone disqualifies the graph from being a tree, independent of
  edge count.
- **necessary and sufficient** (adj. phrase) — a precise way to state that "connected AND
  n-1 edges" doesn't just imply "is a tree," it's logically equivalent to it — useful
  vocabulary for justifying a shortcut condition instead of just asserting it works.
- **"…fail fast"** — reusable phrase for justifying an early return the moment an
  invariant is violated (here, the moment a cycle-forming edge is found), rather than
  finishing all the work before checking.
