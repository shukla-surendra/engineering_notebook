# Operating Systems, Part 2: CPU Scheduling

[Part 1](01_processes_and_threads.md) covered what a process and a thread each own. This
part covers how the OS decides *which* of the many runnable threads actually gets the CPU
right now, and for how long — the scheduler.

## The Core Design Question: Fairness, Throughput, or Responsiveness?

A CPU has far fewer cores than there are runnable threads competing for them at any given
moment. The scheduler's job is to pick, and it's optimizing for goals that actively trade
off against each other:

- **Throughput** — total work completed per unit time. Favored by running each thread for
  long, uninterrupted stretches (fewer context switches, better cache locality).
- **Latency / responsiveness** — how quickly a thread that just became ready gets to run.
  Favored by switching threads frequently and letting new/interactive work preempt
  long-running work.
- **Fairness** — no thread starves indefinitely waiting for the CPU.

A scheduler tuned purely for throughput would let a batch job run for seconds at a stretch
— great for total work done, terrible if a user just clicked something and expects the UI
to respond in milliseconds. A scheduler tuned purely for latency would context-switch so
often that most CPU time goes to switching overhead instead of actual work (see [Part 5's
discussion of context-switch cost](05_context_switching_and_kernel_boundary.md)). Real
schedulers are a tuned compromise, not a pure optimization for any one of these.

## The Building Blocks: States and the Ready Queue

Every thread is, at any instant, in one of a small number of states:

| State | Meaning |
|---|---|
| **Running** | Actually executing on a CPU core right now |
| **Ready** | Runnable, waiting in the scheduler's ready queue for a core to become free |
| **Blocked / Waiting** | Not runnable — waiting on I/O, a lock, a `sleep()`, a network response |
| **Terminated** | Finished; resources being reclaimed |

The scheduler's core loop is: when a core is free (the running thread blocked, was
preempted, or finished), pick the next thread from the **ready queue** to run. Everything
below is really about *how that queue is ordered* and *how long a chosen thread is allowed
to keep running before the scheduler reconsiders*.

## Scheduling Policies: The Actual Mechanisms

- **First-Come, First-Served (FCFS).** Simplest possible — run threads in arrival order to
  completion. Terrible in practice: one long thread blocks every short thread behind it
  from running at all (the "convoy effect"), so responsiveness for anything queued behind a
  long job collapses.
- **Shortest Job First (SJF) / Shortest Remaining Time.** Provably optimal for average wait
  time — but requires knowing each thread's remaining runtime in advance, which the OS
  generally doesn't. Mostly a theoretical baseline other policies are measured against, not
  something deployed as-is.
- **Round Robin.** Give every thread a fixed **time slice** (a "quantum," typically single
  or low double-digit milliseconds); when it expires, preempt the running thread and move
  it to the back of the ready queue. This is the actual mechanism behind
  responsiveness — no thread can hog the CPU for longer than one quantum, no matter how
  long its total work is. The quantum size is itself a trade-off: too short and you burn
  most of your CPU time context-switching instead of running anything; too long and you're
  back to FCFS-like unresponsiveness.
- **Priority scheduling (and multilevel feedback queues).** Give some threads higher
  priority so they preempt lower-priority ones. The dangerous failure mode this introduces
  is **starvation** — a low-priority thread that never gets to run because higher-priority
  work keeps arriving. The standard fix is **aging**: gradually raise a waiting thread's
  effective priority the longer it's been stuck in the ready queue, guaranteeing it
  eventually wins. Real-world schedulers (Linux's CFS, Windows' scheduler) are
  multilevel-feedback systems: they blend priority with round-robin-style time slicing and
  continuously re-rank threads based on observed behavior (an I/O-bound thread that keeps
  blocking quickly gets treated as "interactive" and favored for responsiveness; a
  CPU-bound thread that keeps using its full quantum gets treated as "batch" and given
  longer, less frequent slices).

