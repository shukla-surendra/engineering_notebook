# Pattern: Greedy

## What problem does this solve?

A greedy algorithm makes the locally-best choice at each step and never reconsiders it,
trusting that a sequence of locally optimal choices leads to a globally optimal answer.
This is much cheaper than DP (which considers and compares multiple choices) or
backtracking (which explores and can undo choices) — but it's only *correct* when the
problem has a specific structural property, so the real skill in this topic is proving
(at least informally, out loud in an interview) *why* the greedy choice can't be beaten,
not just proposing one.

## How to recognize it — and how to justify it

Greedy is worth trying when you notice a locally obvious "best" choice at each step. But
before committing to it, be able to articulate one of these arguments:

- **Exchange argument**: "If an optimal solution did something different from my greedy
  choice, I could swap it for my greedy choice without making the answer worse." (This is
  the classical proof technique for interval scheduling, Huffman coding, etc.)
- **Matroid / no-benefit-to-waiting argument**: "Delaying this choice can never help,
  because the situation only gets worse (or stays the same) the longer you wait." (Jump
  Game: if index `i` is unreachable, waiting longer doesn't make it reachable — you must
  have already locked in the best possible reach by the time you get there.)
- **Monotonicity of the objective**: for Kadane's algorithm (Maximum Subarray), a running
  sum that's gone negative can *never* help any future subarray — so resetting immediately
  is strictly at least as good as carrying it forward.

If you can't construct *any* version of these arguments, that's a signal the problem might
not actually be greedy — it might need DP instead (DP is the "I'm not sure greedy works, so
let me consider all choices and let the recurrence figure out the best one" fallback).

## The general template

There's no single code template the way there is for two pointers or sliding window —
greedy is more about the *decision rule* than a fixed control-flow shape. But the recurring
shapes are:

**Sort, then make one pass with a running decision:**
```python
items.sort(key=...)   # by start time, end time, ratio, etc. — the sort key IS the greedy rule
result = []
running_state = initial
for item in items:
    if compatible(item, running_state):
        take(item)
        running_state = update(running_state, item)
```
(Non-overlapping Intervals, Gas Station's "reset the candidate start" logic.)

**Single running best/threshold, updated as you scan:**
```python
running_value = initial
best = initial
for x in arr:
    running_value = locally_best_choice(running_value, x)
    best = max(best, running_value)  # or min, depending on the problem
```
(Maximum Subarray via Kadane's, Jump Game's "farthest reachable" tracker.)

## Variations you'll see

- **Kadane's algorithm** (Maximum Subarray): the greedy rule is "reset to 0 whenever the
  running sum goes negative" — justified because a negative running sum can only hurt any
  future extension.
- **Farthest-reach tracking** (Jump Game): the greedy rule is "always record the furthest
  index reachable so far" — you never need to consider *which* earlier jump got you there,
  only the frontier itself.
- **Reset-on-deficit** (Gas Station): whenever the running tank goes negative starting from
  a candidate station, none of the stations between the old candidate and here could have
  worked either — jump the candidate past the failure point entirely, an O(n) argument
  that avoids checking each of the n possible starting points independently (which would be
  O(n²)).
- **Sort by end time for interval scheduling** (Non-overlapping Intervals): sorting by
  *end* time (not start) greedily maximizes the number of non-overlapping intervals you
  can keep, because ending earlier always leaves more room for future intervals.
- **Smallest-remaining-first via a heap** (Hand of Straights): the smallest card with any
  count left *must* start a new group, since nothing smaller remains to need it first.

## Common pitfalls

- Assuming greedy works without being able to state why — this is the single most common
  way greedy solutions turn out to be wrong on a counterexample the interviewer supplies.
- Picking the wrong sort key (by start instead of end time, or vice versa) — the direction
  of the sort often *is* the greedy insight, and getting it backwards silently produces a
  suboptimal (not just differently-ordered) answer.
- Applying greedy to a problem that actually requires considering multiple choices
  simultaneously (that's DP) — if a locally-best choice can be invalidated by information
  that only becomes available later, greedy is the wrong tool.

## Complexity characteristics

Usually O(n log n) (dominated by an initial sort) or O(n) (single pass, no sort needed) —
greedy's whole appeal is trading DP's O(n) or O(n²) *table* for a single decisive pass.

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Recognition framing (how I'd open a discussion of this pattern):** "Greedy is worth
  reaching for whenever I notice an obviously-best local choice at each step — sort by
  some key, then take a single pass making one decision per element. The tell is that I'm
  tempted to say 'just always do X' without needing to compare X against alternatives."
- **Justification framing (the part that actually distinguishes strong candidates):** "The
  real skill isn't spotting a plausible greedy rule, it's proving it can't be beaten —
  usually with an exchange argument ('if the optimal solution differed from my choice, I
  could swap them without making it worse') or a no-benefit-to-waiting argument ('delaying
  this decision can only make the situation the same or worse, never better'). I'd say
  which one applies before writing any code."
- **Contrast framing (good for showing you know the boundary of the technique):** "If I
  can't construct any version of that proof — if a locally-best choice could be
  invalidated by information that only shows up later — that's my signal to drop greedy and
  reach for DP instead, which considers multiple choices and lets the recurrence decide
  rather than committing early."

### Vocabulary Builder

- **exchange argument** (n. phrase) — the classic greedy-correctness proof: show any
  optimal solution can be transformed into your greedy solution one swap at a time without
  losing value. *"The exchange argument for sorting by end time is what tells me it's not
  just a heuristic."*
- **local optimum vs. global optimum** (n. phrase) — a locally-best choice is only useful
  if it's provably consistent with the globally-best answer; greedy's entire risk surface
  is confusing the two.
- **counterexample** (n.) — a specific input that breaks a proposed greedy rule; the
  fastest way an interviewer tests whether your "obviously correct" greedy actually is —
  worth actively hunting for one yourself before committing to the approach.
- **"…considers all choices and lets the recurrence decide"** — a compact phrase for
  explaining what DP does differently from greedy, useful when pivoting away from a greedy
  idea that doesn't have a clean proof.
- **suboptimal** (adj.) — technically valid but not the best possible; the usual failure
  mode of a greedy algorithm with the wrong sort key or comparison direction, as opposed to
  outright incorrect.
