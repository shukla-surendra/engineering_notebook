# Pattern: Sliding Window

## What problem does this solve?

Problems about a *contiguous* subarray/substring satisfying some condition (longest,
shortest, count, or check) look like they need to re-examine every possible window — O(n²)
or worse. A sliding window keeps a "current candidate window" defined by `[left, right]`
and adjusts its edges incrementally instead of recomputing from scratch, because most work
done for one window is still valid for the next.

## How to recognize it

Signals that sliding window applies:
- The problem talks about a **contiguous** subarray/substring (not "any subsequence" —
  that's usually DP or backtracking instead).
- You're asked for the longest/shortest/count of windows satisfying a constraint
  ("no repeating characters," "at most k distinct," "sum ≥ target").
- The brute force is "try every `(left, right)` pair and check the window" — O(n²) or
  O(n³) if checking the window itself isn't O(1).
- Growing the window only ever makes some property "more true" or "more false"
  monotonically (this monotonicity is *why* the technique is correct — see below).

## The general template

**Variable-size window** (most common — longest/shortest valid window):
```python
left = 0
window_state = {}  # counts, sums, etc. describing the current window

for right in range(len(arr)):
    add(arr[right], window_state)              # expand: absorb the new right element

    while window_is_invalid(window_state):       # shrink while broken
        remove(arr[left], window_state)
        left += 1

    update_answer(right - left + 1)              # window [left, right] is valid here
```
Or the inverse shape — grow until valid, then shrink to the minimum valid window (Minimum
Window Substring): grow the right edge until the window satisfies the condition, then
greedily shrink the left edge as far as possible while it stays valid, recording the best
window at each valid shrink step.

**Fixed-size window** (Permutation in String — window length is always `len(s1)`):
```python
window_state = {}
for i in range(len(arr)):
    add(arr[i], window_state)
    if i >= k:
        remove(arr[i - k], window_state)
    if i >= k - 1:
        check_and_record(window_state)
```

## Why it's correct: the amortized-O(n) argument

The `left` pointer only ever moves forward, never backward, and it moves at most n times
total across the *entire* run of the algorithm — even though it's inside a `while` loop
that might look like it re-scans. That's what makes the total work O(n) instead of O(n²):
every element is added to the window exactly once and removed at most once.

## Variations you'll see

- **Counting-based validity** (Longest Repeating Character Replacement): validity isn't a
  simple boolean from one variable, but derived from a frequency count (e.g.
  `window_length - max_freq <= k`). The window never needs to *shrink* below its historical
  best size in this style — you can slide both pointers forward together instead of
  re-shrinking, since you only care about the maximum length ever achieved.
- **have/need counters** (Minimum Window Substring): track how many *distinct* required
  keys are currently satisfied, rather than comparing full count dictionaries every step —
  turns an O(26) or O(distinct chars) comparison into an O(1) increment/decrement check.

## Common pitfalls

- Trying to use sliding window on a problem about subsequences (non-contiguous) — that's
  the wrong tool; look at DP instead.
- Recomputing the window's validity from scratch every time instead of incrementally
  updating state as elements enter/leave — that silently reintroduces the O(n²) or worse
  cost the technique is supposed to eliminate.
- Off-by-one in window length (`right - left + 1`, not `right - left`).

## Complexity characteristics

O(n) or O(n · alphabet size) time, O(1) or O(alphabet size) space for fixed-alphabet
problems (like uppercase letters), O(n) space if tracking arbitrary keys in a hash map.

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Recognition framing (how you'd name the pattern before writing code):** "My trigger
  for sliding window is the word 'contiguous' plus a brute force that's obviously
  O(n²) because it re-examines overlapping ranges. I'd say that recognition out loud
  before touching the keyboard — it tells the interviewer I'm classifying the problem,
  not pattern-matching on vocabulary alone."
- **Mental-model framing (good for explaining the technique itself, not one instance of
  it):** "The mental model is: most of the work done evaluating one window is still valid
  for its neighbor, so instead of recomputing from scratch, I incrementally add what
  enters and remove what leaves. The `left` pointer's monotonic forward-only movement is
  what turns an apparently quadratic double loop into a linear amortized scan."
- **Generalization framing (good for showing you can extend the pattern to a new
  variant):** "Once I have the core loop memorized, every new problem is really just
  answering two questions: what state do I track in the window, and what's the validity
  condition? Fixed vs. variable size, and boolean vs. counting-based validity, are the
  axes I'd walk through to classify a problem I haven't seen before."

### Vocabulary Builder

- **amortized** (adj.) — a cost measured over the entire run of an algorithm, not any one
  step; central to why sliding window is O(n) despite an inner `while` loop that looks
  like it could repeat work.
- **monotonic** (adj.) — moving in a single, non-reversing direction; the `left` pointer's
  monotonicity is the mathematical reason the total work stays linear.
- **"the brute force re-examines overlapping work"** — a reusable diagnostic phrase for
  spotting when a sliding window applies: the naive solution redoes work a window could
  carry forward instead.
- **validity condition** (n. phrase) — the boolean or derived check that determines
  whether the current window counts as a candidate answer; naming it explicitly is how
  you generalize across "no repeats," "at most k distinct," "contains all of t," etc.
- **degenerate case** (n. phrase) — an edge case like an empty string or `k` larger than
  the array — worth naming as a boundary check even when the main logic handles it
  automatically.
