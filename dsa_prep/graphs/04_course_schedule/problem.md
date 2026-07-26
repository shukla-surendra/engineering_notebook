# 4. Course Schedule

**Difficulty:** Medium
**Topic:** Graphs
**Pattern:** Cycle detection in a directed graph (topological sort / DFS coloring)

## Problem
Given `numCourses` and a list of prerequisite pairs `[a, b]` (must take `b` before `a`),
return `True` if it's possible to finish all courses (i.e. the prerequisite graph has no
cycle).

## Examples
```
Input: numCourses = 2, prerequisites = [[1,0]] -> True
Input: numCourses = 2, prerequisites = [[1,0],[0,1]] -> False  (cycle)
```

## Approach
Build an adjacency list, then DFS with a 3-color scheme per node: unvisited, "in the
current recursion path" (visiting), and fully processed (visited). If DFS reaches a node
that's currently marked "visiting", that's a back edge — a cycle, so return `False`. After
fully exploring a node's neighbors, mark it "visited" (safe, can be revisited freely by
other paths) and remove it from the "visiting" set. No cycle across the whole graph means
all courses are finishable.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Cycle detection in a directed graph
(topological sort / DFS coloring)**, which itself belongs to the broader **Graph
Traversal (BFS, DFS, Union-Find, Topological Sort)** family of techniques. If the
specific trick above feels like it came out of nowhere, that's the signal to step back
and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of
problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(V + E)
- Space: O(V + E)

## Solution
Runnable, with sample test cases at the bottom (`python3 graphs/04_course_schedule/solution.py`):

```python
--8<-- "graphs/04_course_schedule/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The question 'can all courses
  be finished' is really 'does the prerequisite graph have a cycle,' so I'd restate the
  problem that way immediately. A naive plain-visited DFS can't answer that correctly on a
  directed graph, because it can't tell 'already fully explored elsewhere' apart from
  'currently on my own recursion path' — that gap is exactly why a richer coloring scheme
  is needed, not an optimization afterthought."
- **Invariant framing (good for explaining the three-color scheme precisely):** "The
  invariant is: a node is 'visiting' only while it's on the current DFS call stack. If DFS
  ever reaches a node still marked 'visiting,' that's a back edge to an ancestor — a cycle
  — and I return `False` immediately. Once a node's neighbors are fully explored with no
  cycle found, I mark it 'visited' so future paths can safely skip it without re-deriving
  the same answer."
- **Generalization framing (good for placing this in the bigger toolbox):** "This is
  directed-cycle detection, the same 3-color DFS that underlies topological sort — 'can
  all courses finish' and 'is there a valid course ordering' are the same question asked
  two ways. I'd mention that a valid ordering could be produced with the same traversal if
  asked for it."

### Vocabulary Builder

- **topological sort** (n. phrase) — a linear ordering of a directed acyclic graph's nodes
  such that every edge points from earlier to later in the ordering; only possible when no
  cycle exists, which is exactly what this problem is checking for.
- **back edge** (n.) — in DFS, an edge to an ancestor still on the current recursion path;
  its presence is both necessary and sufficient for a cycle in a directed graph.
- **acyclic** (adj.) — containing no cycles; "DAG" (directed acyclic graph) is the term for
  a graph that satisfies this and is therefore topologically sortable.
- **"…the crux of it is…"** — reusable phrase for compressing a restated problem into its
  core question, e.g. "the crux of it is telling apart 'fully explored' from 'still on my
  current path.'"
