# 2. Clone Graph

**Difficulty:** Medium
**Topic:** Graphs
**Pattern:** DFS/BFS with a hash-map of original -> clone

## Problem
Given a reference node in a connected undirected graph (each node has a value and a list
of neighbors), return a deep copy (clone) of the graph.

## Examples
```
Input: adjList = [[2,4],[1,3],[2,4],[1,3]] -> a structurally identical cloned graph
```

## Approach
DFS from the given node, maintaining a hash map from original node -> its clone. Before
recursing into a node's neighbors, create its clone and store it in the map immediately
(this is what prevents infinite loops on cycles — if we encounter a node already in the
map, we return its existing clone instead of recursing again). For each neighbor, either
reuse its clone from the map or recurse to build it, then append to the current clone's
neighbor list.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **DFS/BFS with a hash-map of original -> clone**,
which itself belongs to the broader **Graph Traversal (BFS, DFS, Union-Find, Topological
Sort)** family of techniques. If the specific trick above feels like it came out of
nowhere, that's the signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it
covers how to recognize this family of problems in general (not just this one), the
reusable template you can write from memory, the usual variations, and the mistakes
people make applying it. Coming back to re-read this problem's approach afterward should
make the specific choices here feel inevitable rather than clever.

## Complexity
- Time: O(V + E)
- Space: O(V) for the map + recursion stack

## Solution
Runnable, with sample test cases at the bottom (`python3 graphs/02_clone_graph/solution.py`):

```python
--8<-- "graphs/02_clone_graph/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "A naive DFS that clones a
  node every time it's referenced would infinite-loop on any cycle, since undirected
  graphs represented this way always have them (each edge implies two directed references).
  So I'd say up front: the real problem isn't traversal, it's *cycle-safe* traversal, which
  is what pushes me to a hash map before writing any recursive call."
- **Invariant framing (good for explaining the map's role precisely):** "The invariant is:
  a node is added to the original-to-clone map *before* I recurse into its neighbors. That
  ordering is what breaks cycles — if I recurse first and map second, a cycle sends me back
  into infinite recursion before the map entry ever gets written."
- **Generalization framing (good for naming the reusable trick):** "This is DFS/BFS plus a
  visited-map that also happens to carry a payload — instead of a boolean 'seen' set, it's
  an original-to-clone dictionary. I'd name that as a reusable variant: whenever a
  traversal needs to *build* something per node while avoiding revisits, a map instead of
  a set is the natural upgrade."

### Vocabulary Builder

- **deep copy** (n. phrase) — a copy where nested/referenced objects are also duplicated,
  not just the top-level reference; contrast with a shallow copy, which would just alias
  the same neighbor list.
- **back edge** (n.) — an edge pointing back to a node already on the current traversal
  path; in an undirected graph, effectively every edge is traversed as a back edge from one
  direction, which is why cycle-safety matters even without an explicit "cycle" in the
  problem statement.
- **memoization** (n.) — caching a computed result so a repeated sub-question is answered
  from cache instead of recomputed; here the "computation" being cached is "the clone of
  this node."
- **"…is the load-bearing detail"** — reusable phrase for calling out the one ordering or
  boundary decision an approach hinges on, e.g. "mapping before recursing is the
  load-bearing detail that prevents infinite recursion."
