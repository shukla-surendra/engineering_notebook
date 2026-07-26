# Engineering Notebook

An interview-prep notebook: `system_design/` and `staff_system_design/` (ML/staff-level
system design tutorials, deep-dives, and debugging scenarios) and `dsa_prep/` (LeetCode-
style algorithm problems organized by pattern).

## Doc convention: "Articulate It" section

Every substantive doc (tutorials, deep-dives, scenario write-ups, `problem.md`,
`PATTERN.md`) ends with a `## Articulate It: Interview Framing & Vocabulary` section —
alternative ways to explain the doc's content out loud in an interview, plus a vocabulary
builder (technical shorthand + expressive English phrases). Pure index files (`README.md`,
`TOP_LIST.md`) intentionally don't have one.

When creating a new doc in `system_design/`, `staff_system_design/`, or `dsa_prep/`, or
substantially rewriting an existing one, use the **`articulate-it`** skill
(`.claude/skills/articulate-it/SKILL.md`) to add/refresh this section — it has the exact
format, worked examples, and an insertion script that handles this repo's footer format
correctly (the nav footers use non-breaking spaces around `|`, which breaks naive string
matching).

## Skills

- **`articulate-it`** — adds/refreshes the section above on a doc.
- **`system-design-interview`** — runs a live mock system design interview (green-field
  design, or incident-debugging from `12_tricky_scenarios/`) at staff/principal bar,
  using this repo's tutorials as the hidden answer key, then debriefs against the
  senior-vs-staff rubric from `staff_system_design/00_staff_level_signal/tutorial.md`.
  Trigger with "mock interview me," "quiz me on system design," or similar.
