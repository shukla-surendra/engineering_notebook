# Operating Systems, Part 3: Virtual Memory & Paging

[Part 1](01_processes_and_threads.md) mentioned that each process gets its own address
space, translated through a page table. This part unpacks that mechanism in full — why it
exists, how paging actually works, and what happens when it fails (a page fault).

## The Core Design Question: Why Not Just Give Processes Real Physical Addresses?

Early systems did exactly that, and it created two hard problems:

1. **Isolation.** If a process's addresses are real physical RAM addresses, nothing stops
   it from writing to another process's memory — one bug takes down the whole machine.
2. **Fitting more than fits.** Physical RAM is finite. If every process's total memory
   needs must physically fit in RAM simultaneously, you can run far fewer programs at once
   than users expect, and a program can't use more memory than the machine physically has.

**Virtual memory** solves both with one indirection: every process gets its own virtual
address space (e.g. a full 48-bit range on a modern 64-bit system) that has nothing to do
with actual physical RAM layout. The **MMU** translates every virtual address a program
uses into a physical one, using a per-process **page table**, on every single memory
access. Isolation falls out for free (Part 1's wall). Overcommitting RAM falls out because
not every virtual page needs to be backed by physical RAM at the same time — which is
exactly what paging enables.

## Paging: The Actual Translation Mechanism

Both virtual and physical memory are divided into fixed-size chunks called **pages**
(typically 4 KB). The page table maps virtual page numbers to physical **frame** numbers.
A virtual address splits into two parts: a page number (looked up in the table) and an
offset within that page (unchanged by translation — it's just added to wherever the frame
starts).

| Field | Comes from |
|---|---|
| Virtual page number | Looked up in the page table → physical frame number |
| Offset within page | Copied unchanged — same offset in the physical frame |

Doing this lookup for *every* memory access would be slow if it meant a full page-table
walk each time, so hardware caches recent translations in the **TLB (Translation
Lookaside Buffer)** — a small, extremely fast cache living right next to the core. A TLB
hit means the physical address is available almost immediately; a **TLB miss** means the
MMU has to walk the page table structure in memory, which costs many times longer. This is
why memory-access patterns that jump around unpredictably across many different pages
(poor locality) are measurably slower than patterns that stay within a small working set of
pages — not because of cache-line effects alone, but because they thrash the TLB too.

## Demand Paging and the Page Fault

Because not every virtual page needs a physical frame simultaneously, the OS practices
**demand paging**: a page is only loaded into physical RAM the first time it's actually
accessed, not when the process starts. Until then, the page table entry for that virtual
page is marked *not present*.

A **page fault** is the MMU trapping into the kernel because a program touched a virtual
address whose page table entry says "not present" (or otherwise invalid). The kernel then
does one of two very different things, and the distinction matters:

- **Minor/soft fault** — the page exists somewhere accessible (e.g. it was evicted from RAM
  but is still findable, or it's a freshly requested heap page that just needs a physical
  frame assigned and zeroed). Resolved quickly, no disk I/O.
- **Major/hard fault** — the page's actual data must be read from disk (a memory-mapped
  file, or a page that was **swapped out** to disk under memory pressure). This is orders
  of magnitude slower than a minor fault — disk/SSD latency versus RAM latency — and is the
  single biggest reason a machine that's genuinely out of physical memory feels like it's
  crawled to a halt: every touch of swapped-out memory now costs a disk round trip instead
  of a RAM access.

## Why This Matters in Practice

**"Why did my process get OOM-killed even though `top` showed free memory?"** Virtual
memory lets a process *reserve* far more address space than it will ever actually touch
(lazy allocation — the same "pay only for what you use" idea as `fork()`'s
copy-on-write in [Part 1](01_processes_and_threads.md#a-concrete-worked-example-fork-and-copy-on-write)).
Reserving 10 GB of virtual address space costs almost nothing until pages are actually
written to, at which point real physical frames get committed. The Linux OOM killer acts
when the system is genuinely out of *committable* physical memory (+ swap), not virtual
address space — a process can have a huge virtual size (`VSZ`) and a comparatively tiny
resident set (`RSS`, the physical memory actually backing it) and be perfectly healthy.

**Why "thrashing" is a real failure mode, not just slowness.** If the sum of what all
running processes are actively touching (their combined **working sets**) exceeds physical
RAM, the OS spends most of its time evicting pages just to immediately fault them back in
— useful work approaches zero while the system is fully "busy" servicing page faults. This
is the memory-system analogue of the convoy effect from [Part 2](02_cpu_scheduling.md):
a resource that's nominally available but structurally can't make forward progress on the
actual work.

**Why paging enables things beyond just "more memory than you have RAM."** Memory-mapped
files (`mmap`), shared memory between processes (a fast IPC mechanism — see
[Part 6](06_interprocess_communication.md)), and copy-on-write `fork()` are all built on
the same primitive: multiple page table entries (possibly in different processes) pointing
at the same physical frame, with the kernel controlling access via the present/read-only
bits it can flip on any entry.

## Quick Self-Check

- Why does virtual memory give processes isolation "for free" — what specifically would
  break without a per-process page table?
- What's the difference between a TLB hit, a TLB miss, and a page fault — and which one is
  actually slow enough to involve disk I/O?
- A process shows a 10 GB virtual size but only 200 MB resident. Is that a memory leak?
  What would you check before concluding either way?
- What is thrashing, mechanically, and why does it make a system's "CPU busy" metric
  misleading?

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Indirection-solves-two-problems framing (the default):** "I'd frame virtual memory as
  one layer of indirection solving two problems at once — isolation between processes and
  the ability to overcommit physical RAM — rather than describing paging as an isolated
  fact. Both fall out of the same mechanism: the MMU translating every access through a
  per-process page table."
- **Mechanism framing (good for 'what actually happens on a page fault'):** "I wouldn't say
  'the OS loads the page from disk' as the whole answer — I'd distinguish a minor fault
  (page exists somewhere accessible, resolved fast, no I/O) from a major fault (must be
  read from disk, orders of magnitude slower), because that distinction is exactly what
  explains why a memory-pressured machine feels like it's crawled to a halt."
- **Debugging framing (good for 'is this process leaking memory'):** "I wouldn't just look
  at virtual size — I'd separate reserved-but-untouched virtual address space from actually
  resident physical memory (RSS), since lazy allocation means a huge VSZ with modest RSS is
  often completely healthy, not a leak."

### Vocabulary Builder

- **page table** (n.) — the per-process data structure the MMU uses to translate virtual
  page numbers to physical frame numbers.
- **TLB (Translation Lookaside Buffer)** (n.) — a small hardware cache of recent
  virtual-to-physical translations; a miss means a full page-table walk.
- **demand paging** (n. phrase) — loading a page into physical RAM only on first access,
  not at process start.
- **thrashing** (n./v.) — spending most of the CPU's time evicting and re-faulting pages
  instead of doing useful work, because the combined working set exceeds physical RAM.
- **resident set (RSS) vs. virtual size (VSZ)** (n. phrase) — physical memory actually
  backing a process versus the total address space it has reserved; conflating the two is
  a common false alarm in memory-leak debugging.
- **"…pays for what it actually touches, lazily"** — a reusable phrase connecting demand
  paging here to copy-on-write `fork()` in [Part 1](01_processes_and_threads.md).

---

**Previous:** [Part 2: CPU Scheduling](02_cpu_scheduling.md)  |  **Next:** [Part 4: Concurrency — Race Conditions, Locks & Deadlock](04_concurrency_locks_and_deadlock.md)
