# 4. Hand of Straights

**Difficulty:** Medium
**Topic:** Greedy
**Pattern:** Sort + greedily consume the smallest remaining run

## Problem
Given an array of card values `hand` and a group size `groupSize`, return `True` if the
cards can be rearranged into groups of `groupSize` consecutive values each.

## Examples
```
Input: hand = [1,2,3,6,2,3,4,7,8], groupSize = 3 -> True  ([1,2,3],[2,3,4],[6,7,8])
Input: hand = [1,2,3,4,5], groupSize = 4          -> False
```

## Approach
If `len(hand)` isn't divisible by `groupSize`, fail immediately. Otherwise count card
frequencies. Repeatedly take the smallest remaining card value with count > 0 as the start
of a new group — it *must* start a group of consecutive values, since nothing smaller
remains to place before it. Consume `groupSize` consecutive values from that start,
decrementing their counts (failing if any needed value is missing). A min-heap of distinct
remaining values makes "smallest remaining" efficient.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Sort + greedily consume the smallest remaining
run**, which itself belongs to the broader **Greedy** family of techniques. If the
specific trick above feels like it came out of nowhere, that's the signal to step back
and read [`../PATTERN.md`](../PATTERN.md) — it covers how to recognize this family of
problems in general (not just this one), the reusable template you can write from
memory, the usual variations, and the mistakes people make applying it. Coming back to
re-read this problem's approach afterward should make the specific choices here feel
inevitable rather than clever.

## Complexity
- Time: O(n log n)
- Space: O(n)

## Solution
Runnable, with sample test cases at the bottom (`python3 greedy/04_hand_of_straights/solution.py`):

```python
--8<-- "greedy/04_hand_of_straights/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move):** "A brute force would try to
  build groups by backtracking over all ways to partition the cards — combinatorial and
  way too slow. The insight that gets me to a greedy O(n log n) solution is that I don't
  need to search: the smallest remaining card is *forced* to start a new group, because
  nothing smaller is left to need it first."
- **Invariant framing (good for proving the greedy choice can't be beaten):** "The
  invariant is: at every step, the smallest value with count > 0 must be the start of some
  group in any valid solution, because no card smaller than it remains to consume it
  otherwise. That's an exchange argument — I'd say it out loud before writing code, since
  it's what makes 'always take the smallest' provably correct, not just convenient."
- **Generalization framing (good for connecting to the pattern library):** "This is
  'sort or heapify, then greedily consume the smallest remaining run' — I'd name that as
  the family, and mention it's a cousin of interval-scheduling greedy problems in that both
  rely on an exchange argument to justify picking the extreme value first."

### Vocabulary Builder

- **frequency map** (n. phrase) — a hash map from value to count; the natural data
  structure here since groups consume specific counts of consecutive values, not just
  presence/absence.
- **exchange argument** (n. phrase) — a proof technique showing any optimal solution can be
  rearranged to match your greedy choice without getting worse; the justification for
  "always start with the smallest remaining card."
- **"…is forced, not chosen"** — a precise phrase for describing a greedy step that isn't
  really a choice among alternatives at all, just the only option consistent with
  optimality — useful for distinguishing true greedy insight from arbitrary tie-breaking.
- **degenerate case** (n. phrase) — here, `len(hand) % groupSize != 0`, which fails
  immediately regardless of card values; naming it up front shows you check feasibility
  before reasoning about the harder cases.
