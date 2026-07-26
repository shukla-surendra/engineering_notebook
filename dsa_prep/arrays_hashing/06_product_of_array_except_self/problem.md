# 6. Product of Array Except Self

**Difficulty:** Medium
**Topic:** Arrays & Hashing
**Pattern:** Prefix / Suffix products

## Problem
Given an integer array `nums`, return an array `answer` such that `answer[i]` is the
product of all elements of `nums` except `nums[i]`. Must run in O(n) **without division**,
and (follow-up) in O(1) extra space excluding the output array.

## Examples
```
Input: nums = [1,2,3,4] -> [24,12,8,6]
Input: nums = [-1,1,0,-3,3] -> [0,0,9,0,0]
```

## Approach
`answer[i]` = (product of everything to the left of i) × (product of everything to the
right of i). Compute prefix products left-to-right into the output array, then walk
right-to-left multiplying in a running suffix product. This avoids division entirely and
uses only the output array plus one running variable.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Prefix / Suffix products**, which itself
belongs to the broader **Hashing for O(1) Lookups** family of techniques. If the
specific trick above feels like it came out of nowhere, that's the signal to step back
and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of
problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(n)
- Space: O(1) extra (output array doesn't count)

## Solution
Runnable, with sample test cases at the bottom (`python3 arrays_hashing/06_product_of_array_except_self/solution.py`):

```python
--8<-- "arrays_hashing/06_product_of_array_except_self/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Constraint-driven framing (name the constraints before the solution):** "Two
  constraints shape this: no division, and ideally O(1) extra space. I'd say that up front,
  because it rules out the 'multiply everything, then divide by nums[i]' shortcut before I
  even try it — division breaks on zeros anyway."
- **Decomposition framing (the core insight, stated plainly):** "`answer[i]` decomposes
  into two independent pieces — the product of everything strictly to the left, and
  everything strictly to the right. Computing each with a single pass, in opposite
  directions, avoids ever recomputing a product from scratch."
- **Space-optimization framing (for the O(1)-extra-space follow-up):** "I'd first write it
  with two separate prefix and suffix arrays to get the idea right, then say: since I only
  ever need one of them at a time during the second pass, I can collapse the suffix array
  into a single running variable and write directly into the output array."

### Vocabulary Builder

- **prefix product** (n. phrase) — the running product of all elements up to (not
  including) index i, computed left to right; the mirror concept, suffix product, runs
  right to left.
- **in-place** (adj.) — reusing the output array itself as scratch space during
  computation, rather than allocating new auxiliary arrays. *"I write the prefix products
  in place, then fold in suffix products on the second pass."*
- **"trades a second array for a single running variable"** — a precise way to describe
  the O(n)-space-to-O(1)-extra-space optimization here.
- **degenerate case** (n. phrase) — worth naming for this problem specifically: multiple
  zeros in the input force every output except at most one position to zero, which a
  division-based approach handles badly.
