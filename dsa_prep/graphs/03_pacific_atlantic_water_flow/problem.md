# 3. Pacific Atlantic Water Flow

**Difficulty:** Medium
**Topic:** Graphs
**Pattern:** Multi-source DFS/BFS from the borders, run backward

## Problem
Given an `m x n` grid of heights, water can flow from a cell to a neighbor with height
`<=` its own, in 4 directions. The Pacific touches the top/left edges, the Atlantic the
bottom/right edges. Return all cells from which water can reach **both** oceans.

## Examples
```
Input: heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
Output: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
```

## Approach
Instead of checking, for every cell, whether it can flow all the way to both borders
(expensive), reverse the direction: start a multi-source DFS/BFS **from every border
cell** of each ocean, moving to a neighbor only if that neighbor's height is `>=` the
current cell's (this is water flowing "uphill" in reverse, equivalent to "could flow down
to here"). This marks every cell that ocean's water can be traced back from. A cell in
both the Pacific-reachable and Atlantic-reachable sets is part of the answer.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Multi-source DFS/BFS from the borders, run
backward**, which itself belongs to the broader **Graph Traversal (BFS, DFS, Union-Find,
Topological Sort)** family of techniques. If the specific trick above feels like it came
out of nowhere, that's the signal to step back and read [`../PATTERN.md`](../PATTERN.md)
— it covers how to recognize this family of problems in general (not just this one), the
reusable template you can write from memory, the usual variations, and the mistakes
people make applying it. Coming back to re-read this problem's approach afterward should
make the specific choices here feel inevitable rather than clever.

## Complexity
- Time: O(m·n)
- Space: O(m·n)

## Solution
Runnable, with sample test cases at the bottom (`python3 graphs/03_pacific_atlantic_water_flow/solution.py`):

```python
--8<-- "graphs/03_pacific_atlantic_water_flow/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The direct reading of the
  problem is: for every cell, can water starting there reach the Pacific border and,
  separately, the Atlantic border? That's a traversal from each of up to m·n cells, so
  O(m²n²) worst case — I'd name that cost explicitly, then say the fix is to run the
  search backward from the oceans instead of forward from every cell."
- **Invariant framing (good for justifying the reversed comparison):** "Once I reverse
  direction, the invariant flips too: I'm no longer asking 'can I flow downhill from here,'
  I'm asking 'could water have flowed downhill *into* here,' which means I move to a
  neighbor only if that neighbor's height is `>=` mine. Getting that inequality backward is
  the single most common bug in this problem, so I'd say it out loud before coding it."
- **Generalization framing (good for naming the reusable trick):** "This is multi-source
  BFS/DFS — seeding the traversal from an entire border simultaneously instead of from one
  cell — which turns an O(cells × traversal) problem into O(one traversal per ocean). I'd
  name that as the same trick used anywhere you'd otherwise repeat a search from many
  starting points that all converge on the same target region."

### Vocabulary Builder

- **multi-source** (adj.) — a traversal seeded from many starting nodes at once rather
  than one; here, every Pacific-adjacent border cell is pushed onto the queue/stack before
  the search even begins.
- **reachability** (n.) — whether a path exists between two nodes at all, independent of
  path length or cost; this problem only needs reachability, not shortest path, which is
  why plain DFS/BFS (not weighted search) suffices.
- **set intersection** (n. phrase) — the final step of combining "reachable from Pacific"
  and "reachable from Atlantic" into cells present in both, which is exactly `pacific &
  atlantic` in set terms.
- **"…run it backward"** — a compact phrase for the reverse-search trick: instead of
  asking "where can I go from here," ask "who could have reached here," and search from
  the destination(s) instead of every possible origin.
