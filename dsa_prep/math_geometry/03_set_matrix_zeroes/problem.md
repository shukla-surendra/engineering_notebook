# 3. Set Matrix Zeroes

**Difficulty:** Medium
**Topic:** Math & Geometry
**Pattern:** Use the matrix's own first row/column as marker storage

## Problem
Given an `m x n` matrix, if an element is 0, set its entire row and column to 0. Do it
**in place**.

## Examples
```
Input: [[1,1,1],[1,0,1],[1,1,1]] -> [[1,0,1],[0,0,0],[1,0,1]]
```

## Approach
Naively zeroing as you scan would cascade incorrectly (newly-zeroed cells would trigger
more zeroing). Instead, use the matrix's own first row and first column as marker arrays:
for each zero found at `(r, c)` with `r, c > 0`, mark `matrix[r][0] = 0` and
`matrix[0][c] = 0`. Handle row 0 and column 0 themselves with two separate boolean flags
(since they double as storage). After marking, do a second pass setting `matrix[r][c] = 0`
wherever `matrix[r][0] == 0` or `matrix[0][c] == 0` (working from `(1,1)` onward first, so
the markers aren't corrupted early), then finally zero row 0 / column 0 based on the flags.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Use the matrix's own first row/column as marker
storage**, which itself belongs to the broader **In-Place Matrix Manipulation** family
of techniques. If the specific trick above feels like it came out of nowhere, that's the
signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to
recognize this family of problems in general (not just this one), the reusable template
you can write from memory, the usual variations, and the mistakes people make applying
it. Coming back to re-read this problem's approach afterward should make the specific
choices here feel inevitable rather than clever.

## Complexity
- Time: O(m·n)
- Space: O(1)

## Solution
Runnable, with sample test cases at the bottom (`python3 math_geometry/03_set_matrix_zeroes/solution.py`):

```python
--8<-- "math_geometry/03_set_matrix_zeroes/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The naive approach is to scan
  for zeroes, record their (row, col) coordinates in a separate set, then do a second pass
  zeroing based on that set — O(m·n) space. I'd present that first as clearly correct, then
  say the follow-up optimization is realizing I don't need a *new* boolean grid at all —
  the matrix's own first row and column can serve as that storage."
- **Invariant framing (good for explaining the multi-pass ordering):** "The invariant I
  have to protect is that the marker cells (`matrix[r][0]`, `matrix[0][c]`) aren't zeroed
  out by the second pass before I've finished reading them. That's why I process from
  `(1,1)` onward first and handle row 0 / column 0 themselves *last*, using two separate
  boolean flags captured before any mutation — get that ordering wrong and I'd corrupt the
  very markers the algorithm depends on."
- **Generalization framing (good for showing this isn't matrix-specific):** "The reusable
  idea is 'repurpose part of the existing structure as marker storage instead of
  allocating a new one' — it trades a slightly trickier multi-pass implementation for O(1)
  extra space. I'd name that trade explicitly as the same family of thinking behind other
  in-place matrix problems, not a trick unique to zeroes."

### Vocabulary Builder

- **auxiliary storage** (n. phrase) — extra space allocated beyond the input itself (e.g.
  a separate boolean grid); the thing this technique specifically avoids.
- **propagate** (v.) — to spread an effect outward from its origin; here, a zero at
  `(r, c)` propagates to its entire row and column.
- **cascading** (adj.) — an error where an earlier mutation incorrectly influences a later
  decision in the same pass; naively zeroing while scanning cascades, because newly-zeroed
  cells then falsely trigger more zeroing.
- **"…trades memory for a trickier implementation"** — reusable phrase for justifying the
  marker-reuse trick: O(1) space is gained at the cost of needing a carefully ordered,
  multi-pass implementation instead of one straightforward pass.
