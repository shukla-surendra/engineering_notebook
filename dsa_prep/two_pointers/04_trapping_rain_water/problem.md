# 4. Trapping Rain Water

**Difficulty:** Hard
**Topic:** Two Pointers
**Pattern:** Two Pointers (converging, bounded by running max)

## Problem
Given `n` non-negative integers `height[i]` representing an elevation map where the width
of each bar is 1, compute how much water it can trap after raining.

## Examples
```
Input: height = [0,1,0,2,1,0,1,3,2,1,2,1] -> 6
Input: height = [4,2,0,3,2,5]              -> 9
```

## Approach
The water trapped above any single bar `i` is
`min(max(height[0..i]), max(height[i..n-1])) - height[i]` — bounded by the shorter of the
tallest wall to its left and the tallest wall to its right. Computing both max arrays up
front gives an O(n) time, O(n) space solution, but two pointers gets the same time bound
in O(1) space.

Start pointers at both ends and track `left_max`/`right_max`, the tallest bar seen so far
from each side. Whichever side currently has the **shorter** height is the one whose water
level is already fully determined: if `height[left] < height[right]`, then no matter what
lies between them, some bar at or before `right` is at least `height[left]` tall (namely
`right` itself), so the right-side bound for position `left` is already at least
`left_max`. That means `left_max` alone determines the trapped water at `left`, whichever
of `left_max`/`right_max` is smaller is safe to finalize now — advance that pointer and
add `max(0, running_max - height[pointer])` to the total.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Two Pointers (converging, bounded by running
max)**, itself an instance of the broader **Two Pointers** family covered in
[`../PATTERN.md`](../PATTERN.md). It's also a direct extension of
[`../03_container_with_most_water/problem.md`](../03_container_with_most_water/problem.md):
both problems converge two pointers inward and both rely on a dominance argument about
which side to advance. The difference is what each problem optimizes — Container With Most
Water wants the single best *pair* of walls (so it discards the shorter wall's position
entirely once compared), while Trapping Rain Water wants a *running total* contributed by
every position (so it can't discard anything — it finalizes one position's contribution
per step instead). If the "move the shorter side" instinct from Container feels familiar
here, that's the right instinct; what's new is realizing *why* it's safe to commit to an
answer for that position immediately, rather than just safe to keep searching.

## Complexity
- Time: O(n)
- Space: O(1)

## Solution
Runnable, with sample test cases at the bottom (`python3 two_pointers/04_trapping_rain_water/solution.py`):

```python
--8<-- "two_pointers/04_trapping_rain_water/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The naive version recomputes,
  for every index, the tallest wall to its left and the tallest wall to its right by
  rescanning the array each time — O(n²). The first optimization is precomputing both max
  arrays in a single left-to-right and right-to-left pass, which gets to O(n) time but costs
  O(n) extra space. I'd name both of those out loud before mentioning two pointers, since
  the two-pointer trick is really 'keep the same O(n) time, drop the space to O(1)' — a
  space optimization on top of an already-correct O(n)/O(n) solution, not a different idea."
- **Invariant framing (good for justifying why it's safe to finalize a position
  immediately):** "The invariant I'd state out loud is: whichever side currently has the
  smaller running max is the side whose water level is already decided, because the other
  side is guaranteed to have *some* wall at least that tall standing between the pointers.
  That's what lets me commit to `left_max - height[left]` as final the moment I look at it,
  instead of needing both full max arrays before I can trust any single value."
- **Generalization framing (good for connecting this to its sibling problem):** "This is
  the same converging two-pointer skeleton as Container With Most Water, and the 'advance
  the smaller side' move even looks identical — but I'd flag the difference explicitly:
  Container discards the shorter side's position after comparing it, because it only cares
  about the single best pair, while this problem can't discard anything, because every
  position contributes to the running total. Same movement rule, different reason it's
  safe, different thing being computed."

### Vocabulary Builder

- **prefix max / suffix max** (n. phrase) — the tallest value seen so far scanning from the
  left (or right) up to a given index; the O(n)-space brute force that two pointers
  compresses into two running scalars.
- **running max** (n. phrase) — a single value updated in place as pointers move, replacing
  a precomputed array; the mechanism that lets this solution drop from O(n) to O(1) space.
- **"…a space optimization on top of an already-correct solution"** — a reusable phrase for
  framing a two-pointer trick as refining a working O(n) approach rather than inventing the
  algorithm from scratch, which reads as more confident than presenting it as the only path.
- **lower bound** (n. phrase) — a guaranteed minimum value; here, the far wall on the taller
  side is a lower bound on that side's true max, which is precisely what makes committing
  to the shorter side's contribution safe before scanning the rest of the array.

