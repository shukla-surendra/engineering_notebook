# 4. Koko Eating Bananas

**Difficulty:** Medium
**Topic:** Binary Search
**Pattern:** Binary search on the answer space

## Problem
Koko has `piles` of bananas and `h` hours before the guards return. Each hour she chooses
some pile and eats up to `k` bananas from it (if the pile has fewer than `k`, she finishes
it and stops for the hour). Find the minimum integer `k` such that she can eat all bananas
within `h` hours.

## Examples
```
Input: piles = [3,6,7,11], h = 8 -> 4
Input: piles = [30,11,23,4,20], h = 5 -> 30
Input: piles = [30,11,23,4,20], h = 6 -> 23
```

## Approach
This isn't searching *in* an array — it's binary searching over the **answer**, `k`,
which ranges from 1 to `max(piles)`. Define `hours_needed(k) = sum(ceil(pile / k) for pile
in piles)`. This function is monotonically non-increasing in `k` (a bigger eating speed
never needs more hours), so binary search for the smallest `k` where `hours_needed(k) <= h`.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Binary search on the answer space**, which
itself belongs to the broader **Binary Search** family of techniques. If the specific
trick above feels like it came out of nowhere, that's the signal to step back and read
[`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of problems in
general (not just this one), the reusable template you can write from memory, the usual
variations, and the mistakes people make applying it. Coming back to re-read this
problem's approach afterward should make the specific choices here feel inevitable
rather than clever.

## Complexity
- Time: O(n · log(max(piles)))
- Space: O(1)

## Solution
Runnable, with sample test cases at the bottom (`python3 binary_search/04_koko_eating_bananas/solution.py`):

```python
--8<-- "binary_search/04_koko_eating_bananas/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Reframing framing (the insight that unlocks this problem):** "The array `piles` isn't
  what I binary search over — I binary search over the *answer*, the eating speed `k`,
  which ranges from 1 to `max(piles)`. That's the jump from 'binary search on data' to
  'binary search on a decision variable,' and naming it explicitly is what shows I
  recognize the pattern rather than stumbling into it."
- **Monotonicity framing (the correctness argument to state before coding):** "I'd state
  and briefly justify monotonicity before writing the loop: a faster eating speed can never
  *increase* hours needed, so `hours_needed(k)` is non-increasing in `k`. Without that
  property, binary search on the answer would silently give a wrong result, so it's worth
  saying out loud, not assuming."
- **Feasibility-function framing (how to structure the actual code):** "I write
  `feasible(k)` first, as its own function, convince myself it's monotonic, and only then
  write the search loop — the loop itself is boilerplate I want to spend zero creative
  energy on."

### Vocabulary Builder

- **monotonic** (adj.) — consistently non-increasing or non-decreasing as its input grows;
  the property that makes binary-search-on-the-answer valid at all. *"hours_needed(k) is
  monotonically non-increasing in k, which is what lets me binary search over k safely."*
- **feasibility predicate** (n. phrase) — a yes/no function of a candidate answer (here,
  "can she finish within h hours at speed k?") that binary search narrows in on.
- **"binary search on the answer" / "binary search on the answer space"** — the standard
  term of art for searching a derived decision variable rather than array indices; using it
  by name signals fluency beyond textbook binary search.
- **ceiling division** (n. phrase) — `ceil(pile / k)`, needed because Koko can't eat a
  fractional hour's worth of a pile; a common source of off-by-one bugs if implemented as
  integer division without adjustment.
