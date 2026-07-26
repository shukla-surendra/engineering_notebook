# Pattern: Tree Traversal (DFS & BFS)

## What problem does this solve?

Almost every tree problem is a traversal with a twist: visit nodes in some order (pre/in/
post-order DFS, or level-by-level BFS), and either transform the tree, compute something
bottom-up from children, or compute something top-down from ancestors. Recognizing *which*
of these shapes a problem needs is most of the battle — the traversal skeleton itself is
almost always the same few lines.

## How to recognize which traversal to use

- **"Level by level" / "shortest path in an unweighted tree" / "average per level"** → BFS
  with a queue.
- **A property depends only on a node's own value and its children's already-computed
  results** ("depth," "is this a valid BST," "max path sum") → post-order DFS: recurse into
  children *first*, then combine their results at the current node.
- **A property flows from the root down (ancestors constrain descendants)** — Validate
  BST's `(low, high)` range, or "path from root to a target" → pre-order DFS: compute
  something at the current node *first*, then pass it down to children as an argument.
- **"Same shape as another tree" / "is X a subtree of Y"** → DFS comparison, often as a
  helper function called from within a second traversal.

## The general template

**Post-order DFS (bottom-up — most common):**
```python
def solve(node):
    if not node:
        return base_case_value           # e.g. 0 for depth, True for "valid" checks

    left_result = solve(node.left)
    right_result = solve(node.right)

    return combine(node.val, left_result, right_result)
```
This is the shape of Max Depth (`1 + max(left, right)`), Same Tree, Subtree of Another
Tree, and — with a bit more bookkeeping — Binary Tree Maximum Path Sum (where you also
update a *global* running maximum before returning the "one-sided" value upward, because a
node can be a path's peak using both children even though it can only report one side to
its own parent).

**Pre-order DFS (top-down — range/constraint propagation):**
```python
def solve(node, state_from_ancestors):
    if not node:
        return True  # or whatever the "vacuously fine" base case is

    if not valid_given(node.val, state_from_ancestors):
        return False

    new_state_left = update_state(state_from_ancestors, node.val, going_left=True)
    new_state_right = update_state(state_from_ancestors, node.val, going_left=False)
    return solve(node.left, new_state_left) and solve(node.right, new_state_right)
```
This is Validate BST: the "state from ancestors" is the `(low, high)` valid range, tightened
as you descend.

**BFS (level order):**
```python
queue = deque([root])
while queue:
    level_size = len(queue)          # snapshot BEFORE draining — this is the whole trick
    level = []
    for _ in range(level_size):
        node = queue.popleft()
        level.append(node.val)
        if node.left: queue.append(node.left)
        if node.right: queue.append(node.right)
    result.append(level)
```
Snapshotting `len(queue)` before the inner loop is what separates levels cleanly, without
needing `None` sentinels between levels.

**BST-specific shortcut:** because a BST's in-order traversal visits nodes in sorted order,
"Kth smallest" doesn't need to materialize the whole traversal — an iterative in-order walk
with an explicit stack can stop the instant the k-th node is popped. And search-style
problems (LCA of a BST) don't need to explore both children at all — the BST ordering tells
you which single child to descend into.

## Common pitfalls

- Doing a post-order computation but trying to also enforce a top-down constraint in the
  same pass without threading the ancestor state through — these are two different
  traversal shapes and mixing them up produces subtly wrong answers (the classic bug:
  checking a BST by only comparing a node to its immediate children instead of the full
  valid range).
- Forgetting the null/base case, causing a crash instead of a clean vacuous result.
- For "maximum path sum" style problems, forgetting to clamp negative child contributions
  to 0 before including them (a negative branch should just be excluded, not subtracted).

## Complexity characteristics

O(n) time (every node visited once) for standard traversals. O(h) space for DFS recursion
(h = tree height: O(log n) balanced, O(n) worst-case skewed), O(w) space for BFS (w = max
width of the tree, up to O(n)).

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Recognition framing (the default for opening a new tree problem):** "Before writing
  anything, I ask myself one question: does this value flow *up* from children, or *down*
  from ancestors? If it's 'compute something from my children's results,' that's
  post-order DFS. If it's 'a constraint my ancestors impose on me,' that's pre-order DFS
  with state threaded down. Naming which one out loud before coding prevents me from
  writing a traversal that fights the problem's actual data flow."
- **Mental-model framing (good for explaining the template as reusable, not memorized):**
  "I don't memorize solutions per problem — I memorize three shapes: post-order combine,
  pre-order propagate, and BFS-by-level. Every problem in this file is one of those three
  shapes plus a different `combine`, `valid_given`, or `update_state` function. Saying that
  explicitly is how I signal I'm generalizing, not pattern-matching from memory."
- **Generalization/insight framing (good for the 'why does this work at all' question):**
  "The deep reason tree recursion works cleanly is that a tree has no cycles — every
  subtree is a strictly smaller, independent instance of the same problem, so 'solve the
  children first, then combine' is always well-founded. That's also exactly why the same
  post-order idea generalizes to DAGs (topological order) once cycles are ruled out."

### Vocabulary Builder

- **post-order / pre-order** (adj.) — describing *when* a node is processed relative to
  its children (after vs. before); the single most useful classification question for any
  new tree problem.
- **well-founded** (adj.) — describing a recursion guaranteed to terminate because each
  call operates on a strictly smaller subproblem; trees are well-founded by construction
  since they're acyclic.
- **"…fights the problem's actual data flow"** — a reusable phrase for describing a
  traversal choice (top-down vs. bottom-up) that's technically workable but awkward,
  usually producing convoluted state-passing instead of a clean recurrence.
- **threading (state)** (v.) — passing accumulated context down through recursive calls
  as an explicit argument (e.g. the `(low, high)` range in Validate BST), as opposed to
  relying on return values or global variables.
- **acyclic** (adj.) — containing no cycles; the property that makes trees a special,
  easier case of general graph traversal, where "visited" tracking is unnecessary because
  no path can loop back on itself.
