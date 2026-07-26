# Pattern: In-Place Matrix Manipulation

## What problem does this solve?

A cluster of matrix problems ask you to transform a 2-D grid (rotate it, traverse it in a
particular order, zero out rows/columns) under an implicit or explicit constraint: do it
**in place**, without allocating a second matrix. The interesting part of these problems is
almost always a small geometric or bookkeeping insight that avoids the naive "copy into a
new matrix" approach, not an advanced algorithm.

## How to recognize it

Signals for each sub-technique:
- **"Rotate the matrix in place"** → decompose the rotation into two simpler, well-known
  in-place operations (transpose + reverse) rather than trying to compute the rotated
  position formula directly cell by cell.
- **"Traverse in spiral/diagonal/zigzag order"** → track shrinking boundaries
  (top/bottom/left/right) and walk each edge in turn, rather than trying to compute a
  single closed-form index formula for the nth element of the spiral.
- **"Some marker condition (a zero, a flagged cell) should propagate to its whole row/
  column"** → use part of the matrix itself (often the first row/column) as free storage
  for markers, instead of allocating a separate boolean grid.

## The general techniques

**Transpose + reverse (90° rotation)** — a clockwise rotation is mathematically
`transpose` then `reverse each row` (or `reverse each row` then `transpose` for
counter-clockwise):
```python
n = len(matrix)
for i in range(n):
    for j in range(i + 1, n):
        matrix[i][j], matrix[j][i] = matrix[j][i], matrix[i][j]   # transpose (swap across diagonal)
for row in matrix:
    row.reverse()                                                   # then reverse each row
```
Internalizing this decomposition means you never have to re-derive "where does cell (r,c)
go after rotation" under time pressure — you just apply the two named steps.

**Shrinking boundary traversal (spiral order)**:
```python
top, bottom, left, right = 0, rows - 1, 0, cols - 1
while top <= bottom and left <= right:
    walk top row left-to-right;      top += 1
    walk right col top-to-bottom;    right -= 1
    if top <= bottom: walk bottom row right-to-left;  bottom -= 1
    if left <= right: walk left col bottom-to-top;    left += 1
```
The two `if` guards before the bottom-row and left-column walks matter specifically for
non-square matrices, where one dimension can be exhausted mid-spiral before the other.

**Using the matrix itself as marker storage (Set Matrix Zeroes)**: instead of allocating an
`m x n` boolean grid to remember which rows/columns need zeroing, repurpose the matrix's
own first row and first column as that storage (with two extra flags for whether row 0 /
column 0 themselves originally contained a zero, since they double as storage and would
otherwise lose that information). This turns an O(m·n) *space* solution into O(1) extra
space, at the cost of a slightly more careful multi-pass implementation.

## Common pitfalls

- Trying to compute the rotated/spiral index with a single formula under time pressure
  instead of using the named decomposition (transpose+reverse, shrinking boundaries) —
  much more error-prone to derive from scratch than to apply a memorized template.
- For spiral traversal on non-square or single-row/column matrices, forgetting the
  `top <= bottom` / `left <= right` re-checks between the four edge walks, causing
  duplicate or out-of-bounds entries.
- For Set Matrix Zeroes, processing the marking pass and the zeroing pass in the wrong
  order (or over the wrong range), corrupting the very markers you're relying on — process
  the first row/column's own flags *last*, after using them to zero everything else.

## Complexity characteristics

O(m·n) time (every cell touched a constant number of times), and — the entire point of
this topic — O(1) *extra* space, achieved through geometric decomposition or repurposing
existing storage rather than allocating a second grid.

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Recognition framing (how you'd name the pattern out loud, fast):** "Whenever a matrix
  problem says or implies 'in place,' I stop looking for a clever single-pass formula and
  ask instead: can I decompose this into named, simpler in-place steps (transpose +
  reverse), track shrinking boundaries (spiral), or repurpose part of the structure itself
  as storage (marker reuse)? Naming which of those three shapes applies is most of the
  work before any code."
- **Mental-model framing (good for explaining the unifying idea across sub-techniques):**
  "I think of every problem in this family as answering the same underlying question: 'what
  free space already exists in this structure that I can exploit instead of allocating a
  new one?' Sometimes that's geometric — two operations compose into the transform I need —
  and sometimes it's literal — the first row and column are memory I'm not otherwise using."
- **Generalization/teaching framing (good for demonstrating depth beyond one problem):**
  "The insight worth calling out explicitly is that these problems are rarely
  algorithmically hard — they're bookkeeping-hard. The interesting content is the ordering
  of operations (transpose before reverse, mark before overwrite, process (1,1) before row
  0), not an unfamiliar algorithm, so I'd narrate the ordering decisions out loud rather
  than rushing past them."

### Vocabulary Builder

- **in-place** (adj.) — an algorithm using O(1) extra space beyond the input itself,
  mutating the given structure rather than building a new one.
- **decomposition** (n.) — breaking a single complex transform into a sequence of simpler,
  well-known operations that compose to the same result; the core move behind rotation via
  transpose + reverse.
- **auxiliary space** (n. phrase) — memory used beyond the input and output; minimizing
  this, not runtime, is usually the actual constraint driving the "trick" in this pattern
  family.
- **"…exploit existing structure instead of allocating new"** — a reusable phrase for
  describing the shared insight across this pattern family, applicable whether the "existing
  structure" is the matrix's own first row/column or a geometric relationship between rows
  and columns.
