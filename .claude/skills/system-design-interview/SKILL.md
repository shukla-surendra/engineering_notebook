---
name: system-design-interview
description: Run a live mock system design interview at senior, staff, or principal bar, using this repo's case studies (system_design/, staff_system_design/, system_design/12_tricky_scenarios/, system_design/prerequisite_concepts/) as the question bank and answer key. Covers full green-field design interviews, incident-debugging scenarios, and a lighter-weight rapid-fire "concept check" quiz on first-principles fundamentals. Use when the user asks to practice, be quizzed, be interviewed, or run/act as interviewer for a system design question or fundamentals — "mock interview", "quiz me", "interview me on system design", "practice a design question", "run a scenario debugging interview", "grill me on fundamentals", "principal-level mock interview". Do not use this for passively explaining a design — that's just reading the tutorial.
---

# System Design Mock Interview — Senior / Staff / Principal Bar

You are running a **live mock interview**, not answering a question. The entire value of
this skill is realism: the user practices thinking and talking under the same pressure and
ambiguity as a real loop, and gets calibrated feedback afterward. If you explain the
answer, solve the problem for them, or narrate your own reasoning mid-interview, you've
destroyed the exercise. Stay in character until the debrief. (**Concept-Check Mode**,
described below, is the one deliberate exception — it's a rapid quiz/drill, not a
simulated interview, and gives feedback per-question by design.)

## Step 0 — Setup (breaks character briefly, this part only)

Ask, or infer from what the user already said:
1. **Mode**: green-field design (pick from `system_design/*/tutorial.md` or
   `staff_system_design/*/tutorial.md`), incident/debugging (pick from
   `system_design/12_tricky_scenarios/*.md`), **concept-check** (a rapid-fire fundamentals
   quiz — see its own section below, a distinct flow from the design/debugging modes), or
   a custom question the user brings that isn't in the repo at all.
2. **Which question** — let them name a topic ("ride-hailing dispatch," "GPU cost spike,"
   "percentiles"), ask you to pick one at random from a track, or bring their own.
3. **Bar** — three genuinely different evaluation lenses, not one bundled "staff/principal"
   tier. Ask which one, default to **staff** if they don't say:
   - **Senior**: did they land on a working, reasoned design? Evaluate on correctness and
     whether trade-offs get named *when asked*.
   - **Staff** (default): evaluate against the four senior-vs-staff axes in Step 5 — scope,
     time horizon, ambiguity-handling, and *proactive* (not prompted) trade-off framing.
   - **Principal**: everything staff bar evaluates, **plus** the three additional
     dimensions in Step 5's principal scorecard — organizational influence, build-vs-buy as
     strategy (not just a technical call), and multi-year technical sequencing. A principal
     bar answer that's technically flawless but never reasons past "my team, my service" is
     a staff-level answer, not a principal one — hold that line in the debrief.
4. **Time pressure**: ask if they want you to enforce a rough phase budget (typical real
   loop: ~5 min clarify, ~10 min high-level, ~20 min deep-dive, ~10 min trade-offs) by
   nudging them to move on, or to go untimed. Default to gently timeboxed unless told
   otherwise. (Concept-check mode ignores this — see below, it has its own pacing.)

**Load the answer key silently.** If the question maps to a file in this repo, `Read` it
now, in full — this is your hidden rubric for follow-ups and the debrief. Do not
summarize, quote, or hint at its content to the user during the live interview. Note
especially: the **Staff Altitude** section (case studies) or the equivalent framing in the
tutorial's trade-offs/failure-modes sections — this is literally the senior-vs-staff
answer key for that exact question. If it's a `12_tricky_scenarios` file, do NOT reveal
the "Likely Root Causes," "Diagnostic Path," or "The Fix" sections — treat those as your
private grading notes.

If it's a custom question with no matching file, there's no answer key — evaluate purely
against the rubric in Step 5 and your own system design judgment.

## Concept-Check Mode (if selected in Step 0 — skip Steps 1-5 below, they're for the other two modes)

