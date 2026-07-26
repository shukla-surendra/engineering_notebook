# Pattern: 1-D Dynamic Programming

## What problem does this solve?

DP applies when a problem has **overlapping subproblems** (the same smaller question gets
asked repeatedly if you solve it recursively) and **optimal substructure** (the best answer
to the whole problem can be built from the best answers to smaller pieces of it). Without
DP, a naive recursion re-solves the same subproblem exponentially many times; DP solves
each distinct subproblem exactly once and reuses the result.

## How to recognize it

Signals that 1-D DP applies:
- The problem asks to "count the number of ways," "find the min/max," or "determine if
  possible" — and a greedy or two-pointer approach doesn't obviously work because the
  optimal choice at each step *depends on* what was chosen earlier, not just local info.
- A recursive brute force would naturally be written as `solve(i)` depending only on
  `solve(i-1)`, `solve(i-2)`, ... — a *single* index parameterizes the state (as opposed to
  2-D DP, where two indices are needed, e.g. two strings or a grid position).
- You catch a recursive brute force re-computing `solve(5)` many times across different
  call paths — that's the signature of overlapping subproblems, and memoizing (or building
  bottom-up) is the fix.

## The general process (do this every time, in order)

1. **Define the state precisely in words first**: "`dp[i]` = the answer to the problem
   considering only the first `i` elements" (or "ending exactly at index `i`" — these are
   subtly different and picking the wrong one is the most common source of bugs).
2. **Write the recurrence**: how does `dp[i]` relate to `dp[i-1]`, `dp[i-2]`, etc.? This
   usually falls out of asking "what are my choices at step `i`, and what does each choice
   cost/require from the state before it?"
3. **Identify the base case(s)**: the smallest `i` where the answer is obvious without
   recursing further.
4. **Decide direction**: build up from the base case(s) to the final answer (bottom-up,
   usually preferred — avoids recursion-depth issues), or recurse with memoization
   (top-down — sometimes more natural to write first, then convert).
5. **Optimize space** *last*, only after the recurrence is correct: if `dp[i]` only ever
   depends on the last one or two entries, you don't need a full array — a couple of
   rolling variables suffice (see Climbing Stairs, House Robber).

## The general template

**Bottom-up with a full array (write this version first):**
```python
dp = [None] * (n + 1)
dp[0] = base_case_0
dp[1] = base_case_1              # if the recurrence needs two prior states
for i in range(2, n + 1):
    dp[i] = combine(dp[i - 1], dp[i - 2], arr[i])   # the recurrence
return dp[n]
```

**Space-optimized (once the array version works, collapse it):**
```python
prev2, prev1 = base_case_0, base_case_1
for i in range(2, n + 1):
    prev2, prev1 = prev1, combine(prev1, prev2, arr[i])
return prev1
```

## Variations you'll see

- **Include/exclude at each step** (House Robber): `dp[i] = max(skip nums[i], take
  nums[i] + dp[i-2])` — the classic "take it or leave it" recurrence.
- **Reduce a circular constraint to two linear subproblems** (House Robber II): run the
  same linear DP twice, once excluding the first element and once excluding the last, and
  take the better result.
- **Expand-around-center instead of a DP table** (Longest Palindromic Substring,
  Palindromic Substrings): sometimes an O(n²) technique that *isn't* a DP array is simpler
  and just as efficient as the "official" DP table formulation — recognize when a simpler
  tool solves the same complexity class.
- **Unbounded reuse of items** (Coin Change): iterate amounts in the outer loop and items
  in the inner loop, referencing `dp[amount - item]` — because the same item can be reused,
  you don't need to track "which items used" as extra state.
- **Reachability DP over string prefixes** (Word Break): `dp[i]` = "can the first `i`
  characters be fully segmented" — a boolean recurrence rather than a min/max one.
- **Patience sorting for O(n log n)** (Longest Increasing Subsequence): the O(n²) DP
  recurrence (`dp[i] = 1 + max(dp[j] for j < i if arr[j] < arr[i])`) has a faster
  reformulation using binary search over "smallest tail per subsequence length" — worth
  knowing both, since the O(n²) version is what you'd derive first under pressure.

## Common pitfalls

- Defining the state ambiguously ("ending at i" vs. "considering the first i") and mixing
  the two definitions mid-recurrence — pick one and stay consistent.
- Forgetting a base case for `dp[1]` when the recurrence needs two prior states, causing an
  index-out-of-range or silently wrong answer for small inputs.
- Jumping straight to the space-optimized rolling-variable version before verifying the
  full-array recurrence is even correct — debug the array version first.

## Complexity characteristics

Usually O(n) time and O(n) space (or O(1) with rolling-variable optimization) for 1-D DP —
this is the entire value proposition versus the exponential-time naive recursion it
replaces.

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Recognition framing (how you'd explain *when* to reach for 1-D DP, before any specific
  problem):** "I listen for 'count the ways,' 'find the min/max,' or 'is it possible' where
  the optimal choice at a step depends on earlier choices, not just local information —
  that dependency is what rules out a pure greedy or two-pointer approach and rules in DP."
- **Mental-model framing (good for explaining the derivation process itself, not a specific
  recurrence):** "My process is always the same order: define the state in words first,
  derive the recurrence from 'what are my choices here,' identify base cases, decide
  bottom-up vs. top-down, and only then optimize space. I'd narrate that order explicitly —
  it's the difference between deriving a DP live and guessing at one."
- **Generalization framing (good for signaling you see the variations as one family):** "I'd
  point out that 'include/exclude,' 'unbounded reuse,' and 'reachability over prefixes' are
  all the same underlying idea — combine an optimal answer at `i` from optimal answers at
  smaller indices — just with different combination rules. Naming the shared skeleton is
  what lets me adapt fast to a DP variant I haven't seen before."

### Vocabulary Builder

- **optimal substructure** (n. phrase) — the property that an optimal solution to the whole
  problem is built from optimal solutions to its subproblems; one of the two formal
  preconditions (with overlapping subproblems) for DP to apply at all.
- **state** (n.) — the minimal information needed to describe a subproblem (here, a single
  index `i`); defining it precisely in words is the first and most important step, before
  any recurrence gets written.
- **"…the crux of it is defining the state precisely before touching the recurrence"** — a
  reusable phrase for opening any DP explanation, emphasizing process over memorized
  formulas.
- **top-down / bottom-up** (adj. phrases) — recursion-with-memoization versus iterative
  table-filling; two implementations of the same recurrence, worth naming which one you're
  choosing and why (bottom-up avoids recursion-depth limits).
