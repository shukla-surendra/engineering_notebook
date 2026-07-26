# Pattern: Interval Scheduling

## What problem does this solve?

Interval problems (meetings, ranges, bookings) almost always become easy once sorted in
the right order — the difficulty is realizing *which* sort key makes the rest of the
problem a simple linear scan, since sorting by start vs. end time solve different
questions.

## How to recognize it

Signals that this topic's techniques apply:
- Input is a list of `[start, end]` pairs.
- The question is about overlap ("can all meetings be attended," "merge overlapping
  ranges"), counting simultaneous usage ("minimum meeting rooms"), or fitting the maximum
  number of non-conflicting items ("minimum removals to make non-overlapping").

## The two sort keys, and when to use each

**Sort by start time** — use this when you need to process intervals in the order they
begin, to merge adjacent/overlapping ones or detect any overlap at all:
```python
intervals.sort(key=lambda pair: pair[0])
merged = [intervals[0]]
for start, end in intervals[1:]:
    if start <= merged[-1][1]:              # overlaps the last kept interval
        merged[-1][1] = max(merged[-1][1], end)   # extend it
    else:
        merged.append([start, end])          # starts a new, separate interval
```
(Merge Intervals, Meeting Rooms — where any overlap at all is a failure, so you only need
to compare each interval to the *previous* one once sorted.)

**Sort by end time** — use this for greedy *selection* problems: "keep the maximum number
of non-overlapping intervals" (equivalently, "remove the minimum number to make the rest
non-overlapping"):
```python
intervals.sort(key=lambda pair: pair[1])
kept_end = float('-inf')
count_kept = 0
for start, end in intervals:
    if start >= kept_end:
        kept_end = end
        count_kept += 1
```
Why end time and not start time here: greedily keeping the interval that **finishes
earliest** among the compatible candidates always leaves the most room for future
intervals — this is the exchange-argument justification (see
`../greedy/PATTERN.md`) for why this specific sort key is optimal and sorting by start
time would not be.

**Sweep with a heap for "concurrent usage" problems** (Meeting Rooms II — how many
rooms/resources are needed simultaneously):
```python
intervals.sort(key=lambda pair: pair[0])
heap = []  # end times of meetings currently "in progress"
for start, end in intervals:
    if heap and heap[0] <= start:
        heapq.heappop(heap)     # earliest-ending meeting has freed a room — reuse it
    heapq.heappush(heap, end)
return len(heap)   # max concurrent size across the whole sweep is the answer
```
The heap always holds the end times of currently-active intervals; its size at any moment
is "rooms in use right now," and the *maximum* size reached during the whole sweep is the
answer.

**Three-way split for inserting into an already-sorted, non-overlapping list** (Insert
Interval): walk once through intervals strictly before the new one (copy as-is), then
intervals overlapping it (merge into an expanding `[start, end]`), then intervals strictly
after (copy as-is) — no need to re-sort since the input is already sorted and the new
interval is inserted in a single linear pass.

## Common pitfalls

- Sorting by the wrong key for the question being asked (start vs. end) — this is the
  single most common mistake in this topic; always ask "am I detecting overlap/merging"
  (sort by start) or "am I greedily selecting a maximum compatible subset" (sort by end)
  before writing any code.
- Off-by-one in the overlap check: `start <= end` (touching endpoints count as overlapping)
  vs. `start < end` (touching endpoints are fine, e.g. one meeting ending at 10 and another
  starting at 10) — read the problem statement carefully, since both conventions appear
  across different problems.
- For Meeting Rooms II, forgetting that popping from the heap only happens when the
  earliest-ending meeting has *already* ended by the time the new one starts — get the
  comparison direction right (`heap[0] <= start`, not `<`, depending on whether touching
  counts as freeing the room).

## Complexity characteristics

O(n log n), dominated by the initial sort — every variant here does O(n) work after that
single sort.

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Recognition framing (how I'd open a discussion of this pattern):** "Whenever I see a
  list of `[start, end]` pairs and a question about overlap, merging, counting simultaneous
  usage, or selecting a maximum compatible subset, I know the problem becomes easy once
  sorted — the real skill is picking the sort key that matches the question, not writing
  the scan itself."
- **Decision framing (good for showing you know the two default choices and why):** "There
  are two default sort keys, and they answer different questions: sort by start time when
  the question is about detecting or merging overlap, sort by end time when the question is
  about greedily selecting the maximum number of compatible intervals. I'd say explicitly
  which question I'm answering before picking the sort key, since picking wrong silently
  produces a suboptimal, not obviously broken, answer."
- **Generalization framing (good for tying in the heap-based variant):** "For 'how many
  resources are needed simultaneously' questions, sorting alone isn't enough — I need a
  sweep line with a heap of active end times, since I'm tracking a running count over time,
  not just comparing pairs. I'd name that as the third template in this family, distinct
  from the two pure-sort approaches."

### Vocabulary Builder

- **sort key** (n. phrase) — the field used to order the input before a linear scan; in
  this pattern the choice of sort key *is* the core algorithmic decision, not a minor
  implementation detail.
- **sweep line** (n. phrase) — processing sorted events over time while maintaining running
  state (here, a heap of active end times); the technique that extends this pattern beyond
  simple merge/detect questions into concurrency-counting questions.
- **boundary convention** (n. phrase) — whether touching endpoints count as overlapping
  (`start <= end`) or not (`start < end`); worth confirming with the interviewer early,
  since both conventions appear across different variants of this problem family.
- **"…is what makes the rest of the problem a simple linear scan"** — a reusable phrase for
  crediting the sort step as doing the real algorithmic work, with everything after it
  being comparatively mechanical.
- **exchange argument** (n. phrase) — the proof technique (shared with the Greedy pattern,
  see `../greedy/PATTERN.md`) that justifies *why* sorting by end time specifically
  maximizes the number of compatible intervals kept.
