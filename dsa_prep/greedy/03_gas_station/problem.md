# 3. Gas Station

**Difficulty:** Medium
**Topic:** Greedy
**Pattern:** Single-pass greedy with a running deficit / total-surplus check

## Problem
There are `n` gas stations in a circle. `gas[i]` is the gas available at station `i`, and
`cost[i]` is the gas needed to travel from station `i` to station `i+1`. Return the
starting station index from which you can complete the circuit, or `-1` if impossible
(the answer is guaranteed unique if it exists).

## Examples
```
Input: gas = [1,2,3,4,5], cost = [3,4,5,1,2] -> 3
Input: gas = [2,3,4], cost = [3,4,3]         -> -1
```

## Approach
If `sum(gas) < sum(cost)`, it's impossible overall — return -1. Otherwise a valid start
always exists, and it can be found in one pass: track a running `tank` as you simulate
from a candidate start; whenever `tank` goes negative at station `i`, none of the stations
from the previous start through `i` could have worked either (they'd only make the deficit
worse arriving at `i`), so reset the candidate start to `i + 1` and reset `tank` to 0. The
final candidate start after one full pass is the answer (guaranteed valid given the total
surplus check passed).

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Single-pass greedy with a running deficit /
total-surplus check**, which itself belongs to the broader **Greedy** family of
techniques. If the specific trick above feels like it came out of nowhere, that's the
signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to
recognize this family of problems in general (not just this one), the reusable template
you can write from memory, the usual variations, and the mistakes people make applying
it. Coming back to re-read this problem's approach afterward should make the specific
choices here feel inevitable rather than clever.

## Complexity
- Time: O(n)
- Space: O(1)

## Solution
Runnable, with sample test cases at the bottom (`python3 greedy/03_gas_station/solution.py`):

```python
--8<-- "greedy/03_gas_station/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "The brute force tries every
  station as a starting point and simulates the full loop — O(n²). The crux of it is that
  simulating from scratch at each candidate throws away information; if I track the
  running deficit instead, I can rule out a whole range of candidates in one pass, getting
  to O(n)."
- **Exchange-argument framing (good for justifying the reset rule):** "Whenever the running
  tank goes negative at station i, that tells me every station between my current
  candidate and i also fails — none of them could have carried enough surplus to survive
  past i either, since they'd hit i with an even smaller tank. So I reset my candidate to
  i+1 without re-checking any of the skipped stations."
- **Generalization framing (good for signaling you see the broader shape):** "This is the
  reset-on-deficit pattern — a single-pass greedy with a running total, closely related to
  Kadane's algorithm on `gas[i] - cost[i]`. Naming that connection shows I recognize it as a
  variant of 'find where a running sum first goes non-negative and stays there,' not a
  one-off trick."

### Vocabulary Builder

- **deficit** (n.) — the amount by which a running total falls short; here, the tank going
  negative at station i is a deficit that invalidates every candidate start up to i.
- **surplus** (n.) — the opposite of deficit; checking `sum(gas) >= sum(cost)` up front is
  a global surplus check that guarantees *some* valid start exists before searching for it.
- **"the crux of it is…"** — a reusable phrase for naming the one insight that makes an
  O(n²) approach collapse to O(n), useful whenever you want to signal you've found the
  real leverage point rather than just optimizing constants.
- **invariant** (n.) — here, "no station between the last reset point and the current
  index can be a valid start" — stating it explicitly is what justifies skipping them
  without individually re-checking each one.
