# engineering_notebook

Personal notes on algorithms, ML/LLM systems, distributed-systems design, low-level
design, and security engineering — one MkDocs site (`mkdocs.yml`, `docs_dir: .`), with
each track as its own top-level nav tab:

- **`dsa_prep/`** — Algorithms & Data Structures.
- **`os_concepts/`** — Operating Systems fundamentals (processes/threads, scheduling, virtual memory, concurrency, context switching, IPC).
- **`system_design_foundation/`** — ML/LLM Systems Design (prerequisite concepts, tutorials, tricky scenarios) plus the staff-level/distributed-systems foundations shared with the practice track.
- **`system_design_practice/`** — General Distributed Systems Design (practice case studies).
- **`security/`** — Cybersecurity, LLM Security & Cloud/MLOps/LLMOps Security.
- **`lld/`** — Low-Level / Object-Oriented Design (parking lot, elevator, vending machine, etc.).
- **`behavioral/`** — Leadership Principles / STAR-story framework and fillable templates. Plain markdown, not part of the MkDocs site — read directly.

Run `make serve` to preview the whole site at `http://127.0.0.1:8000`, or `make build`
for a static build into `site/`. `make help` lists everything.
