# 4. Daily Temperatures

**Difficulty:** Medium
**Topic:** Stack
**Pattern:** Monotonic Stack

## Problem
Given an array `temperatures`, return an array `answer` where `answer[i]` is the number of
days you'd have to wait after day `i` for a warmer temperature. If no such day exists,
`answer[i] = 0`.

## Examples
```
Input: temperatures = [73,74,75,71,69,72,76,73] -> [1,1,4,2,1,1,0,0]
```

## Approach
Use a monotonic decreasing stack of **indices** (temperatures at those indices decrease as
you go down the stack). For each new day, pop every index off the stack whose temperature
is lower than today's — today is the "warmer day" they were waiting for, so set
`answer[popped_index] = i - popped_index`. Then push today's index. Every index is pushed
and popped at most once, so this is O(n) overall despite the inner while loop.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Monotonic Stack**, which itself belongs to the
broader **Stack** family of techniques. If the specific trick above feels like it came
out of nowhere, that's the signal to step back and read [`../PATTERN.md`](../PATTERN.md)
— it covers how to recognize this family of problems in general (not just this one), the
reusable template you can write from memory, the usual variations, and the mistakes
people make applying it. Coming back to re-read this problem's approach afterward should
make the specific choices here feel inevitable rather than clever.

## Complexity
- Time: O(n)
- Space: O(n)

## Solution
Runnable, with sample test cases at the bottom (`python3 stack/04_daily_temperatures/solution.py`):

```python
--8<-- "stack/04_daily_temperatures/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The naive approach is, for
  every day, scan forward until a warmer day shows up — O(n²) worst case, e.g. on a
  strictly decreasing array. I'd name that, then say a monotonic stack gets to O(n) by
  answering many days' questions in a single forward pass instead of a nested scan."
- **Invariant framing (good for explaining why it's still O(n) despite the inner while
  loop):** "The invariant is the stack holds indices in decreasing order of temperature.
  When a warmer day appears, I pop every index whose 'waiting question' that day just
  answers — and crucially, each index is pushed once and popped at most once across the
  *entire* run, so the total work across all iterations is O(n), even though any single
  iteration's inner loop looks like it could be expensive."
- **Generalization framing (good for signaling pattern recognition):** "This is the
  textbook monotonic stack — 'find the next greater/smaller element for every position' —
  and I'd name that family explicitly, since it's the same shape as Next Greater Element
  and stock-span-style problems."

### Vocabulary Builder

- **monotonic stack** (n. phrase) — a stack maintained in strictly increasing or
  decreasing order, used to answer 'next greater/smaller element' queries in amortized
  O(n).
- **amortized** (adj.) — a cost averaged across the whole algorithm's run; here, even
  though the inner `while` loop can pop several elements at once, no element is ever
  popped more than once in total, so the amortized per-element cost is O(1).
- **"each element is pushed once and popped at most once"** — the standard proof
  sketch for why a monotonic-stack solution is O(n) despite looking like nested loops —
  worth stating verbatim when asked to justify the complexity.
- **resolve** (v.) — to determine an element's final answer and remove it from further
  consideration; here, popping an index "resolves" its "how many days until warmer"
  question.
