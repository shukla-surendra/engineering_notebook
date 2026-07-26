# 4. Task Scheduler

**Difficulty:** Medium
**Topic:** Heap / Priority Queue
**Pattern:** Greedy scheduling by frequency, with a cooldown constraint

## Problem
Given a list of CPU `tasks` (each a letter) and a cooldown `n` (a task must wait at least
`n` intervals before the same type can run again), return the minimum number of intervals
needed to finish all tasks (idle intervals allowed).

## Examples
```
Input: tasks = ["A","A","A","B","B","B"], n = 2 -> 8   (A B idle A B idle A B)
Input: tasks = ["A","A","A","B","B","B"], n = 0 -> 6
```

## Approach
There's a neat closed-form using just counts: let `max_freq` be the highest frequency of
any task, and `max_count` the number of distinct tasks that hit that max frequency. The
most frequent task defines `(max_freq - 1)` full "chunks" of size `(n + 1)`, plus one
final partial chunk holding all `max_count` most-frequent tasks. The answer is
`max((max_freq - 1) * (n + 1) + max_count, len(tasks))` — the `len(tasks)` floor covers the
case where there are enough distinct tasks to fill every idle slot naturally (no idling
needed at all). A priority-queue simulation (pick the most frequent available task each
round) arrives at the same answer and is the more "obviously correct" approach to derive
under interview pressure.

## Why This Approach (Generalizing the Pattern)
This problem is a concrete instance of **Greedy scheduling by frequency, with a cooldown
constraint**, which itself belongs to the broader **Heap / Priority Queue** family of
techniques. If the specific trick above feels like it came out of nowhere, that's the
signal to step back and read [`../PATTERN.md`](../PATTERN.md) — it covers how to
recognize this family of problems in general (not just this one), the reusable template
you can write from memory, the usual variations, and the mistakes people make applying
it. Coming back to re-read this problem's approach afterward should make the specific
choices here feel inevitable rather than clever.

## Complexity
- Time: O(n) counting-based approach; O(total intervals · log 26) simulation approach
- Space: O(1) (26-letter alphabet)

## Solution
Runnable, with sample test cases at the bottom (`python3 heap_priority_queue/04_task_scheduler/solution.py`):

```python
--8<-- "heap_priority_queue/04_task_scheduler/solution.py"
```

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Simulation-first framing (the safer default under interview pressure):** "I'd actually
  reach for the priority-queue simulation first, not the closed-form formula — greedily run
  the most frequent available task each round, respecting cooldown. It's slower to derive
  the formula live and easier to get an off-by-one wrong in front of someone watching, so I
  build the correct-but-simpler version first and mention the formula as an optimization."
- **Derivation framing (good if asked to explain the closed-form):** "The crux of it is
  that the most frequent task defines the schedule's shape: it forces `max_freq - 1` full
  cooldown chunks of size `n+1`, plus one final partial chunk. Everything else either fills
  idle slots inside those chunks for free, or — if there are enough distinct tasks — there's
  no idling at all, which is why the answer is a `max` against `len(tasks)`."
- **Generalization framing (good for tying it back to the heap family):** "This is greedy
  scheduling by frequency under a cooldown constraint — I'd name it as a heap-family problem
  because the simulation approach is the same 'always take the current max' idea as Last
  Stone Weight, just with an added constraint on *when* something is eligible to be taken
  again."

### Vocabulary Builder

- **closed-form** (adj.) — a formula computed directly from a few summary statistics
  (here, `max_freq` and `max_count`) rather than by simulating the process step by step.
- **idle slot** (n. phrase) — a scheduled gap where no task runs because everything
  eligible is still on cooldown; the formula's whole job is counting how many of these are
  unavoidable.
- **"…is easier to get an off-by-one wrong in front of someone watching"** — an honest,
  reusable phrase for explaining why you'd default to the simpler-but-slower approach in a
  live setting even when you know a faster one exists.
- **cooldown constraint** (n. phrase) — a rule that an action can't repeat until some
  amount of time/steps has passed; naming the constraint explicitly clarifies what makes
  this different from ordinary greedy scheduling.
