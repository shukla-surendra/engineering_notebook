# 2. Spiral Matrix

**Difficulty:** Medium
**Topic:** Math & Geometry
**Pattern:** Shrinking boundary traversal

## Problem
Given an `m x n` matrix, return all elements in spiral order (clockwise, from the
outside in).

## Examples
```
Input: [[1,2,3],[4,5,6],[7,8,9]] -> [1,2,3,6,9,8,7,4,5]
```

## Approach
Maintain four boundaries: `top`, `bottom`, `left`, `right`. Traverse right along `top`,
down along `right`, left along `bottom`, up along `left` — then shrink each boundary
inward after completing its side (`top += 1`, `right -= 1`, etc.). After each of the
bottom and left traversals, re-check that `top <= bottom` / `left <= right` before
proceeding, since a non-square matrix can exhaust a dimension mid-spiral. Repeat until the
boundaries cross.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Shrinking boundary traversal**, which itself
belongs to the broader **In-Place Matrix Manipulation** family of techniques. If the
specific trick above feels like it came out of nowhere, that's the signal to step back
and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of
problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(m·n)
- Space: O(1) extra (excluding output)

## Solution
Runnable, with sample test cases at the bottom (`python3 math_geometry/02_spiral_matrix/solution.py`):

```python
--8<-- "math_geometry/02_spiral_matrix/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "I'd rule out trying to
  compute a single closed-form 'index n of the spiral' formula up front — it's technically
  possible but error-prone to derive live. Instead I'd say the standard approach is
  simulation: track four shrinking boundaries and walk each edge in turn, which is easier
  to get right under pressure than a formula I have to re-derive from scratch."
- **Invariant framing (good for explaining the extra boundary checks):** "The invariant is
  `top <= bottom and left <= right` defines 'there's still unvisited matrix left.' The
  reason I re-check `top <= bottom` and `left <= right` *between* the bottom and left edge
  walks specifically is that a non-square matrix can exhaust one dimension mid-spiral —
  skipping that check would revisit or overrun already-collapsed rows or columns."
- **Generalization framing (good for showing this isn't spiral-specific):** "Shrinking
  boundary traversal generalizes to any 'walk the matrix layer by layer from the outside
  in' problem — it's the same four-boundary skeleton whether the traversal order is
  clockwise spiral or something else. I'd point to that as the reusable template rather
  than a spiral-only trick."

### Vocabulary Builder

- **shrinking boundary** (n. phrase) — maintaining four indices (top, bottom, left, right)
  that move inward as each edge of the current layer is consumed.
- **degenerate case** (n. phrase) — a matrix with one row, one column, or a single cell —
  worth naming explicitly, since a spiral traversal that's only tested on square matrices
  often breaks on these.
- **simulation** (n.) — directly executing the process described by the problem step by
  step, as opposed to deriving a closed-form shortcut; often the more reliable choice under
  interview time pressure.
- **"…breaks down when…"** — reusable phrase for flagging exactly where a simplification
  fails, e.g. "the naive single-loop version breaks down when the matrix isn't square,
  because one dimension exhausts before the other."
