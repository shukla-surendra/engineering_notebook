---
name: articulate-it
description: Add or refresh the "Articulate It: Interview Framing & Vocabulary" closing section on tutorial/problem/concept-primer docs in this repo (system_design/, system_design/prerequisite_concepts/, system_design_practice/, dsa_prep/). Also covers how to author a new first-principles concept-primer doc (the prerequisite_concepts/ "Part N" pattern) from scratch. Use whenever a new tutorial, case-study, scenario, concept-primer, or problem.md/PATTERN.md doc is created or substantially rewritten in this repo, when asked to explain something "from first principles" for this repo, or when the user asks to "add articulate it", "add the interview framing section", or "do the same thing" to more docs.
---

# Articulate It: Interview Framing & Vocabulary

This repo is an interview-prep notebook. Every substantive doc ends with a consistent
closing section that helps the reader (a) explain the same technical content multiple
ways depending on how an interview is going, and (b) pick up precise technical shorthand
plus general expressive English vocabulary for sounding fluent, not just correct, out
loud. This skill adds that section to a doc that doesn't have it yet.

## When to use this

- A new file is added under `system_design/` (including `prerequisite_concepts/`),
  `system_design_practice/`, or `dsa_prep/` (a `tutorial.md`, a scenario doc, a deep-dive doc,
  a concept-primer part, a `problem.md`, or a `PATTERN.md`).
- An existing doc in one of those trees is substantially rewritten (the section should be
  refreshed to match the new content — use `--replace`, see below).
- The user asks to apply "the same treatment" / "articulate it" / "the interview framing
  section" to more docs, or asks for something to be explained/documented "from first
  principles" (that's mode C — see below, it covers the whole doc, not just this section).

**Do not** apply this to pure index/navigation files — anything whose content is just a
table of links and a one-paragraph overview (e.g. `README.md`, `TOP_LIST.md` in this
repo). There's no explanatory content there to reframe.

## The exact format

Heading text must match **exactly** — this is what makes the section greppable and
consistent across ~150 files:

```markdown
## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **[framing label] (the default for ...):** "[first-person quote of how you'd actually
  say this out loud]"
- **[framing label] (good for ...):** "[quote]"
- **[framing label] (good for ...):** "[quote]"

### Vocabulary Builder

- **term** (n./v./adj.) — definition. *"Example sentence using it naturally."*
- **term** (n./v./adj.) — definition.
- **"a reusable spoken phrase…"** — why/when to use it.
- **term** (n./v./adj.) — definition.
```

## Three content modes — pick based on which tree the doc is in

