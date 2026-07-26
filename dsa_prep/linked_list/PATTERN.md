# Pattern: Linked List Pointer Manipulation

## What problem does this solve?

Linked lists can't be random-accessed or easily "peeked ahead" like arrays, so most linked
list problems are really about disciplined pointer bookkeeping: knowing exactly which
pointer to save before you overwrite `.next`, and using multiple pointers moving at
different speeds or from different starting points to extract information (length,
midpoint, cycle presence) in a single pass without extra memory.

## How to recognize it

Signals for each core technique:
- **Reversal needed** ("reverse", "reorder") → iterative pointer reversal with a `prev`
  tracker.
- **"Middle of the list" or "is there a cycle"** → fast/slow pointers (Floyd's algorithm).
- **"Nth from the end" without knowing the length up front, in one pass** → two pointers
  separated by a fixed gap of `n`.
- **Merging/removing nodes, especially possibly at the head** → a dummy sentinel node
  before `head`, so you never need special-case logic for "what if we remove/modify the
  head itself."
- **Combining multiple sorted lists** → a heap (see `../heap_priority_queue/PATTERN.md`) of
  the current head of each list.

## The general template

**Iterative reversal:**
```python
prev, curr = None, head
while curr:
    next_node = curr.next   # save before overwriting — this is the one line that matters
    curr.next = prev
    prev = curr
    curr = next_node
return prev  # prev is the new head once curr runs off the end
```

**Fast/slow pointers (find middle, or detect a cycle):**
```python
slow = fast = head
while fast and fast.next:
    slow = slow.next
    fast = fast.next.next
    if slow is fast:      # only for cycle detection
        return True
# for "find middle": slow is now at the middle when fast has run off the end
```
Why it works: `fast` moves twice as fast as `slow`, so if there's a cycle, the *gap*
between them shrinks by 1 every step once both are inside the cycle — they're guaranteed
to meet. If there's no cycle, `fast` simply reaches the end first.

**Gap-of-n pointers (Nth from end) — always pair with a dummy head:**
```python
dummy = ListNode(0, head)
fast = slow = dummy
for _ in range(n):
    fast = fast.next          # advance fast n steps first
while fast.next:
    fast, slow = fast.next, slow.next   # now advance both together
# slow.next is exactly the target node
```

**Dummy head, in general:** anytime the head of the list might need to change or be
removed, create `dummy = ListNode(0, head)`, do all your work relative to `dummy`, and
return `dummy.next` at the end. This one trick eliminates an entire category of
"what if the answer is an empty list" / "what if we remove the first node" edge cases.

## Variations you'll see

- **Combine primitives**: Reorder List is literally "find middle" + "reverse second half"
  + "merge two lists by alternating" chained together — recognizing the sub-problems is
  the whole difficulty.
- **Heap of k pointers**: Merge k Sorted Lists generalizes "merge two sorted lists" by
  keeping the current head of every list in a min-heap instead of just two variables.

## Common pitfalls

- Overwriting `.next` before saving the pointer you needed from it (classic reversal bug).
- Forgetting the dummy head, then writing awkward special-case code for "what if we're
  modifying the very first node."
- Off-by-one in the fast/slow gap setup for "Nth from end" (should `fast` advance `n` or
  `n+1` steps before the paired walk starts? — depends on whether you want the node itself
  or the node before it; work it out on a small example, don't guess).

## Complexity characteristics

Nearly everything in this topic is O(n) time, O(1) space — that O(1) space (versus an
O(n)-space array-conversion approach) is usually the entire point of doing it with pointer
manipulation instead of copying values into an array first.

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Recognition framing (how you'd name the pattern before writing code):** "Whenever I
  see a linked-list problem, my first question is 'can I random-access or peek ahead
  here?' — I can't, so the whole problem is really about disciplined pointer bookkeeping.
  I'd say that out loud, then map the problem onto one of a handful of known shapes:
  reversal, fast/slow, fixed-gap, or dummy-head-plus-merge."
- **Mental-model framing (good for explaining the technique itself, not one instance of
  it):** "The mental model I use is: a linked list gives you exactly one thing arrays
  don't — cheap in-place relinking — and takes away exactly one thing arrays have — random
  access. Every technique in this family is really about substituting a second (or third)
  pointer for the random access you don't have."
- **Generalization framing (good for showing you can extend the pattern to a new
  variant):** "If I saw a brand-new linked-list problem in an interview, I'd first ask
  which of the four templates it smells like — reversal, fast/slow, fixed-gap, or
  dummy-head-merge — and often it's a composition of two, the way Reorder List chains
  three of them together. Naming the sub-problems is usually 90% of solving it."

### Vocabulary Builder

- **pointer bookkeeping** (n. phrase) — the discipline of tracking which pointer to save
  before overwriting another; the unifying skill across this entire pattern family.
- **sentinel/dummy node** (n. phrase) — a placeholder node used to eliminate special-case
  logic for "what if the head changes"; worth having as a default reflex, not something
  you reach for only when you get stuck.
- **"this reduces to a known sub-problem"** — a reusable phrase for signaling
  decomposition when a new problem is really a chain of familiar primitives (as in
  Reorder List or Merge k Sorted Lists).
- **amortized** (adj.) — describing a cost measured over a whole sequence of operations
  rather than any single one; the fast/slow and gap-pointer templates are both O(n)
  overall even though a step "looks" like it could repeat work.
- **monotonic** (adj.) — moving in one direction only; the `left`/`slow` pointers in
  these templates only ever advance, never retreat, which is exactly what keeps the total
  work linear.
