# Operating Systems, Part 1: Processes vs. Threads

This is the entry point for the [`os_concepts/`](README.md) track — the fundamentals every
later doc here assumes. If you can already explain precisely what a thread shares with its
siblings and what it doesn't, skim the tables and jump to
[Part 2: CPU Scheduling](02_cpu_scheduling.md).

## The Core Design Question: How Much Should Concurrent Work Share?

A process and a thread are both "a unit of execution the OS can run," but they sit at
opposite ends of an isolation-vs-sharing trade-off:

- **Process: maximum isolation.** Its own virtual address space, its own file descriptor
  table, its own everything. Two processes can't corrupt each other's memory even if one
  has a bug — the hardware's memory management unit (MMU) enforces the wall.
- **Thread: maximum sharing.** Threads within the same process share that process's address
  space, heap, open files, and global variables. What's private per-thread is just enough
  to let each one run independently: its own stack, its own CPU register state, its own
  program counter, its own thread-local storage.

Neither is "better" — isolation costs you communication overhead (processes can't just read
each other's variables; see [Part 6: IPC](06_interprocess_communication.md)), and sharing
costs you safety (threads *can* corrupt each other's data, which is the entire subject of
[Part 4: Concurrency](04_concurrency_locks_and_deadlock.md)).

## What's Actually Shared vs. Private

| | Shared across threads in a process | Private per thread |
|---|---|---|
| Address space (code, heap, global vars) | Yes | No |
| Open file descriptors | Yes | No |
| Stack | No | Yes — each thread gets its own |
| Registers / program counter | No | Yes — saved/restored on every context switch |
| Thread-local storage (TLS) | No | Yes — explicitly per-thread by design |

| | Two separate processes |
|---|---|
| Address space | Separate — process A cannot dereference a pointer valid in process B |
| Open file descriptors | Separate (unless explicitly duplicated via `fork` or passed via IPC) |
| Communication | Must go through the kernel — pipes, sockets, shared memory (see [Part 6](06_interprocess_communication.md)) |
| A crash in one | Does not directly corrupt the other — the MMU wall holds |

**The mechanism that makes the wall real**: every process has its own **page table** — the
data structure the MMU uses to translate virtual addresses to physical ones (full mechanism
in [Part 3](03_virtual_memory_and_paging.md)). Process A's virtual address `0x1000` and
process B's virtual address `0x1000` map to *different* physical RAM. That's not a
convention the OS asks programs to respect — it's enforced in hardware on every single
memory access. Threads within one process share that one page table, which is precisely why
they can see each other's data: there's only one translation table to look things up in.

## Why This Trade-off Matters in Practice

**Crash blast radius.** A segfault in one thread takes down the whole process — every
thread in it, since they share one address space and the OS can't selectively preserve
"the parts that were fine." A crash in one process leaves sibling processes running. This
is why a browser runs each tab as a separate *process* (a crashed/hung tab doesn't take the
browser down) but uses *threads* within a tab's renderer process for parsing, layout, and
paint (those need to share the DOM without marshaling it across a process boundary on every
frame).

**Creation cost.** Spinning up a new process means the OS allocates a fresh address space
and page table — expensive. Spinning up a new thread reuses the existing process's address
space — an order of magnitude cheaper. This is why a high-concurrency server (a thread pool
handling thousands of connections) uses threads, not a process per connection, and why
`fork()` (new process) is measurably slower than `pthread_create()` (new thread) on the
same machine.

**Communication cost.** Threads communicate by just reading/writing shared memory — fast,
but requires explicit synchronization to be correct (locks, atomics). Processes must
communicate through the kernel — slower per-message, but the isolation means you don't need
to reason about a stray write from an unrelated part of the program corrupting your data.

## A Concrete Worked Example: `fork()` and Copy-on-Write

`fork()` creates a new process that's (conceptually) an exact duplicate of the calling
process — same code, same heap contents, same open files. Naively, that sounds like it
should copy the entire address space, which would be prohibitively slow for a large
process. In practice, modern OSes use **copy-on-write (COW)**: `fork()` gives the child a
new page table whose entries point at the *same physical pages* as the parent, all marked
read-only. Nothing is actually copied yet. Only when either process tries to *write* to a
shared page does the MMU trap into the kernel, which then copies just that one page (a few
KB) and gives the writer its own private copy. A `fork()` immediately followed by `exec()`
(the common pattern for launching a new program, e.g. what a shell does) may end up copying
almost nothing — the child overwrites its address space with the new program before it
ever touches most of the inherited pages. This is the same "pay only for what you actually
use" idea that shows up as lazy allocation in [Part 3's discussion of demand
paging](03_virtual_memory_and_paging.md).

## Quick Self-Check

- If two threads in the same process both declare a local variable with the same name in
  the same function, do they see each other's value? Why or why not?
- Why does a crash in one thread take down every other thread in that process, while a
  crash in one process typically doesn't affect a sibling process?
- Why is creating a new thread cheaper than creating a new process, mechanically — what is
  the new thread *not* allocating that the new process would have to?
- What does copy-on-write actually copy, and at what moment does the copy happen?

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Isolation-vs-sharing framing (the default):** "I wouldn't describe processes and
  threads as 'processes are heavy, threads are light' — that's a symptom, not the cause.
  The actual design axis is isolation versus sharing: a process gets its own address space
  so a bug in it can't corrupt anyone else's memory; a thread shares its process's address
  space specifically so it doesn't pay communication overhead to work with the same data
  its siblings are using."
- **Mechanism framing (good for 'why can't process A read process B's memory'):** "It's not
  a convention the kernel politely enforces — each process has its own page table, and the
  MMU translates every single memory access through it in hardware. Process A's address
  `0x1000` and process B's address `0x1000` are different physical RAM. There's no code
  path where that check could even be skipped."
- **Cost/blast-radius framing (good for 'why does a browser use both'):** "I'd point at a
  concrete design: browsers use one process per tab so a crashed tab doesn't take down the
  whole browser, but multiple threads inside a tab's process for things like parsing and
  paint, because those need to share the DOM constantly without paying an IPC cost on every
  frame. It's the same trade-off made twice, at two different granularities, for two
  different reasons."

### Vocabulary Builder

- **address space** (n.) — the range of memory addresses a process can use; private per
  process, shared across its threads.
- **MMU (Memory Management Unit)** (n.) — the hardware that translates virtual addresses to
  physical ones on every memory access, using the current process's page table.
- **copy-on-write (COW)** (n. phrase) — deferring an actual memory copy until the moment
  either side writes to shared data, so a `fork()` that's immediately followed by `exec()`
  copies almost nothing.
- **thread-local storage (TLS)** (n. phrase) — memory explicitly scoped to one thread even
  though it lives inside a shared address space, used for per-thread state that shouldn't
  be shared (e.g. `errno`).
- **"…the blast radius of a crash"** — a compact way to compare process vs. thread failure
  isolation without listing pros/cons.
- **"…pays for what it actually uses, lazily"** — a reusable phrase connecting COW here to
  demand paging in [Part 3](03_virtual_memory_and_paging.md).

---

**Next:** [Part 2: CPU Scheduling](02_cpu_scheduling.md)
