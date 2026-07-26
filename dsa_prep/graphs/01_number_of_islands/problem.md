# 1. Number of Islands

**Difficulty:** Medium
**Topic:** Graphs
**Pattern:** Grid DFS/BFS flood fill

## Problem
Given an `m x n` binary grid where `1` is land and `0` is water, return the number of
islands (groups of land connected horizontally/vertically).

## Examples
```
Input: grid = [
 ["1","1","0","0","0"],
 ["1","1","0","0","0"],
 ["0","0","1","0","0"],
 ["0","0","0","1","1"]]
Output: 3
```

## Approach
Scan every cell. On an unvisited `1`, increment the island count and flood-fill (DFS or
BFS) to mark every connected land cell as visited (e.g. by mutating it to `"0"` in place,
or using a separate visited set). This ensures each island is only counted once no matter
its shape.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Grid DFS/BFS flood fill**, which itself belongs
to the broader **Graph Traversal (BFS, DFS, Union-Find, Topological Sort)** family of
techniques. If the specific trick above feels like it came out of nowhere, that's the
signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to
recognize this family of problems in general (not just this one), the reusable template
you can write from memory, the usual variations, and the mistakes people make applying
it. Coming back to re-read this problem's approach afterward should make the specific
choices here feel inevitable rather than clever.

## Complexity
- Time: O(m·n)
- Space: O(m·n) worst case recursion/queue depth

## Solution
Runnable, with sample test cases at the bottom (`python3 graphs/01_number_of_islands/solution.py`):

```python
--8<-- "graphs/01_number_of_islands/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The naive idea would be to
  somehow group land cells after the fact, but there's really no cheaper starting point
  than a full scan — so I'd say up front: I'm going to visit every cell once, and the only
  question is what I do when I hit unvisited land. That framing makes the O(m·n) bound
  obvious before I've written a line of code."
- **Invariant framing (good for explaining why double-counting can't happen):** "The
  invariant is: once a land cell has been flood-filled, it's marked visited and will never
  trigger a new count again. Incrementing the island counter only on an *unvisited* `1`,
  combined with immediately marking the whole connected region during flood-fill, is what
  guarantees each island is counted exactly once regardless of its shape."
- **Generalization framing (good for signaling you see the bigger family):** "I'd say
  explicitly: a grid is just a graph where cells are nodes and adjacency is up/down/left/
  right — this is flood fill, the same primitive behind Pacific Atlantic Water Flow, just
  with a different stopping condition. I'd point to that shared reframing rather than
  presenting it as a grid-specific trick."

### Vocabulary Builder

- **flood fill** (n./v.) — traversing (DFS or BFS) all cells reachable from a starting
  cell under some adjacency rule, marking each as visited. *"I flood-fill from every
  unvisited land cell to sweep the whole island in one pass."*
- **connected component** (n. phrase) — a maximal set of nodes all reachable from one
  another; each island is one connected component of the land subgraph.
- **in-place mutation** (n. phrase) — reusing the input grid itself (flipping `"1"` to
  `"0"`) as the visited-tracking structure, trading a cleaner interface for O(1) extra
  visited-space.
- **"…is a graph problem in disguise"** — a reusable phrase for reframing grid problems,
  signaling recognition of the underlying pattern rather than treating the grid as a
  special case.