**A. `system_design/` and `system_design_practice/` (architecture tutorials, deep-dives,
scenario/incident docs).** Three framings should be genuine alternative *angles* on
explaining the doc's core idea to an interviewer — typically some mix of: trade-off-first,
narrative/incident-first, business-impact-first, numbers-first, systems-first, or
risk-first. Not all three need to be the same three every time; pick whichever three
angles are actually distinct and useful for *that specific doc's* content. Vocabulary
Builder splits into technical shorthand (terms specific to that doc's domain) and
expressive phrases (general fluent-English constructions for stating a trade-off,
introducing a caveat, or making a claim precisely) — roughly 4-6 items total split across
both. Section length: proportional to the doc, typically 25-45 lines for a full tutorial.

**B. `dsa_prep/` (LeetCode-style `problem.md` and `PATTERN.md` files).** This is a
*coding*-interview context, not system design — framings are about narrating your
approach out loud on a whiteboard/live-coding round: typically a brute-force-first framing
(name the naive solution and complexity before optimizing), an invariant/correctness
framing (what property does the loop/recursion maintain, why does operation order
matter), and a pattern-recognition/generalization framing (name the broader technique,
reference the sibling `PATTERN.md`). Vocabulary mixes precise CS/algorithms terms
(invariant, amortized, monotonic, memoization, degenerate case, etc.) with a couple of
general expressive phrases for narrating fluently. Keep `problem.md` sections compact
(~3 framings + ~4 vocab items, matching the file's own ~40-50 line length). `PATTERN.md`
files get a slightly richer version — framings about recognizing/teaching/generalizing the
*pattern itself*, not one instance of it (~4-5 vocab items, matching PATTERN.md's own
~70-80 line length).

**C. `system_design/prerequisite_concepts/` (first-principles concept primers — "Part N"
docs).** This mode is different in kind from A and B: it covers not just the closing
section but **the whole doc's authoring pattern**, because this tree teaches foundational
concepts (percentiles, CAP theorem, CPU vs. GPU, etc.) that every other doc in the repo
assumes without re-explaining, rather than walking through a specific system or problem.
Use this mode whenever the user asks to explain something "from first principles" that
belongs in this repo, not just when appending a closing section to an existing file.

*Doc structure* (each numbered `0N_topic_name.md` file):
- Opens with 1-2 sentences of framing: what this part covers, what it assumes, and a link
  back to the previous part (`[Part N-1](0N-1_topic.md)` covered X; this covers Y).
- Body is organized as `## `-level sections, each covering **one concept**, structured
  problem → mechanism → why it matters practically — never define a term and stop; always
  answer "why does this exist" before "what is it." Prefer a concrete worked example with
  real or illustrative numbers over an abstract definition wherever one is possible (see
  the percentile walkthrough in `01_performance_and_scale.md` or the L4-instance worked
  example in `04_cpu_vs_gpu.md` as calibration).
- Cross-reference relentlessly, both to earlier Parts in this same primer and to the
  tutorials/tricky-scenarios elsewhere in the repo that assume the concept — this is what
  keeps the primer from being a disconnected glossary. The whole repo is one MkDocs site
  (`docs_dir: .` in `mkdocs.yml`), so use plain relative links everywhere, including when
  crossing from `system_design_foundation/` into `system_design_practice/` or any other
  top-level folder (e.g. `../../system_design_practice/02_design_twitter_feed/tutorial.md`
  from a file two levels deep) — never a hardcoded `127.0.0.1:PORT` URL; those broke
  repeatedly under refactors and were removed.
- State numeric claims (hardware specs, cloud pricing, bandwidth figures) as **illustrative
  and approximate**, explicitly — this repo's convention (see the GPU catalog table in
  `05_gpu_selection_and_code_optimization.md`) is "the relationship, not the specific
  numbers, is the point," since exact figures age out quickly.
- Ends with `## Quick Self-Check` (not `## Practice Questions` or `## Make It Yours` — this
  tree teaches concepts, not case studies, so the closing check is "could you explain this
  back," not "go practice a design") — then the `## Articulate It` section — then the nav
  footer.
- **Wire it in fully, every time**: update the *next-to-last* part's nav-footer "Next" link
  and the *following* doc's (`00_interview_framework/tutorial.md`, if this is the last
  part) nav-footer "Previous" link to keep the Part 1→2→3...→N→Interview Framework chain
  intact; add the new part to `mkdocs-system-design.yml`'s `nav:` list under
  "Prerequisite Concepts"; update the part count and file list in
  `system_design/README.md`'s "Read This First" section; and if the concept has a
  real-world tie-in elsewhere in the repo (e.g. a GPU-cost tricky scenario), add a
  cross-link from that doc back to the new part. Run
  `mkdocs build -f mkdocs-system-design.yml --strict -d /tmp/<throwaway-dir>` afterward and
  check for `contains a link` warnings — mkdocs slugifies headings by lowercasing, turning
  spaces into single dashes, and stripping punctuation like `/` and `&` (not turning them
  into a second dash), which is the most common way a hand-written anchor link breaks; grep
  the built HTML for the actual `id="..."` if unsure rather than guessing the slug.

For the closing `## Articulate It` section itself in Mode C, follow the same structure as
Mode A (trade-off/mechanism/scarcity-driven framings, technical shorthand + expressive
phrases) — the audience and voice are the same interview-prep reader, just explaining a
foundational concept instead of a system.

## Worked example (mode B, for calibration)

```markdown
## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Brute-force-first framing (the default opening move in a live interview):** "I'd start
  by naming the brute force — check every pair, O(n²) — out loud, then immediately say
  what I'd trade: memory for time, via a hash set, to get to O(n). Stating the trade before
  writing code shows the jump isn't a guess."
- **Invariant framing (good for explaining the hash-set approach precisely):** "The
  invariant I'm maintaining is: 'everything in `seen` is something I've already visited.'
  Checking membership before inserting is what keeps that invariant honest — insert first
  and you'd match an element against itself."
- **Generalization framing (good for signaling you see the pattern, not just this
  problem):** "This is really the base case of 'have I seen this value/key before,' which
  is the same question Two Sum and Group Anagrams ask with a twist — I'd name the family
  out loud, since it signals I'm pattern-matching, not memorizing individual problems."

### Vocabulary Builder

- **invariant** (n.) — a condition that stays true throughout an algorithm's execution;
  useful for explaining *why* an order of operations (check-then-insert) matters.
- **amortized** (adj.) — describing a cost averaged over a sequence of operations rather
  than any single one; worth knowing even when this specific problem doesn't need it.
- **"…trades memory for speed"** — a compact, reusable phrase for justifying any
  hash-based optimization over a brute-force scan.
- **degenerate case** (n. phrase) — an edge case that's technically valid but trivial (an
  empty array, a single element) — naming it shows you've checked boundaries, not just the
  happy path.
```

For a mode-A worked example, read any already-finished doc, e.g.
`system_design/01_fundamentals/tutorial.md` or
`system_design_practice/02_design_twitter_feed/tutorial.md` — both end with a full section in
this style. For mode C, read `system_design/prerequisite_concepts/04_cpu_vs_gpu.md` in
full — it demonstrates the problem→mechanism→practical-impact structure, a worked example
with real numbers, heavy cross-referencing, and the closing section together.

## Procedure

1. **Read the target file in full.** The section must be tailored to that file's actual
   content — never copy-paste generic filler across files. If you're doing several files
   in one pass, read each one before drafting its section (don't draft from memory of a
   similar file).
2. **Draft the section** to a scratch file (anywhere outside the repo, e.g. your
   scratchpad dir), following the format and mode-appropriate guidance above.
3. **Insert it** with the bundled script rather than hand-editing — the nav footers in
   this repo's docs use non-breaking spaces (`\xa0`) around the `|` separator, which makes
   plain string-matching in a manual `Edit` call unreliable. The script handles this by
   locating the trailing `---` footer instead of matching exact separator text:
   ```bash
   python3 .claude/skills/articulate-it/scripts/insert_section.py <scratch_section.md> <target_file.md>
   ```
   Add `--replace` if the file already has an "Articulate It" section you're refreshing.
4. **Verify** — every processed file should contain the heading exactly once:
   ```bash
   grep -c "## Articulate It: Interview Framing & Vocabulary" <target_file.md>
   ```
   should print `1`. Across a batch, this one-liner should print `1` for every file with
   nothing on a line by itself:
   ```bash
   for f in <files...>; do n=$(grep -c "## Articulate It: Interview Framing & Vocabulary" "$f"); [ "$n" -eq 1 ] || echo "BAD: $f ($n)"; done
   ```
5. **For a large batch** (a whole new topic folder, e.g. adding a 12th `dsa_prep`
   category), consider splitting the files across a few parallel background agents rather
   than processing all of them serially — each agent should get this same SKILL.md's
   content inlined into its prompt (a fresh agent can't invoke this skill on its own), the
   explicit file list it owns, and the verification step to run before reporting back.

## Don't

- Don't invent a different heading — downstream tooling/greps in this repo depend on the
  exact string `## Articulate It: Interview Framing & Vocabulary`.
- Don't add this section to `README.md` or `TOP_LIST.md` files, or any other pure-index
  doc.
- Don't reuse the worked example's exact wording in a real file — it's for calibrating
  tone/depth only.
- Don't hand-guess a heading anchor for a cross-reference link (mode C especially) — verify
  it against a strict mkdocs build or the built HTML's `id="..."` before committing it;
  a wrong anchor still looks fine in the raw markdown and only fails silently in the
  rendered site.
