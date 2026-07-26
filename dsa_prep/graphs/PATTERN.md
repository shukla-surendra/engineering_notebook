# Pattern: Graph Traversal (BFS, DFS, Union-Find, Topological Sort)

## What problem does this solve?

Graphs generalize trees and grids: entities (nodes) connected by relationships (edges),
where you need to explore reachability, shortest paths, connectivity, or ordering
constraints. The four core tools — BFS, DFS, Union-Find, and topological sort — each solve
a specific *shape* of question, and picking the right one is 90% of the difficulty; the
implementations themselves are short and formulaic.

## How to recognize which tool to use

- **"Shortest path" in an unweighted graph** → BFS. BFS explores in increasing distance
  order, so the first time you reach a target, that's guaranteed the shortest path.
- **"Does a path exist," "flood fill a region," "explore everything reachable"** → DFS (or
  BFS — both work; DFS is often simpler to write recursively).
- **"Are these two nodes in the same group," "count groups," "adding this edge would
  create a cycle"** → Union-Find (Disjoint Set Union) — when you're building connectivity
  incrementally (processing a list of edges) rather than starting from a fixed adjacency
  list you traverse all at once.
- **"Is there a valid ordering given prerequisite/dependency constraints," "does a cycle
  exist in a *directed* graph"** → topological sort / DFS cycle detection with a
  visiting/visited coloring scheme.
- **Grid problems ("number of islands," "water flow")** → the grid itself *is* the graph;
  cells are nodes, adjacency is up/down/left/right neighbors. Recognize this reframing
  immediately — grid problems are graph problems in disguise.

## The general template

**BFS (shortest path / level-order):**
```python
from collections import deque
queue = deque([start])
visited = {start}
distance = 0
while queue:
    for _ in range(len(queue)):     # process one full "layer" at a time
        node = queue.popleft()
        if node == target:
            return distance
        for neighbor in graph[node]:
            if neighbor not in visited:
                visited.add(neighbor)
                queue.append(neighbor)
    distance += 1
return -1  # unreachable
```

**DFS (reachability / flood fill):**
```python
def dfs(node, visited):
    if node in visited:
        return
    visited.add(node)
    for neighbor in graph[node]:
        dfs(neighbor, visited)
```

**Union-Find (with path compression + union by size — near-O(1) amortized ops):**
```python
parent = list(range(n))
size = [1] * n
def find(x):
    while parent[x] != x:
        parent[x] = parent[parent[x]]   # path compression
        x = parent[x]
    return x
def union(a, b):
    ra, rb = find(a), find(b)
    if ra == rb:
        return False       # already connected — this edge would create a cycle
    if size[ra] < size[rb]:
        ra, rb = rb, ra
    parent[rb] = ra
    size[ra] += size[rb]
    return True
```

**Directed-cycle detection (3-color DFS — the basis of topological sort):**
```python
WHITE, GRAY, BLACK = 0, 1, 2   # unvisited, in current recursion path, fully done
color = defaultdict(int)
def has_cycle(node):
    if color[node] == BLACK: return False
    if color[node] == GRAY: return True     # back edge -> cycle
    color[node] = GRAY
    for neighbor in graph[node]:
        if has_cycle(neighbor):
            return True
    color[node] = BLACK
    return False
```
The GRAY state (currently on the path from the root to here) is the crucial third state —
a plain visited/unvisited boolean can't distinguish "already fully explored, safe to
revisit" from "currently being explored higher up the same path, revisiting means a cycle."

**Multi-source BFS/DFS** (Pacific Atlantic Water Flow): instead of checking, for every
cell, whether it can reach a target region (expensive, checked from n² starting points),
start the search *from every border cell simultaneously*, run it in reverse (what can flow
back to here), and mark everything reachable — one traversal per ocean instead of one per
cell.

## Common pitfalls

- Using a plain visited set for cycle detection in a *directed* graph — this misses the
  distinction between "fully explored" and "currently being explored on this path,"
  causing false negatives on cycles.
- Forgetting union-by-size/rank and path compression in Union-Find — still correct, but
  can degrade to O(n) per operation on adversarial input instead of near-O(1).
- Treating a grid problem as "not really a graph problem" and missing that visited-marking
  and 4-directional adjacency is exactly graph BFS/DFS.
- BFS without tracking visited nodes *before* enqueueing (checking visited only on dequeue)
  can enqueue the same node multiple times, wasting work or breaking distance correctness.

## Complexity characteristics

O(V + E) for BFS/DFS. O((V + E) · α(V)) for Union-Find (α = inverse Ackermann, effectively
constant). Space is O(V) for visited-tracking plus O(V) worst-case for the queue/stack/
recursion depth.

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Recognition framing (how you'd name the right tool out loud, fast):** "Before writing
  any code, I translate the question into one of a handful of shapes: 'shortest path in an
  unweighted graph' means BFS, 'does a path exist / flood-fill a region' means DFS or BFS,
  'is this edge redundant / how many groups' means Union-Find, and 'is there a valid
  ordering under dependencies' means topological sort. Naming the shape first is what
  keeps me from writing the wrong traversal and discovering it thirty lines in."
- **Mental-model framing (good for explaining why the tools aren't interchangeable):** "I
  think of these four tools as answering four different *questions*, not four different
  *implementations* of the same question — BFS answers 'how far,' DFS answers 'can I get
  there,' Union-Find answers 'are these already connected,' and coloring-based DFS answers
  'is there a cycle in a specific direction.' Picking the wrong one usually still compiles
  and runs — it just answers a question nobody asked."
- **Generalization/teaching framing (good for demonstrating depth beyond one problem):**
  "The deepest reusable idea here is that a 'graph' doesn't have to be handed to you as an
  adjacency list — grids are graphs with implicit 4-directional edges, and Word Ladder's
  words-as-nodes graph is generated lazily via wildcard patterns. I'd walk through that
  reframing explicitly, since recognizing an implicit graph is often the actual hard part,
  not the traversal itself."

### Vocabulary Builder

- **adjacency list** (n. phrase) — a mapping from each node to its list of neighbors; the
  standard explicit representation a graph is traversed over, whether given directly or
  built on the fly.
- **amortized** (adj.) — a cost averaged over a sequence of operations rather than any
  single one; Union-Find's near-O(1) operations are amortized, not worst-case-O(1) per call.
- **degenerate case** (n. phrase) — a trivial but valid edge case, such as a graph with one
  node and no edges, or a start node equal to the target — worth naming to show boundaries
  were checked, not just the typical case.
- **"…is a graph problem in disguise"** — a reusable phrase for the moment a problem's
  surface form (grid, word list, dependency list) isn't obviously graph-shaped, but the
  underlying structure is — naming the disguise is what signals pattern recognition.
