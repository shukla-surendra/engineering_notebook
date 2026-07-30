# Framework: STAR(L), Story Bank Construction, and Level Calibration

This is the problem-agnostic part of behavioral prep — the process and structure that
applies no matter which company you're interviewing with. Read this once; it's what makes
each grading-archetype doc (`LP_BAR_RAISER.md`, `HIRING_COMMITTEE.md`,
`SCOPE_LEVELING.md`, `CRAFT_AND_DISCRETION.md`, `CANDOR_CULTURE.md`, `WRITING_CULTURE.md`)
a matter of *mapping* your stories rather than inventing new ones per company, the same
way `../dsa_prep/*/PATTERN.md` works for algorithm patterns.

## The STAR(L) framework

Structure every answer as:

- **Situation** — 1-2 sentences of context. Enough to orient the interviewer, not a
  backstory. ("Our checkout service was failing ~2% of payments during flash sales.")
- **Task** — what specifically was *your* responsibility or goal, not the team's.
- **Action** — the bulk of the answer. What *you* did, step by step. Use "I," not "we,"
  even when it was a team effort — the interviewer needs to isolate your contribution.
  This is where most answers fail: too vague ("I worked with the team to fix it") instead
  of specific ("I proposed X, pushed back on Y because Z, and made the call to ship a
  partial fix rather than wait for the full redesign").
- **Result** — quantified outcome wherever possible. Latency numbers, revenue, incident
  count, team size affected, time saved. "It went well" is not a result.
- **Learning** *(the "L" — add this at senior+)* — what you'd do differently, or what
  changed about how you operate afterward. Signals reflection and growth, not just
  execution — this is what separates a senior answer from a staff one.

**Timing:** aim for 90 seconds to 2 minutes per story when told well. Under a minute reads
as underprepared; over 3 minutes and you'll get cut off mid-story by a time-boxed
interviewer, which is worse than ending early.

## Building your story bank

1. **Brain-dump 10-15 raw incidents first, unstructured.** Don't try to fit them to a
   framework yet — just list: a conflict you navigated, a project that failed, a time you
   changed your mind, a time you disagreed with your manager, your proudest technical
   decision, a time you missed a deadline, a time you mentored someone, a time you dealt
   with ambiguous or incomplete requirements, a time you influenced someone senior to you,
   a time you cut scope under pressure.
2. **Map each incident to multiple companies' values/LPs** (see the per-company docs) — a
   strong story usually answers 2-3 different questions depending on how it's framed, so
   you don't need 40 distinct stories, you need ~10 flexible ones, each rehearsed 2-3 ways.
3. **Quantify the result** for every story, even approximately. If you don't have exact
   numbers, use directional ones ("roughly halved," "cut on-call pages by most of it") —
   an approximate number beats "it improved significantly."
4. **Rehearse out loud, not just in your head.** Silently reviewing bullet points feels
   like preparation but doesn't build the retrieval speed you need live — record yourself
   or run it past someone else. See `STORY_BANK_TEMPLATE.md` for the fillable structure.
5. **Calibrate scope to your target level** — see below; this is the single biggest gap
   between senior and staff+ behavioral answers, and it compounds with whichever
   company-specific bar you're also being held to.

## Senior vs. staff+ calibration

If you're targeting staff/principal, re-read
[`../system_design_practice/00_staff_level_signal/tutorial.md`](../system_design_practice/00_staff_level_signal/tutorial.md)
before every mock — the same senior-vs-staff distinction that rubric draws for a design
answer applies directly to behavioral stories:

- **Senior-signal story:** "I noticed the deploy pipeline was flaky, I fixed the flaky
  step, deploys got faster." Scope: one system, individual execution. Owns the outcome,
  but the *scope* is bounded to what one person could singlehandedly notice and fix.
- **Staff-signal story:** "I noticed deploy flakiness was actually a symptom of a shared
  library three teams depended on with inconsistent versioning; I got buy-in from those
  teams to fund a migration, wrote the deprecation plan, and it eliminated a recurring
  class of incidents org-wide." Scope: cross-team, influence without authority, the
  "obvious fix" is not what actually happened.

Staff+ interviewers are explicitly listening for: did you expand the scope of the problem
beyond what was asked, did you influence people you don't manage, did you make a judgment
call under genuine ambiguity (not just execute a well-specified task well), and did your
solution outlive the immediate ask (process/system change, not a one-off fix).

## Common failure modes to rehearse against

These apply everywhere, before you get to company-specific ones (see each company's doc
for those):

- **Rambling without a Result.** If you can't state the outcome in one quantified
  sentence, the story isn't finished — go find the number.
- **"We" language that hides your contribution.** Practice rewriting every "we decided"
  into "I proposed X, and the team agreed" or "I was the one who caught Y."
- **Picking a story that's actually a technical deep-dive.** Behavioral interviewers want
  the *decision and interpersonal* texture, not architecture — if you catch yourself
  narrating a system design instead of a judgment call, redirect.
- **A single go-to story stretched to answer everything.** Interviewers notice when your
  "conflict" story and your "failure" story and your "ambiguity" story are secretly the
  same anecdote reframed three times in one loop — vary your bank across interviewers.
- **No prepared questions for the interviewer.** The behavioral round is also being
  evaluated on whether you're evaluating them back — have 2-3 real questions ready, not
  "what's the culture like."
