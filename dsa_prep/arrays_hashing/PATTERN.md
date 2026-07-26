# Pattern: Hashing for O(1) Lookups

## What problem does this solve?

A huge fraction of array/string problems reduce to one question asked repeatedly: *"have
I seen this value (or its complement, or a computed key derived from it) before?"* The
brute-force way to answer that is to re-scan everything you've seen so far — O(n) per
check, O(n²) overall. A hash map/set answers the same question in O(1) average time by
trading memory for speed: it remembers what you've seen in a structure that supports
near-instant membership checks and key → value lookups.

## How to recognize it

Reach for hashing when you notice any of these in a problem statement:
- "Does there exist a pair/triplet that sums to X" (complement lookup)
- "Return indices/count of duplicates" (seen-before check)
- "Group things by some derived property" (anagrams by sorted letters, etc.)
- "Find the most/least frequent element(s)" (frequency counting)
- You're tempted to write a nested loop whose inner loop is just "search for something in
  the array" — that inner linear search is almost always replaceable by a hash lookup.

## The general template

Two dominant shapes cover almost every hashing problem:

**1. Single-pass complement/seen-before check** (e.g. Two Sum, Contains Duplicate):
```python
seen = {}  # or set()
for i, x in enumerate(nums):
    complement = target - x          # or just x itself for a "seen before" check
    if complement in seen:
        return (found!)
    seen[x] = i                       # record x AFTER checking, not before
```
The subtlety: check for the complement *before* inserting the current element, so you
never match an element with itself.

**2. Bucket by a derived key** (e.g. Group Anagrams, Top K Frequent):
```python
buckets = defaultdict(list)  # or Counter for frequencies
for item in items:
    key = derive_canonical_key(item)   # e.g. sorted(item), or a count signature
    buckets[key].append(item)
```
The trick is always in choosing `derive_canonical_key` — it must map every item that
"belongs together" to the exact same key, and nothing else to that key.

## Variations you'll see

- **Frequency counting** (`collections.Counter`) — when the question is about "how many
  times" rather than "does it exist."
- **Bucket sort by frequency** — when you need the top/bottom-k by frequency in O(n)
  instead of O(n log n): index buckets by count (0..n), since frequency is bounded by n.
- **Prefix sums in a hash map** — for "subarray sums to K" style problems: store running
  prefix sums seen so far, and check if `current_prefix - K` has been seen.
- **Set-based "only expand from a true start"** — e.g. Longest Consecutive Sequence: avoid
  quadratic re-work by only starting an expansion from an element whose predecessor isn't
  in the set, so the total work across all expansions is still O(n).

## Common pitfalls

- Forgetting that a hash map lookup only helps if you check *before* you mutate/insert the
  current element (self-matching bugs).
- Using a mutable/unhashable key (like a `list`) — convert to `tuple` first.
- Assuming O(1) is worst-case: Python dicts are O(1) *average* case; adversarial hash
  collisions are a non-issue in practice for interview purposes, but it's worth knowing
  the caveat exists.

## Complexity characteristics

Almost always O(n) or O(n·k) time (k = size of a derived key, like word length) and O(n)
or O(n·k) space — you're paying memory to avoid repeated scanning. If interview follow-ups
ask for O(1) space, that's a signal to look for a two-pointer or in-place technique instead
(see `../two_pointers/PATTERN.md`).

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Recognition framing (how you'd name the pattern the moment you see it):** "As soon as
  I catch myself about to write a nested loop where the inner loop just re-searches
  something I've already scanned, I stop and say out loud: this is a hashing problem —
  I'm going to trade memory for speed and remember what I've seen instead of re-deriving it
  every time."
- **Mental-model framing (how you'd teach it to someone else in thirty seconds):** "Almost
  every hashing problem boils down to one repeated question: 'have I seen this value, or
  some key derived from it, before?' Once you frame it that way, the only real decision left
  is what the key is — the raw value, its complement, or some canonical transformation of
  it."
- **Generalization framing (what separates a pattern-matcher from someone who's memorized
  seven problems):** "I'd point out that Two Sum, Group Anagrams, and Top K Frequent are
  the same skeleton with three different key functions — complement lookup, canonical
  signature, and frequency count. Naming that skeleton out loud is what tells the
  interviewer I'm generalizing, not pattern-matching from memory."

### Vocabulary Builder

- **canonical key** (n. phrase) — a derived value chosen so that all "equivalent" inputs
  map to the exact same key and nothing else does; the central design decision in the
  bucket-by-key template. *"Choosing the right canonical key is the whole problem —
  everything after that is boilerplate."*
- **amortized** (adj.) — a cost measured as an average over a full sequence of operations
  rather than guaranteed on any single one; relevant whenever a hash-based loop looks
  worse than O(n) at first glance but isn't.
- **"trades memory for speed"** — the single most reusable phrase for justifying any
  hash-based optimization over a brute-force scan, worth having ready verbatim.
- **prefix sum** (n. phrase) — a running cumulative total; combined with a hash map of
  sums-seen-so-far, it extends this pattern to subarray-sum problems beyond simple
  membership checks.
- **self-matching bug** (n. phrase) — the classic error of checking membership *after*
  inserting the current element instead of before, causing an element to incorrectly pair
  with itself.