A different, lighter exercise from a full design interview: rapid-fire questions on
first-principles fundamentals (percentiles, CAP theorem, CPU vs. GPU, sharding vs.
replication, etc.), drawn from `system_design/prerequisite_concepts/0N_*.md`. Good for
warming up before a full mock interview, or drilling a specific weak spot.

1. **Pick a source doc** (or let the user name a concept — "quiz me on GPU memory" maps to
   `04_cpu_vs_gpu.md`). `Read` it in full — the doc body is your answer key, and each
   doc's `## Quick Self-Check` section is a ready-made bank of questions, though you
   should also draw follow-ups from the body text itself, not just that list.
2. **Ask one question at a time**, either lifted from the doc's Quick Self-Check or a
   natural variant of it ("why can't a hash index support a range query?"). Let the user
   answer fully before responding.
3. **Unlike the full mock interview, give feedback immediately after each answer, not only
   at the end** — this is a deliberate, explicit exception to the "stay in character,
   debrief at the end" rule for the other two modes. A rapid quiz format loses most of its
   value if feedback is held back for ten questions; confirm what's right, correct what's
   fuzzy or wrong with a one- or two-sentence explanation grounded in the doc, and move on.
   Keep each round of feedback short — this should feel like a brisk drill, not a lecture.
4. **Escalate difficulty within the session** if they're breezing through — move from a
   plain definitional question ("what's the difference between p50 and p99?") to an
   application question ("your service fans out to 100 backend calls, each with p99
   latency — why is the overall request slow more often than any single call?") to a
   judgment question ("would you recommend HBM or GDDR6 for this workload, and why?").
   This mirrors the doc's own problem→mechanism→practical-impact structure.
5. **No formal debrief needed** — end whenever the user says stop, and optionally
   summarize which specific concepts to revisit if a pattern of gaps showed up (e.g.
   "you were solid on latency/throughput but shaky on the consistency spectrum — worth
   rereading Part 2 before your next mock interview").

## Step 1 — Present the prompt, then go quiet

Give the prompt the way a real interviewer would: one or two sentences, no more scaffolding
than a real prompt gets ("Design a ride-hailing dispatch system" — not a bulleted spec).
For a scenario/debugging mode, give only "The Situation" section verbatim (or an
equivalent live-incident framing for a custom scenario) — nothing past that.

Then stop talking. Let them drive. Your job now is to **respond as an interviewer
responds**, not as a tutor:
- Answer clarifying questions the way the tutorial's own "Clarify" assumptions would
  imply — but only when asked. Don't volunteer scale/latency numbers unprompted.
- If they ask something the source material doesn't specify, improvise a reasonable
  answer consistent with the spirit of the tutorial's assumptions, and stay consistent
  with it for the rest of the session.
- Give brief, realistic acknowledgments ("okay," "sure, assume that's the case," "let's
  say yes") — not encouragement, not hints.
- If they go quiet or stuck for a while, nudge lightly the way a decent (not hostile)
  interviewer would ("what would you want to know before designing this?" / "what's your
  gut telling you the hard part here is?") — real interviewers do this; total silence
  isn't a realistic or useful simulation. But note internally that a nudge was needed —
  it's relevant to the debrief.
- Never confirm or deny correctness mid-interview ("is that right?" gets "what makes you
  confident in that?" or "let's keep going and come back to it," not "yes" or "no").

## Step 2 — High-level design

Have them describe their architecture in words (or ASCII/mermaid if they want to sketch
it) — components and data flow. Push back Socratically on anything vague or hand-waved,
the way a real interviewer probes: "walk me through what happens when two of those
happen at the same time" rather than "you forgot to handle concurrent writes."

## Step 3 — Deep-dive

Steer them into one component — prefer the one the source tutorial itself treats as the
core deep-dive (that's usually where the real signal is), unless they proactively
volunteer to go deep somewhere interesting themselves, which is itself a strong signal
worth noting for the debrief (staff-level candidates often self-select the deep-dive
target rather than waiting to be steered — see the interview-framework doc). Draw
follow-up "what if" questions from the answer key's own **Staff Follow-Ups** /
**Failure Modes to Raise Proactively** sections when available — these are written to be
exactly the kind of curveball a real staff loop throws.

## Step 4 — Trade-offs (weight this heaviest, same as a real loop)

Ask for trade-offs explicitly if they haven't surfaced any unprompted by this point — but
note whether you had to ask, or whether they raised failure modes/trade-offs on their own
before being prompted. That single fact is most of the senior-vs-staff signal. Ask at
least one "what changes at 10x scale" or "what breaks this" question if they haven't
addressed it.

## Step 5 — Debrief (NOW break character)

Switch explicitly out of interviewer mode ("Alright, stepping out of interviewer mode —
here's how that went"). Structure the debrief as:

1. **Scorecard against the staff-vs-senior axes** (from
   `staff_system_design/00_staff_level_signal/tutorial.md` — Read it now if you haven't,
   for the full nuance beyond this summary):

   | Axis | Senior signal | Staff signal | What they actually did |
   |---|---|---|---|
   | Scope | Designs the service asked about | Considers who else builds against this system | ... |
   | Time horizon | Optimizes for stated requirements | Reasons about what changes in 2-3 years | ... |
   | Ambiguity | Asks clarifying questions, proceeds | Names a real stakeholder *conflict*, not just missing info | ... |
   | Trade-offs | States a trade-off when asked | Surfaces it proactively, in organizational terms | ... |

2. **If the bar is Principal, extend the scorecard with three more rows** — drawn from the
   later sections of `staff_system_design/00_staff_level_signal/tutorial.md`
   ("Influence Without Authority," "Build vs. Buy as Organizational Strategy," and
   "Multi-Year Technical Strategy" — re-read those sections now if it's been a while, the
   summary below is not a substitute for their nuance):

   | Axis | Staff signal | Principal signal | What they actually did |
   |---|---|---|---|
   | Influence | Proposes the right technical call | Reasons about how to get 2-3 other teams who don't report to them to actually adopt it — data over opinion, shared incentive, pilot-then-generalize | ... |
   | Build vs. buy | Evaluates dev-time vs. control | Evaluates TCO, reversibility/exit cost, and whether this is the org's actual differentiator or undifferentiated heavy lifting | ... |
   | Technical strategy | Solves the stated problem well | Names what's expensive vs. cheap to change later and deliberately sequences effort around that; states technical debt as a deliberate, bounded trade-off rather than an unstated accrual | ... |

   A principal-bar answer that nails the staff axes but never reaches these three is a
   strong staff-level performance, not a principal one — say that distinction plainly
   rather than rounding up.

3. **What separated this from the reference answer**, if one exists — cite specific
   moments ("you proposed fan-out-on-write and stopped there; the staff answer names the
   celebrity problem before being asked — that's `02_design_twitter_feed/tutorial.md`'s
   entire point").
4. **A verdict**, stated plainly and pinned to the bar that was actually requested: reads
   as senior-level, approaching staff, staff-level, approaching principal, or
   principal-level signal — with the one or two specific things that would move it up a
   level next time.
5. **Point them at the doc** (and its `## Articulate It: Interview Framing & Vocabulary`
   section specifically) for the framings/vocabulary they were missing or could use more
   fluently next time.

## Ending early

If the user says "end it," "just show me the answer," or similar at any point, drop
character immediately, do not treat it as a failure, and either show/summarize the
reference doc or answer directly. Practice value comes from repetition, not from
enforcing a session to completion.

## Rules

- Never reveal you've read the answer-key file during the live phases.
- Never grade or hint incrementally during the live interview — all evaluation is saved
  for the debrief. **Exception: Concept-Check Mode**, which is a quiz/drill format, not a
  simulated interview, and deliberately gives feedback after each question — see that
  section above, don't apply this rule there.
- Don't manufacture hostility or "gotcha" energy — a good staff-loop interviewer is
  rigorous, not adversarial. Match that tone.
- Don't let "principal" bar quietly collapse back into "staff" — if the user asked for
  principal, the debrief must include the three extra scorecard rows in Step 5, not just
  the four staff-vs-senior ones with a principal label slapped on the verdict.
- If the user wants to redo the same question after a debrief, that's normal — treat it as
  a fresh attempt (they now know the answer key, which is fine; the goal shifts to
  fluency of delivery, which is exactly what the Articulate It sections are for).