## Why This Matters in Practice

**I/O-bound vs. CPU-bound threads get fundamentally different treatment.** A thread that
mostly blocks on I/O (a network call, a disk read) uses very little CPU time when it *does*
run, then quickly gives the CPU back by blocking again. A good scheduler favors these
threads for the CPU the instant they become ready, because doing so costs little total
throughput and hugely improves the responsiveness of I/O-heavy work (exactly the kind of
work a request-handling server thread does). A CPU-bound thread (tight numerical loop) is
the opposite: it wants long uninterrupted slices, not frequent tiny ones. A scheduler that
can't tell these apart — or that's mistuned for the wrong workload mix — is a common root
cause behind "why does my server feel laggy even though CPU utilization looks fine":
overall utilization can be low while *latency-sensitive* threads are still waiting too long
in the ready queue behind CPU-bound work.

**"Why is my program running slower with more threads than cores?"** If you spin up more
CPU-bound threads than you have cores, the scheduler must time-slice them — each thread now
gets less wall-clock CPU time and pays a context-switch cost every time it's swapped out
(state save/restore, cache and TLB effects — see [Part 5](05_context_switching_and_kernel_boundary.md)).
Past a certain thread count for CPU-bound work, adding more threads makes total throughput
*worse*, not better, purely from scheduling overhead — this is the mechanical reason
CPU-bound thread pools are typically sized to roughly the core count, while I/O-bound pools
can be sized much larger (those threads spend most of their time blocked, not competing for
the CPU).

## Quick Self-Check

- Why does Round Robin fix the convoy-effect problem that First-Come-First-Served has, and
  what determines whether a given quantum size helps or hurts?
- What's starvation, mechanically, and what's the standard fix?
- Why would a scheduler give shorter, more frequent time slices to a CPU-bound thread and
  near-immediate priority to an I/O-bound thread the moment it's ready to run again?
- Why can adding more CPU-bound threads than you have cores make total throughput *worse*?

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Trade-off framing (the default):** "I wouldn't name scheduling algorithms as a list —
  I'd frame the whole topic as one trade-off: throughput versus responsiveness versus
  fairness, and every named algorithm (FCFS, Round Robin, priority) is a different point on
  that trade-off, not an arbitrary alternative."
- **Mechanism framing (good for 'what actually happens when a quantum expires'):** "A timer
  interrupt fires, the kernel's scheduler runs, the currently running thread gets moved
  back to the ready queue, and the next thread in line gets its registers restored and
  starts executing. That's the same context-save/restore mechanism from Part 5, just
  triggered by a timer instead of a blocking call."
- **Debugging framing (good for 'why is my service laggy despite low CPU%'):** "Low
  aggregate CPU utilization doesn't mean nothing is waiting — I'd check whether
  latency-sensitive threads are sitting in the ready queue behind CPU-bound work, which a
  scheduler mistuned for the workload mix will do even when the CPU has idle-looking
  headroom on average."

### Vocabulary Builder

- **quantum / time slice** (n.) — the fixed amount of CPU time a thread gets before Round
  Robin-style preemption reconsiders who runs next.
- **starvation** (n.) — a thread that never gets scheduled because other work keeps
  outranking it; fixed by aging.
- **aging** (n., scheduling-specific) — gradually raising a waiting thread's effective
  priority the longer it waits, guaranteeing eventual scheduling.
- **convoy effect** (n. phrase) — one long-running unit of work blocking many short ones
  queued behind it, the core failure mode FCFS has and Round Robin fixes.
- **"…a different point on the same trade-off, not an arbitrary alternative"** — a reusable
  phrase for framing a list of named algorithms/techniques as one underlying axis instead
  of a memorized list.

---

**Previous:** [Part 1: Processes vs. Threads](01_processes_and_threads.md)  |  **Next:** [Part 3: Virtual Memory & Paging](03_virtual_memory_and_paging.md)
