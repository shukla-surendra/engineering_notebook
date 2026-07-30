# 4. LRU Cache (as a Designed Class)

**Difficulty:** Medium
**Topic:** Low-Level Design
**Pattern:** Composition (hash map + doubly linked list) + Strategy (eviction policy)

## Requirements — and why this is LLD, not just DSA

"Implement an LRU cache with O(1) `get`/`put`" is a `dsa_prep`-style question when the ask
is a single class with hardcoded LRU behavior. It becomes an **LLD** question the moment
the ask changes to *"design a cache"* — because now the API surface, extensibility to
other eviction policies (LFU, TTL), and generic key/value typing are in scope, and those
are design decisions, not algorithmic ones. This doc treats it as the latter: the data
structure is a means, the class design around it is the point.

Requirements: O(1) `get`, O(1) `put`, fixed capacity, evict the least-recently-used entry
on overflow, and — the part that makes it LLD — **the eviction policy shouldn't be
hardwired into the cache class**, so a future LFU or TTL-based cache doesn't require
touching `Cache` at all.

## Core entities

- **`Node`** — a doubly-linked-list node holding a key/value pair. Storing the *key* on
  the node (not just the value) is the detail people miss — eviction needs to know which
  map entry to delete, and the node is the only thing the list touches directly.
- **`DoublyLinkedList`** — a sentinel-headed list (`head`/`tail` dummy nodes) giving O(1)
  `remove`, `add_front`, `pop_back` with no null-checking special cases at the boundaries.
  This class knows nothing about caching or eviction — it's a general-purpose ordered
  list, reusable outside this problem entirely.
- **`EvictionPolicy`** (interface) — `on_access`, `on_insert`, `evict`. `LRUEvictionPolicy`
  is the one implementation: on access, move the node to the front; on evict, pop the
  back. This is the Strategy seam — the entire reason this is a *design* question and not
  a *data structure* question.
- **`Cache`** — the coordinator. Owns the hash map (`key → Node`, giving O(1) lookup) and
  the `DoublyLinkedList` (giving O(1) reordering), and delegates every ordering decision to
  its `EvictionPolicy`. Notice `Cache` itself never decides *which* node to evict — it just
  asks the policy.

## Relationships

`Cache` **has** a `DoublyLinkedList` and an `EvictionPolicy` (composition, both
swappable). `Node` is referenced by both the hash map *and* the linked list simultaneously
— the same object, two access paths, which is precisely what gives O(1) on both operations:
the hash map finds the node instantly, the list already knows how to reorder/evict it once
found.

## Why the eviction logic isn't just in Cache.put()

If `evict` logic lived directly in `Cache.put`, swapping LRU for LFU means rewriting
`Cache`. With `EvictionPolicy` pulled out, `Cache`'s `get`/`put` never change — they always
just call `self.policy.on_access(...)` / `self.policy.evict(...)`. An `LFUEvictionPolicy`
would swap `DoublyLinkedList` recency-ordering for a frequency-count structure entirely
internal to the policy; `Cache` doesn't need to know or care.

## Extension follow-up

*"Now make this thread-safe for concurrent access, and add a TTL so entries expire even
without hitting capacity."* With this design: thread-safety is a lock around `get`/`put`
in `Cache` (doesn't touch the policy or list at all — it's an orthogonal concern). TTL is
a **second, independent** eviction trigger — either a background sweep that calls
`policy.evict` proactively, or a check-on-access in `get` that treats an expired node as
absent. Neither requires touching `DoublyLinkedList` or `LRUEvictionPolicy`, which is the
composition paying off again.

## Complexity
- Time: O(1) amortized for both `get` and `put`.
- Space: O(capacity).

## Solution
Runnable, with sample test cases at the bottom (`python3 lld/04_lru_cache/solution.py`):

```python
--8<-- "04_lru_cache/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **"Design vs. implement" framing (the default opening move for this specific
  problem):** "I'd clarify up front whether this is 'write an LRU cache' or 'design a
  cache' — the second phrasing means the eviction policy needs to be swappable, which
  changes the class boundaries before I write a line of code. Naming that distinction is
  itself a signal I know this round differs from a pure DSA one."
- **Mechanism framing (good for justifying the O(1) claim precisely):** "The O(1) bound
  comes from two structures pointing at the same node — a hash map for instant lookup, a
  doubly linked list for instant reorder/evict — and I'd be explicit that neither
  structure alone gets you both operations at O(1); it's the combination that does."
- **Extensibility framing (good for the eviction-policy design choice):** "I'd pull
  eviction behind a `Strategy` interface specifically so `Cache.get`/`put` never change
  when the policy changes — I'd point at the follow-up question I'm anticipating (LFU,
  TTL) as the justification, not just assert 'this is more extensible' abstractly."

### Vocabulary Builder

- **sentinel node** (n. phrase) — a dummy head/tail node in a linked list that's never a
  real entry, existing purely to eliminate null-checks at the list boundaries; makes
  `remove`/`add_front`/`pop_back` branch-free.
- **amortized O(1)** (adj. phrase) — constant time on average across a sequence of
  operations; the correct claim for hash-map-backed structures, worth stating precisely
  rather than just saying "O(1)."
- **eviction policy** (n. phrase) — the pluggable rule deciding which entry to remove on
  overflow; the concept that turns this from a data-structure question into a design one.
- **"…is the composition paying off again"** — a reusable phrase for closing an extension
  follow-up by naming *which* earlier design choice made the new requirement cheap.
