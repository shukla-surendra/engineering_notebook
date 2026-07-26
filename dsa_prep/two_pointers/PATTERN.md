# Pattern: Two Pointers

## What problem does this solve?

Many array/string problems ask you to find a pair, a palindrome check, or a partition that
satisfies some ordering condition. Brute force checks every pair — O(n²). Two pointers
exploits *sortedness or a monotonic relationship* to eliminate whole ranges of candidates
at once, cutting this down to O(n) with O(1) extra space.

## How to recognize it

Signals that two pointers applies:
- The array is sorted, or can cheaply be sorted, and the problem is about pairs/triplets
  with a target sum/relationship.
- You need to check a string/array from both ends inward (palindrome checks).
- You're computing something like "area between two lines," "capacity between two walls,"
  or any objective that depends on a *pair* of positions and shrinks as they converge.
- The brute force is O(n²) with a nested loop over pairs, and there's some monotonic
  structure you could exploit to avoid checking every pair.

## The general template

**Converging pointers** (opposite ends moving inward — palindrome check, container with
most water, 3Sum's inner loop):
```python
left, right = 0, len(arr) - 1
while left < right:
    if condition(arr[left], arr[right]):
        # process / record the answer
        left += 1
        right -= 1
    elif need_to_move_left(arr[left], arr[right]):
        left += 1
    else:
        right -= 1
```
The key correctness argument you should be able to state out loud: *"moving the pointer I
chose to move can never make the answer worse, and not moving it would definitely not help
future candidates."* For Container With Most Water, that's: moving the taller line inward
can only shrink width without ever increasing the limiting (shorter) height, so it's
strictly dominated — you must move the shorter line.

**Fixed + sliding second pointer** (3Sum: fix one index, two-pointer the rest):
```python
nums.sort()
for i in range(len(nums)):
    left, right = i + 1, len(nums) - 1
    while left < right:
        ... # standard converging two-pointer on the subarray after i
```

**Fast/slow pointers** (same direction, different speeds — cycle detection, middle of a
linked list): see `../linked_list/PATTERN.md`, which is really a two-pointer variant.

## Variations you'll see

- **Sort first, then two-pointer** — turns an O(n²) or O(n³) brute force (3Sum) into
  O(n log n) + O(n²), which is the best known approach for that family of problems.
- **Skip duplicates** — after sorting, when you want *unique* combinations (3Sum), you
  must explicitly skip over repeated values at each pointer once a candidate is recorded,
  or you'll emit the same triplet multiple times.
- **Read/write pointers in the same array** — a different flavor: one pointer scans, the
  other marks where the next "kept" element should be written (e.g. removing duplicates
  in place, partitioning). Both pointers move in the same direction here, not toward
  each other.

## Common pitfalls

- Forgetting to skip duplicate values after finding a match (produces duplicate output in
  3Sum-style problems).
- Off-by-one errors in the loop condition (`<` vs `<=` for `left`/`right` crossing).
- Reaching for two pointers on an *unsorted* array where the target relationship isn't
  monotonic — sort first, or recognize this isn't actually a two-pointer problem.

## Complexity characteristics

O(n) after any required O(n log n) sort, and O(1) extra space — this is usually the space
optimization over a hash-map approach (see `../arrays_hashing/PATTERN.md`) when the array
is already sorted or sorting is affordable.

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Recognition framing (the default for spotting when two pointers applies):** "The
  question I ask myself is: is there a monotonic relationship I can exploit — sortedness,
  or a value that only shrinks/grows as pointers move — and does the brute force look like
  a nested loop over pairs? If both are true, two pointers is very likely to collapse an
  O(n²) brute force to O(n), and I'd say that reasoning out loud before jumping to code."
- **Mental-model framing (good for explaining the technique as a family, not a trick):**
  "I think of two pointers as three related shapes, not one: converging pointers from
  opposite ends, a fixed pointer with a sliding second one (3Sum's inner loop), and
  same-direction fast/slow pointers for linked-list-style problems. Naming which shape
  applies is more useful than reciting 'use two pointers,' since the correctness argument
  differs across the three."
- **Generalization/insight framing (good for the 'why does moving one pointer suffice'
  question):** "The deep justification, in every converging-pointer problem I've seen, is
  a dominance argument: the move I choose to make is never worse, and the move I skip could
  never have helped. I'd say that's the actual thing to prove on the whiteboard — 'I moved
  two pointers inward' is not a proof, 'here's why the move I skipped was safe to skip' is."

### Vocabulary Builder

- **monotonic** (adj.) — consistently non-decreasing or non-increasing; the structural
  property (usually from sorting) that two pointers exploits to eliminate candidates
  without checking them individually.
- **dominance argument** (n. phrase) — a correctness proof showing one choice is never
  worse than another, making it safe to discard the alternative without exploring it; the
  standard justification pattern for a greedy pointer move.
- **"…collapses an O(n²) brute force to O(n)"** — a precise, reusable phrase for stating
  the complexity payoff of two pointers up front, before walking through the mechanism.
- **partition** (v.) — to divide a collection into distinct regions in place using pointers
  (e.g. a read pointer and a write pointer moving in the same direction), a distinct
  variant from the converging-pointer shape covered above.
- **degenerate case** (n. phrase) — an input where the two pointers start already crossed
  or adjacent (an empty array, a single-element array); worth naming as the boundary
  condition that a `while left < right` guard is specifically designed to handle safely.
