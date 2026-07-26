# 5. Number of Connected Components in an Undirected Graph

**Difficulty:** Medium
**Topic:** Graphs
**Pattern:** Union-Find (Disjoint Set Union) or DFS/BFS over unvisited nodes

## Problem
Given `n` nodes labeled `0` to `n-1` and a list of undirected `edges`, return the number
of connected components in the graph.

## Examples
```
Input: n = 5, edges = [[0,1],[1,2],[3,4]] -> 2
Input: n = 5, edges = [[0,1],[1,2],[2,3],[3,4]] -> 1
```

## Approach
Union-Find is the natural tool here: initialize each node as its own parent, then for
every edge, union the two endpoints' sets (using union-by-rank/size and path compression
for near-O(1) operations). The final answer is the number of distinct root parents
remaining. (An equivalent DFS/BFS approach: iterate over every node, and each time an
unvisited node is found, that's a new component — flood-fill mark everything reachable
from it.)

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Union-Find (Disjoint Set Union) or DFS/BFS over
unvisited nodes**, which itself belongs to the broader **Graph Traversal (BFS, DFS,
Union-Find, Topological Sort)** family of techniques. If the specific trick above feels
like it came out of nowhere, that's the signal to step back and read
[`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of problems in
general (not just this one), the reusable template you can write from memory, the usual
variations, and the mistakes people make applying it. Coming back to re-read this
problem's approach afterward should make the specific choices here feel inevitable
rather than clever.

## Complexity
- Time: O(n + E · α(n)) with union-find (α = inverse Ackermann, effectively constant)
- Space: O(n)

## Solution
Runnable, with sample test cases at the bottom (`python3 graphs/05_number_of_connected_components/solution.py`):

```python
--8<-- "graphs/05_number_of_connected_components/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "A brute-force reading would
  be to build an adjacency list and DFS from every unvisited node, counting how many times
  I start a fresh traversal — that's a completely valid O(V+E) answer. I'd present it
  first, then offer Union-Find as the alternative that's often preferred here because it
  handles the edges as an incremental stream rather than requiring the full adjacency list
  up front."
- **Invariant framing (good for explaining what Union-Find is actually tracking):** "The
  invariant is: at any point, the number of distinct root parents equals the number of
  connected components formed by the edges processed *so far*. Each union either merges two
  components (root count drops by one) or is a no-op on an edge that connects a component
  to itself (root count unchanged) — there's no way to accidentally overcount."
- **Generalization framing (good for naming when Union-Find beats DFS):** "I'd say
  explicitly: DFS and Union-Find are interchangeable for a fixed graph, but Union-Find is
  the natural choice specifically when connectivity is being built up edge-by-edge and I
  might need the answer *between* edges too — that's the signal that separates this from a
  plain 'count islands in a static graph' problem."

### Vocabulary Builder

- **disjoint set / Union-Find** (n. phrase) — a data structure tracking a partition of
  elements into non-overlapping sets, supporting near-constant-time union and find.
- **path compression** (n. phrase) — flattening the tree during `find` so future lookups
  are faster; without it, Union-Find degrades toward O(n) per operation on adversarial
  chains.
- **amortized** (adj.) — describing a cost averaged over a sequence of operations; with
  path compression and union-by-size, each Union-Find operation is amortized nearly O(1).
- **"…incrementally, not all at once"** — reusable phrase for describing when Union-Find
  beats a fixed adjacency-list traversal: the structure updates cleanly as edges arrive one
  at a time, rather than needing the full graph up front.
