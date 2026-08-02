# Operating Systems Interview Prep

Core OS fundamentals as asked at top-tier companies — usually a 30-45 minute
"CS fundamentals" round (common at Google, Meta, Amazon, Microsoft, and most
infra-heavy teams elsewhere) that sits alongside the coding round rather than
replacing it. The bar isn't reciting textbook definitions; it's being able to
explain *why* a mechanism exists (what problem it solves, what breaks without
it) and reason about a concrete scenario ("what happens if two threads
increment the same counter without a lock").

Each doc follows the same first-principles structure as
[`system_design_foundation/prerequisite_concepts/`](../system_design_foundation/prerequisite_concepts/01_performance_and_scale.md):
problem → mechanism → why it matters practically, with a worked example, a
**Quick Self-Check**, and an **Articulate It** section for how to say it out
loud in an interview.

## How to use this

1. Read in order — later docs assume earlier ones (concurrency assumes you
   already know what a thread is; context switching assumes you know what a
   process's state consists of).
2. Each doc ends with **Quick Self-Check** questions — answer them out loud,
   unscripted, before moving on. If you can't, re-read the mechanism section,
   don't just reread the definition.
3. These are conceptual primers, not coding problems — there's no
   `solution.py` here. If a topic below has a natural coding companion, it's
   cross-linked from that doc (e.g. concurrency primitives link to problems
   that use locks).

## Topics (in suggested order)

| # | Doc | Topic |
|---|-----|-------|
| 1 | [`01_processes_and_threads.md`](01_processes_and_threads.md) | Processes vs. Threads |
| 2 | [`02_cpu_scheduling.md`](02_cpu_scheduling.md) | CPU Scheduling |
| 3 | [`03_virtual_memory_and_paging.md`](03_virtual_memory_and_paging.md) | Virtual Memory & Paging |
| 4 | [`04_concurrency_locks_and_deadlock.md`](04_concurrency_locks_and_deadlock.md) | Concurrency: Race Conditions, Locks & Deadlock |
| 5 | [`05_context_switching_and_kernel_boundary.md`](05_context_switching_and_kernel_boundary.md) | Context Switching & the Kernel/User Boundary |
| 6 | [`06_interprocess_communication.md`](06_interprocess_communication.md) | Inter-Process Communication (IPC) |

## Why these six

This is the set that actually recurs across FAANG-style loops, not an
exhaustive undergrad-OS syllabus:

- **Processes vs. threads** and **concurrency** show up constantly as
  "design a thread-safe cache" or "what's the difference between X and Y"
  warm-up questions.
- **CPU scheduling** and **context switching** are where "why is my
  multi-threaded code slower than expected" debugging questions live.
- **Virtual memory & paging** underpins nearly every "why did this process
  get OOM-killed" or "what is a page fault" question, and is a direct
  prerequisite for understanding the container/cgroups material that shows
  up in infra-adjacent system design rounds.
- **IPC** is the connective tissue question once a candidate has established
  they understand processes — "how would two processes on the same machine
  talk to each other" — and bridges naturally into the distributed-systems
  material in [`system_design_foundation/`](../system_design_foundation/README.md).

Boot sequence, device drivers, and filesystem internals are intentionally
out of scope — they're asked far less frequently in general SWE loops and
are closer to a systems/kernel-specialist track than a general fundamentals
round.
