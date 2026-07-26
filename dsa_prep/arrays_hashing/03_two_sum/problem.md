# 3. Two Sum

**Difficulty:** Easy
**Topic:** Arrays & Hashing
**Pattern:** Hash Map (complement lookup)

## Problem
Given an array of integers `nums` and an integer `target`, return the **indices** of the
two numbers that add up to `target`. Exactly one valid answer exists; you may not use the
same element twice.

## Examples
```
Input: nums = [2,7,11,15], target = 9 -> [0,1]
Input: nums = [3,2,4], target = 6     -> [1,2]
```

## Approach
Brute force checks every pair — O(n²). Instead, walk the array once, keeping a hash map of
`value -> index` seen so far. At each element, check whether `target - nums[i]` is already
in the map; if so, we've found the pair in O(n).

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Hash Map (complement lookup)**, which itself
belongs to the broader **Hashing for O(1) Lookups** family of techniques. If the
specific trick above feels like it came out of nowhere, that's the signal to step back
and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of
problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(n)
- Space: O(n)

## Solution
Runnable, with sample test cases at the bottom (`python3 arrays_hashing/03_two_sum/solution.py`):

```python
--8<-- "arrays_hashing/03_two_sum/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the standard opener for this exact problem):** "Brute force
  checks every pair, O(n²), and I'd say that out loud before writing a line of code. Then
  I'd pivot: instead of searching for the complement, I can remember what I've seen and
  look it up in O(1), collapsing the whole thing to a single O(n) pass."
- **Invariant framing (for explaining the single hash map pass precisely):** "At index `i`,
  the map contains every index before `i` — nothing at or after. So checking `target -
  nums[i]` against the map first, then inserting `nums[i]` afterward, guarantees I never
  pair an element with itself."
- **Generalization framing (ties it to the broader family):** "This is the canonical
  complement-lookup problem — the same shape shows up any time you're asked 'does some
  earlier value combine with the current one to satisfy a condition,' which is worth naming
  since it's the template for a whole class of problems."

### Vocabulary Builder

- **complement** (n.) — the value that, combined with the current element, satisfies the
  target condition; here, `target - nums[i]`. *"I look up the complement in the map before
  inserting the current value."*
- **single-pass** (adj.) — an algorithm that only walks the input once; a strong signal of
  O(n) when paired with O(1) lookups per step.
- **"trades memory for speed"** — a compact, reusable phrase for justifying any hash-based
  optimization over a brute-force scan.
- **exactly-one-solution guarantee** (n. phrase) — naming that the problem promises a
  unique answer, which is why we can return immediately on the first match rather than
  collecting all pairs.
