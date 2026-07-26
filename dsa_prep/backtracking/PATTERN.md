# Pattern: Backtracking

## What problem does this solve?

Whenever a problem asks for *all* valid configurations (subsets, permutations,
combinations, valid board placements) rather than just one optimal value, brute force means
generating every possibility and filtering — often wildly wasteful because most partial
attempts fail early and don't need to be completed to know that. Backtracking builds a
solution incrementally, and the instant a partial choice can't possibly lead anywhere
valid, it abandons that branch immediately ("prunes") instead of finishing it — this is
what separates it from blind brute-force enumeration.

## How to recognize it

Signals that backtracking applies:
- The problem says "return all..." (all subsets, all permutations, all valid boards, all
  paths) rather than "return the count" (which might be pure DP) or "return one" (which
  might be greedy or DP with reconstruction).
- There's a sequence of decisions to make (include/exclude an element, which number to
  place next, which direction to move), and each decision can be *undone* to try the next
  option.
- A constraint can be checked incrementally, letting you cut off a whole subtree of
  possibilities early (an N-Queens placement that conflicts, a combination that already
  exceeds the target sum).

## The general template

**The choose → explore → un-choose skeleton — this is the whole pattern:**
```python
path = []

def backtrack(state):
    if is_complete(state):
        record(path)          # or return True for "does any solution exist"
        return                # (or `return True` and propagate up, for existence checks)

    for choice in options(state):
        if not is_valid(choice, state):
            continue          # prune — skip invalid choices without recursing

        path.append(choice)               # choose
        backtrack(next_state(state, choice))   # explore
        path.pop()                         # un-choose — this line is what makes it "backtracking"

backtrack(initial_state)
```
The `path.pop()` (or un-marking a visited cell, or removing a column/diagonal from a
conflict set) after the recursive call returns is the single most important line: without
it, choices from one branch would leak into sibling branches.

## Variations you'll see

- **Include/exclude at every index** (Subsets): two recursive calls per index — one with
  the element in the path, one without — rather than a `for` loop over choices.
- **Reuse allowed vs. not** (Combination Sum vs. Permutations/N-Queens): whether the
  recursive call advances to `i + 1` or stays at `i` controls whether an element can be
  reused. A `used[]` array or a "remove from remaining options" set controls the
  no-reuse case.
- **Grid/graph backtracking** (Word Search): "choose" means moving to a neighbor cell,
  "un-choose" means restoring that cell's visited-marker after exploring all 4 directions
  from it — in-place marking (overwrite then restore) avoids needing a separate visited
  set.
- **Constraint sets instead of a visited array** (N-Queens): track "used columns" and
  "used diagonals" as sets for O(1) conflict checks, rather than re-scanning the board.

## Common pitfalls

- Forgetting to undo a choice (`path.pop()`, un-marking a cell, removing from a used set)
  — causes wildly wrong results because state leaks between branches.
- Doing the validity/pruning check *after* recursing instead of before — this still gets
  the right answer eventually but explores far more of the tree than necessary, sometimes
  turning a fast solution into a timeout.
- Copying the path incorrectly (`result.append(path)` instead of `result.append(path[:])`)
  — since `path` is mutated in place, appending a reference means every recorded "answer"
  ends up pointing to the same (eventually empty) list.

## Complexity characteristics

Exponential in the worst case (that's inherent — you're enumerating an exponentially large
solution space), but pruning is what makes previously-intractable inputs feasible in
practice. Space is O(depth of the recursion) for the path/call stack, plus whatever the
output itself costs to store.

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Recognition framing (the trigger phrase to listen for):** "Whenever a problem asks me
  to return *all* valid configurations, not the count and not just one, that's my signal
  to reach for backtracking rather than pure DP or greedy. I'd say that distinction out
  loud, since it's the fastest way to justify the approach before writing any code."
- **Mental-model framing (how you'd teach the skeleton to someone in one breath):** "It's
  choose, explore, un-choose — at every step, make a choice, recurse into the consequences
  of that choice, and then undo it before trying the next option. The un-choose step is the
  one people forget, and it's the one that actually makes it 'backtracking' instead of
  plain brute-force recursion."
- **Generalization framing (what separates fluency from having memorized five problems):**
  "I'd point out that Subsets, Combination Sum, and Permutations are all the same skeleton
  with one dial turned differently — whether the recursion advances the index or stays put
  controls whether reuse is allowed. Naming that dial, rather than re-deriving each
  solution from scratch, is what shows real pattern recognition."

### Vocabulary Builder

- **prune** (v.) — to abandon a branch of the search the moment it provably can't lead to a
  valid solution, instead of exploring it to completion. *"I prune as soon as a partial
  placement conflicts, rather than discovering the conflict at a leaf."*
- **state space** (n. phrase) — the complete set of configurations a search could visit in
  principle; backtracking's efficiency comes from how much of it pruning lets you skip.
- **"the un-choose step is what makes it backtracking"** — a precise, quotable way to
  distinguish backtracking from generic brute-force recursion when asked to define the term.
- **exponential blowup** (n. phrase) — the inherent worst-case growth of enumerating an
  entire solution space; naming it upfront shows you're not surprised by the complexity
  bound, just managing it with pruning.
- **incremental construction** (n. phrase) — building a solution one decision at a time
  rather than generating full candidates and validating them after the fact; the structural
  reason backtracking can prune mid-build.
