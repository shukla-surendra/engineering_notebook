# 2. Jump Game

**Difficulty:** Medium
**Topic:** Greedy
**Pattern:** Track the farthest reachable index

## Problem
Given an array `nums` where `nums[i]` is the maximum jump length from index `i`,
determine if you can reach the last index starting from index 0.

## Examples
```
Input: nums = [2,3,1,1,4] -> True
Input: nums = [3,2,1,0,4] -> False (stuck at index 3)
```

## Approach
Walk left to right, maintaining `farthest` — the furthest index reachable so far. At each
index `i`, if `i > farthest`, that index is unreachable, so fail immediately. Otherwise
update `farthest = max(farthest, i + nums[i])`. If `farthest` ever reaches or passes the
last index, succeed. This greedy scan never needs to backtrack because reachability is
monotonic — the furthest-so-far is always the best possible position to jump from.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Track the farthest reachable index**, which
itself belongs to the broader **Greedy** family of techniques. If the specific trick
above feels like it came out of nowhere, that's the signal to step back and read
[`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of problems in
general (not just this one), the reusable template you can write from memory, the usual
variations, and the mistakes people make applying it. Coming back to re-read this
problem's approach afterward should make the specific choices here feel inevitable
rather than clever.

## Complexity
- Time: O(n)
- Space: O(1)

## Solution
Runnable, with sample test cases at the bottom (`python3 greedy/02_jump_game/solution.py`):

```python
--8<-- "greedy/02_jump_game/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The brute force is
  backtracking — try every jump length from every index — which is exponential in the
  worst case. I'd say that out loud, then pivot: I don't actually need to know *which*
  sequence of jumps works, only whether the farthest reachable index ever falls behind my
  current position, which turns this into a single O(n) scan."
- **Monotonicity framing (good for justifying why one pass is enough):** "The quantity I'm
  tracking, `farthest`, is monotonically non-decreasing as I scan left to right — it never
  needs to shrink or be reconsidered. That's the 'no benefit to waiting' argument: if index
  i is unreachable, no amount of extra scanning fixes that, so I can fail fast the moment
  `i > farthest`."
- **Generalization framing (good for signaling pattern recognition):** "This is the
  'track the farthest reachable index' pattern — I'd name it explicitly, since it's the
  same frontier-tracking idea behind Jump Game II (minimum jumps) and several interval-
  reachability problems, just with a different quantity being maximized at each step."

### Vocabulary Builder

- **monotonic** (adj.) — never decreasing (or never increasing) as you move through a
  sequence; `farthest` is monotonic here, which is exactly what licenses a single forward
  pass with no backtracking.
- **infeasible** (adj.) — impossible to satisfy given the constraints; useful for
  precisely describing why the scan bails out the instant an index is unreachable, rather
  than saying something vaguer like "broken."
- **"…which turns this into a single pass"** — a reusable phrase for the moment a greedy
  insight collapses an apparently combinatorial search into linear-time scanning.
- **frontier** (n.) — the boundary of what's currently reachable/known; a natural way to
  describe `farthest` without repeating the variable name verbatim.
