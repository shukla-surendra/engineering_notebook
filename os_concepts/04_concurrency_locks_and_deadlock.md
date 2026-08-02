# Operating Systems, Part 4: Concurrency — Race Conditions, Locks & Deadlock

[Part 1](01_processes_and_threads.md) established that threads share their process's
address space. This part covers the consequence: what goes wrong when multiple threads
touch that shared state at the same time, and the mechanisms used to make it safe.

## The Core Problem: Interleaving Is Not Optional

A single line of source code like `counter += 1` is not one atomic hardware operation —
it's typically three: read `counter` into a register, increment the register, write it
back. On a multi-core machine (or even one core with preemptive scheduling — see
[Part 2](02_cpu_scheduling.md)), two threads executing that line can **interleave** their
three steps in any order the scheduler happens to produce. If thread A reads `counter` (say
it's 5), gets preempted before writing back, and thread B also reads `counter` (still 5),
increments, and writes 6 — then A resumes, increments its stale copy, and writes 6 as well.
Two increments happened; the counter only went up by one. This is a **race condition**: the
correctness of the result depends on the timing of thread interleaving, which is
nondeterministic and not something your code controls.

The dangerous part isn't that this is rare — it's that it's timing-dependent. The buggy
version and the correct version produce identical output on most runs, then fail under
specific load or scheduling conditions, which is exactly why race conditions are
notoriously hard to reproduce and show up disproportionately in production, not in tests
run on a quiet machine.

## Locks: The Mechanism That Removes the Interleaving

A **mutex** (mutual exclusion lock) makes a sequence of operations — a **critical
section** — appear atomic to other threads, by guaranteeing only one thread can hold the
lock at a time. Any other thread that calls `lock()` while it's held **blocks** (gets moved
out of the ready queue — see [Part 2](02_cpu_scheduling.md#the-building-blocks-states-and-the-ready-queue))
until the holder calls `unlock()`. `counter += 1` wrapped in `lock()`/`unlock()` can no
longer interleave with another thread's increment, because the second thread can't even
begin its three steps until the first has fully finished and released the lock.

The **invariant** a correctly-used lock protects is: "only one thread is ever inside this
critical section at a time." Every concurrency bug in this space is really a violation of
some invariant like that one — the general skill isn't memorizing "use a mutex here," it's
identifying what invariant needs to hold and which operations must be atomic together to
preserve it.

**Locks aren't the only mechanism** — they're the general-purpose one. Narrower tools trade
generality for speed: **atomic operations** (hardware-supported compare-and-swap,
fetch-and-add) handle single-variable updates like the counter above without the overhead
of blocking/waking a thread at all. **Semaphores** generalize a lock to allow up to N
concurrent holders, useful for bounding concurrent access to a pool of N identical
resources rather than enforcing strict single-ownership.

## Deadlock: What Happens When Locking Itself Goes Wrong

Locks solve races but introduce a new failure mode. **Deadlock** is a cycle of threads each
holding a resource the next one in the cycle is waiting for, so none can ever proceed. The
classic two-thread case: thread A holds lock 1 and waits for lock 2; thread B holds lock 2
and waits for lock 1. Neither will ever release what the other needs — permanent standstill,
not a slowdown.

Deadlock requires all four of these conditions simultaneously (the **Coffman conditions**)
— which is exactly why avoiding deadlock is about breaking just one of them, not solving
some harder general problem:

| Condition | Meaning |
|---|---|
| Mutual exclusion | A resource can only be held by one thread at a time |
| Hold and wait | A thread holds one resource while waiting for another |
| No preemption | A resource can't be forcibly taken from the thread holding it |
| Circular wait | A cycle of threads, each waiting on the next |

In practice, the condition that's cheapest to break is **circular wait**: enforce a global
**lock ordering** — every thread that needs multiple locks must acquire them in the same
predetermined order (e.g. always lock the account with the lower ID first in a
funds-transfer operation). If every thread acquires locks in the same order, a cycle can
never form, because the last lock in any thread's sequence is never one that comes earlier
in another thread's sequence.

## Why This Matters in Practice

**Livelock and starvation are the other two failure modes locking can introduce**, distinct
from deadlock and worth being able to tell apart in an interview: livelock is threads
actively changing state in response to each other (e.g. both repeatedly backing off and
retrying) without any of them making real progress — busy, but stuck. Starvation
(introduced already in [Part 2](02_cpu_scheduling.md#scheduling-policies-the-actual-mechanisms))
is one thread perpetually losing the race for a lock to other threads, even though the lock
itself isn't stuck.

**Granularity is a real design decision, not just "add a lock."** One coarse lock around
an entire data structure is simple and easy to reason about but serializes all access to
it, killing concurrency (every thread queues up for the one lock even when they're touching
unrelated parts of the structure). Fine-grained locking (e.g. one lock per bucket in a hash
map) allows more true parallelism but multiplies the number of locks a thread might need to
juggle at once — directly increasing deadlock risk if lock ordering isn't disciplined. This
is the actual trade-off behind "why not just lock everything" — it's not free, it's
throughput traded for safety, same shape as [Part 2's fairness-vs-throughput
trade-off](02_cpu_scheduling.md#the-core-design-question-fairness-throughput-or-responsiveness).

**Lock-free isn't "no synchronization"** — it's synchronization built from atomic
hardware primitives (compare-and-swap loops) instead of blocking. It avoids the cost of a
thread being put to sleep and woken by the scheduler, which matters on the hot path of a
high-throughput system, but it's substantially harder to reason about correctly than a
plain mutex and should be reached for only when profiling actually shows lock contention as
the bottleneck.

## Quick Self-Check

- Why is `counter += 1` unsafe across threads without synchronization, even though it looks
  like one line of code?
- Name the four Coffman conditions for deadlock, and explain why breaking just one is
  enough to prevent it.
- Why does a global lock-ordering convention prevent circular wait specifically?
- What's the difference between deadlock, livelock, and starvation?
- Why does fine-grained locking increase deadlock risk even though it improves throughput?

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Invariant framing (the default):** "I wouldn't explain a race condition as 'threads
  running at the same time is dangerous' — I'd name the actual invariant that breaks: an
  operation that looks atomic in source code (`counter += 1`) is really read-modify-write,
  and interleaving those three steps across threads violates the 'only one increment
  happens at a time' invariant the code implicitly assumes."
- **Structural framing (good for 'how do you prevent deadlock'):** "I'd name the four
  Coffman conditions and point out you only need to break one — in practice that's almost
  always circular wait, via a global lock-ordering convention, because it's the cheapest of
  the four to enforce without giving up mutual exclusion or blocking semantics you actually
  need."
- **Cost/trade-off framing (good for 'why not just lock everything'):** "I'd frame lock
  granularity as a direct throughput-versus-safety trade-off, same shape as the scheduling
  trade-offs in Part 2 — one coarse lock is easy to reason about but serializes everything;
  fine-grained locks parallelize better but multiply the deadlock surface if ordering isn't
  disciplined."

### Vocabulary Builder

- **race condition** (n.) — a bug where correctness depends on the nondeterministic timing
  of thread interleaving.
- **critical section** (n.) — a sequence of operations that must appear atomic to other
  threads; what a lock protects.
- **Coffman conditions** (n. phrase) — the four conditions (mutual exclusion, hold-and-wait,
  no preemption, circular wait) that must all hold simultaneously for deadlock to occur.
- **lock ordering** (n. phrase) — a global convention for the order in which multiple locks
  must be acquired, the standard fix for circular wait.
- **livelock** (n.) — threads actively responding to each other without making progress,
  distinct from deadlock (which is a static standstill).
- **"…busy, but stuck"** — a compact phrase distinguishing livelock (looks active) from
  deadlock (looks frozen) when narrating the difference out loud.

---

**Previous:** [Part 3: Virtual Memory & Paging](03_virtual_memory_and_paging.md)  |  **Next:** [Part 5: Context Switching & the Kernel/User Boundary](05_context_switching_and_kernel_boundary.md)
