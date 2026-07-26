# Pattern: Stack

## What problem does this solve?

Stacks are the natural tool whenever "the most recently seen unresolved thing" is exactly
what you need next — nested/matching structures, expressions with operator precedence
implied by order, or "the next element that breaks some property" scanning problems. The
LIFO (last-in-first-out) order mirrors how these problems naturally resolve: the innermost
unmatched thing must be resolved before anything opened earlier.

## How to recognize it

Signals that a stack applies:
- Matching/nesting structure: brackets, parentheses, nested tags — anything where "the
  last opened must be the first closed."
- Expression evaluation (postfix/infix), where operators act on the most recently computed
  values.
- "For each element, find the next/previous element that is greater/smaller" — this is the
  **monotonic stack** sub-pattern (Daily Temperatures, Next Greater Element).
  Recognize it by: you're tempted to write a nested loop where the inner loop scans
  forward/backward looking for the first element satisfying some comparison — that inner
  scan is what the stack eliminates.
- You need O(1) access to "the minimum/maximum seen so far, but only among currently
  active elements" (Min Stack) — track auxiliary state alongside the main stack, not by
  rescanning it.

## The general template

**Matching/validation:**
```python
stack = []
for token in tokens:
    if token is an "opener":
        stack.append(token)
    else:  # it's a "closer" or an operator
        if not stack or stack[-1] doesn't_match token:
            return False  # or handle the mismatch
        stack.pop()
return not stack  # everything got matched
```

**Monotonic stack** (find next greater/smaller element for every position):
```python
stack = []  # holds indices, values kept in increasing (or decreasing) order
for i, val in enumerate(arr):
    while stack and arr[stack[-1]] < val:      # for "next greater": pop smaller values
        prev_index = stack.pop()
        answer[prev_index] = i - prev_index    # or val, or whatever "resolved" means here
    stack.append(i)
# whatever remains on the stack has no answer to the right — resolve to a default (0, -1, etc.)
```
The reason this is O(n) despite the `while` loop: every index is pushed once and popped at
most once across the *entire* run — same amortized argument as sliding window's `left`
pointer.

## Variations you'll see

- **Auxiliary stack for running aggregates** (Min Stack): keep a second stack in lockstep
  that always has the running min/max at that depth, so popping the main stack
  automatically "un-does" the aggregate too — no rescanning needed.
- **Stack as an evaluator** (Evaluate Reverse Polish Notation): operands get pushed;
  operators pop the last two operands, combine them, and push the result back. This
  directly mirrors how postfix notation is *defined* to be evaluated.
- **Call-stack simulation** (implicit in recursive DFS, but sometimes made explicit with a
  manual stack to avoid recursion-depth limits on very deep/wide inputs).

## Common pitfalls

- Forgetting to check `if not stack` before popping/peeking — an empty stack means an
  unmatched closer, which should usually fail validation rather than crash.
- For monotonic stacks, mixing up whether you want a strictly-increasing or
  strictly-decreasing stack, and `<` vs `<=` in the while condition — this changes whether
  equal elements resolve to each other.
- Forgetting that the final stack contents after the loop usually still need handling
  (e.g., they have "no answer" and should be resolved to a sentinel value).

## Complexity characteristics

O(n) time (each element pushed/popped a bounded number of times) and O(n) space
worst-case (e.g., a fully nested structure or a strictly monotonic input array).

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Recognition framing (how you'd name the pattern before writing code):** "My trigger
  for a stack is 'the most recently seen unresolved thing is exactly what I need next' —
  nesting, expression evaluation, or a nested loop where the inner loop is scanning
  forward/backward for the first element satisfying a comparison. I'd say that
  classification out loud before picking a data structure."
- **Mental-model framing (good for explaining the technique itself, not one instance of
  it):** "The mental model is LIFO mirrors resolution order: whatever opened most
  recently must close first, whatever's waiting longest for 'the next bigger thing' gets
  resolved as soon as it appears. A stack isn't just a container here — its ordering
  *is* the algorithm's logic."
- **Generalization framing (good for showing you can extend the pattern to a new
  variant):** "Once I recognize 'stack,' the next question is which of three shapes it
  is: matching/validation, monotonic (next greater/smaller), or evaluator (operands and
  operators). I'd walk through those three explicitly to classify an unfamiliar problem
  rather than guessing at an implementation."

### Vocabulary Builder

- **LIFO** (n., last-in-first-out) — the access discipline a stack enforces; the reason a
  stack (not a queue) matches nesting and 'most recent unresolved thing first' problems.
- **monotonic stack** (n. phrase) — a stack kept in strictly increasing or decreasing
  order, specialized for answering 'next greater/smaller element' queries in amortized
  O(n) instead of a naive O(n²) nested scan.
- **amortized** (adj.) — a cost measured over the algorithm's entire run; central to why
  a monotonic stack's inner `while` loop doesn't break the O(n) bound — every element is
  pushed once, popped at most once, total.
- **"the last unresolved thing is exactly what I need next"** — a reusable diagnostic
  phrase for recognizing when a stack applies, before deciding which of the three
  sub-patterns (matching, monotonic, evaluator) fits.
- **sentinel/guard check** (n. phrase) — a check like `if not stack` before popping or
  peeking, to fail gracefully on an unmatched closer instead of crashing.
