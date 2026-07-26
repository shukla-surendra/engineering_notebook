# 1. Rotate Image

**Difficulty:** Medium
**Topic:** Math & Geometry
**Pattern:** In-place matrix transform: transpose + reverse rows

## Problem
Given an `n x n` 2D matrix `matrix`, rotate it 90 degrees clockwise **in place**.

## Examples
```
Input: [[1,2,3],[4,5,6],[7,8,9]] -> [[7,4,1],[8,5,2],[9,6,3]]
```

## Approach
A 90-degree clockwise rotation decomposes into two simpler in-place steps: (1) transpose
the matrix (swap `matrix[i][j]` with `matrix[j][i]` for `i < j`), then (2) reverse each
row. Doing both achieves the rotation without needing an auxiliary matrix. (Rotating layer
by layer, swapping 4 cells at a time, is an equivalent alternative that avoids the
transpose step.)

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **In-place matrix transform: transpose + reverse
rows**, which itself belongs to the broader **In-Place Matrix Manipulation** family of
techniques. If the specific trick above feels like it came out of nowhere, that's the
signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to
recognize this family of problems in general (not just this one), the reusable template
you can write from memory, the usual variations, and the mistakes people make applying
it. Coming back to re-read this problem's approach afterward should make the specific
choices here feel inevitable rather than clever.

## Complexity
- Time: O(n²)
- Space: O(1)

## Solution
Runnable, with sample test cases at the bottom (`python3 math_geometry/01_rotate_image/solution.py`):

```python
--8<-- "math_geometry/01_rotate_image/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The obvious approach is to
  allocate a new n×n matrix and copy `matrix[i][j]` into `result[j][n-1-i]` directly — O(n²)
  time and space. I'd name that first, then say the in-place constraint is really asking
  for a *decomposition*: two well-known operations, transpose and row-reversal, composed
  together, instead of one clever index formula."
- **Invariant framing (good for explaining why order (transpose, then reverse) matters):**
  "Transpose alone gives a counter-clockwise-ish reflection across the diagonal, not the
  rotation; the invariant I'm relying on is that transpose-then-reverse-rows is
  mathematically equivalent to a 90° clockwise rotation for every cell, not just the ones I
  happened to test. Reversing the order of those two steps produces a different, wrong
  transform, so I'd state the decomposition by name rather than trying to re-derive it live."
- **Generalization framing (good for showing this isn't a one-off trick):** "This belongs
  to the in-place matrix manipulation family, where the recurring insight is 'decompose a
  spatial transform into two simpler, named, in-place steps' rather than compute a closed
  form per cell. I'd reference that family explicitly if asked to handle a related
  transform, like counter-clockwise rotation or a 180° rotation."

### Vocabulary Builder

- **transpose** (n./v.) — reflecting a matrix across its main diagonal, swapping
  `matrix[i][j]` with `matrix[j][i]`. *"Transposing first is what makes the row-reversal
  step produce a clean 90-degree rotation."*
- **in-place** (adj.) — modifying a data structure using only O(1) extra space, without
  allocating an auxiliary copy.
- **decomposition** (n.) — breaking a complex transform into a sequence of simpler, named
  operations; the core technique this whole problem family relies on.
- **"…is the load-bearing detail"** — reusable phrase for flagging the one design choice
  an approach depends on, e.g. "the order of transpose-then-reverse is the load-bearing
  detail here."
