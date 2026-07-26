# 3. Container With Most Water

**Difficulty:** Medium
**Topic:** Two Pointers
**Pattern:** Two Pointers (greedy inward move)

## Problem
Given `n` non-negative integers `height[i]` representing vertical lines on the x-axis,
find two lines that together with the x-axis form a container that holds the most water.
Return the maximum area.

## Examples
```
Input: height = [1,8,6,2,5,4,8,3,7] -> 49
Input: height = [1,1]               -> 1
```

## Approach
Start with two pointers at the far left and far right — the widest possible container.
Area is `min(height[left], height[right]) * (right - left)`. Moving the taller pointer
inward can never increase the area (width shrinks, height is capped by the *shorter* side
either way), so always move the pointer at the **shorter** line inward. This greedily
explores only the moves that could possibly improve the answer.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Two Pointers (greedy inward move)**, which
itself belongs to the broader **Two Pointers** family of techniques. If the specific
trick above feels like it came out of nowhere, that's the signal to step back and read
[`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of problems in
general (not just this one), the reusable template you can write from memory, the usual
variations, and the mistakes people make applying it. Coming back to re-read this
problem's approach afterward should make the specific choices here feel inevitable
rather than clever.

## Complexity
- Time: O(n)
- Space: O(1)

## Solution
Runnable, with sample test cases at the bottom (`python3 two_pointers/03_container_with_most_water/solution.py`):

```python
--8<-- "two_pointers/03_container_with_most_water/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The brute force checks every
  pair of lines — O(n²) — and I'd say that number out loud immediately, since it's the
  baseline the two-pointer approach has to beat, and naming it first makes the O(n) jump
  land as a deliberate insight instead of a memorized trick."
- **Invariant framing (good for justifying the greedy move, which is the crux of this
  problem):** "The move that needs defending is: always move the *shorter* line inward.
  I'd state the proof out loud — width only shrinks as the pointers close in, and the area
  is capped by the shorter of the two lines regardless of which one I move, so moving the
  taller line can never do better than what I've already computed, while moving the
  shorter one is the only move that has a chance of finding something taller. That's a
  strict dominance argument, not a heuristic."
- **Generalization framing (good for connecting to the pattern family):** "This is
  converging two pointers with a greedy elimination rule, the same skeleton as the
  palindrome check but with a *why-is-this-move-safe* argument attached instead of a
  simple equality check. I'd flag that whenever a two-pointer move needs a correctness
  argument, not just an intuition, that argument is worth stating explicitly in an
  interview."

### Vocabulary Builder

- **strict dominance** (n. phrase) — when one choice is never worse and sometimes better
  than an alternative, making the alternative safe to discard entirely; the formal
  justification for always moving the shorter line.
- **greedy** (adj.) — describing an algorithm that makes the locally-best choice at each
  step without reconsidering it later, relying on a proof that local optimality leads to
  global optimality.
- **"…the crux of this problem"** — a reusable phrase for signaling you're about to state
  the one insight that makes the rest of the solution follow mechanically.
- **search space** (n. phrase) — the full set of candidate solutions being considered
  (here, all O(n²) pairs); the greedy elimination rule shrinks this space by one pointer
  move per step instead of exploring it exhaustively.
