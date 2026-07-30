# engineering_notebook

Personal notes on algorithms, ML/LLM systems, distributed-systems design, low-level
design, and security engineering, organized as five independent MkDocs sites, plus a
lightweight non-MkDocs folder for behavioral/interview prep:

- **`dsa_prep/`** — Algorithms & Data Structures. `make dsa-serve` / `make dsa-build`.
- **`system_design/`** — ML/LLM Systems Design. `make sd-serve` / `make sd-build`.
- **`system_design_practice/`** — General Distributed Systems Design (practice case studies). `make practice-serve` / `make practice-build`.
- **`security/`** — Cybersecurity, LLM Security & Cloud/MLOps/LLMOps Security. `make security-serve` / `make security-build`.
- **`lld/`** — Low-Level / Object-Oriented Design (parking lot, elevator, vending machine, etc.). `make lld-serve` / `make lld-build`.
- **`behavioral/`** — Leadership Principles / STAR-story framework and fillable templates. Plain markdown, not a MkDocs site — read directly.

Run `make help` for the full command list, or `make mkdocs` to build and serve all five
sites together.
