# Operating Systems, Part 5: Context Switching & the Kernel/User Boundary

[Part 2](02_cpu_scheduling.md) referenced context switches as the mechanism behind
preemption without explaining what one actually costs or involves. This part covers that
mechanism directly, plus the related but distinct boundary between user mode and kernel
mode that every system call crosses.

## What a Context Switch Actually Is

A CPU core can only be "inside" one thread's execution at a time — one set of register
values, one program counter, one stack pointer. A **context switch** is the act of saving
all of that state for the thread currently running and loading the saved state for the
next thread the scheduler has picked, so execution can resume exactly where that thread
left off, potentially much later and on state a completely different thread had just been
using.

This isn't free, and the cost has two very different components:

- **Direct cost** — saving/restoring registers, program counter, and stack pointer; the
  kernel updating its bookkeeping (moving the old thread to ready/blocked, updating
  scheduling data structures). This part is fast — on the order of hundreds of nanoseconds
  to low microseconds — and is pure overhead paid on every switch regardless of what the
  threads involved are doing.
- **Indirect cost** — the new thread starts with a **cold cache**. The CPU's L1/L2 caches
  and the TLB (from [Part 3](03_virtual_memory_and_paging.md#paging-the-actual-translation-mechanism))
  were full of the *previous* thread's data and address translations; the new thread has to
  refill them from scratch as it runs, which shows up as slower-than-expected execution for
  a while after the switch, not as switch time itself. If the new thread belongs to a
  **different process** (not just a different thread in the same process), this cost is
  worse: the MMU's page table pointer changes too, which historically flushed the entire
  TLB (modern CPUs mitigate this with tagged TLB entries, but the address-space change is
  still strictly more expensive than a same-process thread switch).

This is precisely why [Part 2's Round Robin discussion](02_cpu_scheduling.md#scheduling-policies-the-actual-mechanisms)
frames the quantum size as a real trade-off rather than "shorter is always more
responsive" — a quantum too short means the indirect cache-refill cost dominates and total
useful throughput drops, even though each individual switch's direct cost looks tiny.

## The Kernel/User Boundary: A Different Kind of Switch

Separately from switching *between threads*, there's a boundary the CPU crosses *within* a
single thread's execution: **user mode** versus **kernel mode**. This is a hardware-enforced
privilege level, not a software convention — the CPU itself refuses to execute certain
instructions (direct hardware access, modifying page tables, etc.) unless it's currently in
kernel mode.

- **User mode** — where ordinary application code runs. Cannot directly touch hardware,
  cannot modify another process's memory, cannot arbitrarily change its own page table.
- **Kernel mode** — where the OS kernel runs. Full hardware access, including the
  privileged instructions user mode is blocked from.

A **system call** (`read()`, `write()`, `mmap()`, etc.) is the controlled doorway between
them: user code executes a special trap instruction, the CPU switches to kernel mode and
jumps to a fixed, kernel-controlled entry point (never to an address user code chose — that
constraint is exactly what keeps the boundary meaningful as a security guarantee, not just
a performance detail), the kernel does the privileged work on the process's behalf, then
switches back to user mode and returns control. This is also the same trap mechanism behind
a page fault ([Part 3](03_virtual_memory_and_paging.md#demand-paging-and-the-page-fault)) —
the MMU detects an invalid access, traps into kernel mode, the kernel resolves it (loads the
page, or kills the process if the access was genuinely invalid), and traps back.

A user-mode-to-kernel-mode transition is a real cost, similar in order of magnitude to (and
often layered on top of) a context switch — which is the mechanical reason why a
syscall-heavy program (many small `read()`/`write()` calls) is measurably slower than one
that batches the same total I/O into fewer, larger calls: each syscall pays the trap
overhead independent of how much actual work it does.

## Why This Matters in Practice

**"Why does my thread-pool-heavy service have high CPU usage but low actual throughput?"**
If threads are being switched far more often than their useful work justifies — too many
threads competing for too few cores, or a quantum tuned too short for the workload — a real
share of "CPU busy" time is context-switch and cache-refill overhead, not application work.
This is directly measurable (`vmstat`'s context-switch counter, or equivalent) and is one
of the first things to check before assuming a CPU-bound service needs more cores rather
than fewer, better-sized threads.

**Why user-mode/kernel-mode matters for security, not just performance.** The boundary is
what makes it meaningful to say a sandboxed or unprivileged process "can't" do something —
the restriction isn't a permission check application code could route around, it's enforced
by the CPU refusing to execute the instruction outside kernel mode at all. Any exploit that
claims to escalate privilege is, mechanically, finding a way to get arbitrary code executed
*in* kernel mode rather than requesting a syscall to do it — which is why kernel
vulnerabilities are treated with such disproportionate severity compared to bugs confined
to user-mode application code.

**Why async/event-driven I/O models exist at all.** A thread blocked waiting on a syscall
(e.g. waiting for a slow disk read) is not doing useful work but still holds all the memory
and scheduling overhead of a thread. Event-driven models (an event loop plus non-blocking
syscalls) let one thread service many pending I/O operations without needing a
context-switch-and-block per operation — directly attacking the indirect cost described
above rather than trying to make individual context switches cheaper.

## Quick Self-Check

- What two components make up the total cost of a context switch, and why is the indirect
  one often larger than the direct one?
- Why is switching between two threads in the *same* process typically cheaper than
  switching to a thread in a *different* process?
- What makes the kernel/user boundary a hardware guarantee rather than a software
  convention — what would break if it were merely a convention?
- Why does batching many small `read()` calls into fewer, larger ones improve performance
  independent of the total bytes read?

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Two-costs framing (the default):** "I wouldn't describe a context switch as just
  'saving and restoring registers' — I'd split it into the direct cost (fast, fixed,
  register/bookkeeping overhead) and the indirect cost (a cold cache and TLB the new thread
  has to refill), because the indirect cost is usually the bigger and more variable one,
  and it's what actually explains why over-threading a CPU-bound workload hurts
  throughput."
- **Enforcement framing (good for 'why can't user code just do X'):** "I wouldn't say user
  code 'isn't allowed' to touch hardware directly as if it's a permission setting — the CPU
  itself refuses to execute those instructions outside kernel mode. That's what makes a
  syscall a real boundary and not just a convention user code could choose to skip."
- **Unified-mechanism framing (good for connecting syscalls and page faults):** "I'd point
  out that a syscall and a page fault use the exact same underlying trap mechanism — a
  controlled jump from user mode into a fixed kernel entry point — just triggered
  explicitly by application code in one case and implicitly by the MMU in the other."

### Vocabulary Builder

- **context switch** (n.) — saving one thread's execution state and loading another's so a
  core can resume a different thread.
- **cold cache** (n. phrase) — the state of L1/L2 cache and TLB immediately after a switch,
  before the new thread has re-populated them with its own data/translations.
- **trap** (n./v.) — a controlled, hardware-triggered jump from user mode into a fixed
  kernel entry point, used by both syscalls and page faults.
- **user mode / kernel mode** (n. phrase) — the hardware-enforced privilege levels that
  make the kernel/user boundary a CPU guarantee, not a software convention.
- **"…enforced by the CPU refusing to execute the instruction, not by convention"** — a
  reusable phrase for grounding any "why can't X" security/isolation claim in hardware
  mechanism rather than policy.

---

**Previous:** [Part 4: Concurrency — Race Conditions, Locks & Deadlock](04_concurrency_locks_and_deadlock.md)  |  **Next:** [Part 6: Inter-Process Communication (IPC)](06_interprocess_communication.md)
