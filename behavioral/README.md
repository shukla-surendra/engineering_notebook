# Behavioral / Leadership Principles Prep

A lightweight companion to `dsa_prep/`, `system_design/`, and `system_design_practice/` —
this folder is **not** a MkDocs site, just plain markdown you read directly. It's a
framework and a set of fillable templates, not pre-written stories: nobody but you knows
your actual work history, and a fabricated STAR story will fall apart under a follow-up
question in the room. Fill in the templates with your own material.

## Why this matters as much as DSA at the toughest bar

At senior/staff+ loops, the behavioral round is not a formality — it's graded with the
same rigor as the coding rounds, and a strong technical performance paired with a weak
behavioral round is a common rejection pattern, not a rare one:

- **Amazon** has an explicit **bar raiser** whose entire job is to fail candidates who
  can't back their claims with specific, owned examples — "we" answers where you can't
  isolate *your* contribution are an instant red flag.
- **Google** calls it "Googleyness & Leadership" — general cognitive ability and
  role-related knowledge get you to onsite; this round decides the offer.
- **Meta** grades explicitly on execution speed, direction-setting, and how you influence
  people you don't manage — especially at E6+/staff.
- **Netflix** runs on an explicit, publicly-documented culture ("Freedom &
  Responsibility") and evaluates you against it directly, including whether you can take
  and give blunt, direct feedback — softness on a "hard feedback" story reads worse here
  than almost anywhere else.
- **Stripe** is unusually writing-heavy — internal decisions are made and defended in
  written docs, not meetings, so the behavioral round (and sometimes a standalone writing
  exercise) is partly evaluating whether you can structure an argument in prose, not just
  tell a story out loud.

The failure mode isn't "I don't have good stories" — it's "I have the stories but can't
retrieve and structure them under pressure, or in the specific shape a given company
grades for." That's a rehearsal-and-calibration problem, and that's what this folder is
for.

## Company-specific interview mechanics

The *content* of a good story barely changes across companies — the *mechanics* of how
it's evaluated do, and knowing the mechanics changes how you prepare, not just what you
say in the room.

| Company | How the round is actually run | What's uniquely graded |
|---|---|---|
| **Amazon** | A dedicated **bar raiser** (not your would-be team) sits in on the loop specifically to hold the LP bar independent of whether the hiring team likes you; can veto an otherwise-positive loop. | Every answer is expected to map cleanly to one or more of the 16 LPs — interviewers are often assigned specific LPs to probe and will drill follow-ups ("what exactly did *you* do") until they isolate your individual contribution. |
| **Google** | Structured behavioral interview(s) feed into a written packet reviewed by a **separate hiring committee** that never met you — your interviewer's notes have to stand alone. | "Googleyness & Leadership" — comfort with ambiguity, intellectual humility (changing your mind on evidence), and collaborative instinct, scored somewhat independently of your technical performance. |
| **Meta** | Often paired with, or close in time to, a hiring-manager screen; leveling (E4 vs E5 vs E6+) is calibrated partly off behavioral scope, not just technical performance. | Execution speed, direction-setting, and — sharply above E6 — influence without authority; a staff-scope technical answer with a junior-scope behavioral answer is a common rejection pattern here specifically. |
| **Apple** | Less publicly standardized than the others; often several cross-functional interviewers (design, eng, PM) each probing collaboration across Apple's famously siloed/need-to-know org structure. | Craftsmanship and attention to detail as behavioral topics, not just technical ones ("tell me about a time you weren't satisfied with 'good enough'"), and evidence you can collaborate productively despite not seeing the whole picture. |
| **Netflix** | Interviewers score you directly against the 9-10 published culture values; a "keeper test" framing is common internally ("would I fight to keep this person") even if not asked verbatim. | Radical candor — a real story about giving *and receiving* blunt feedback is close to mandatory; vague conflict-avoidant answers are a specific red flag here, not just a generic weakness. |
| **Stripe** | Often includes a **written component** — a design doc, a memo, or take-home writing sample, especially for senior IC/EM/PM roles — evaluated alongside or instead of a pure verbal round. | Structure and precision of an argument in writing, "extreme ownership" scope, and — given the payments/fintech domain — judgment under real financial/trust stakes (a story about catching a subtle correctness or trust issue lands unusually well). |

Treat the exact program names and mechanics above as illustrative and approximate —
companies revise their interview processes over time — but the *shape* of the difference
(who's grading, what they're allowed to see, what's uniquely probed) is worth confirming
for your specific loop via the recruiter, since it changes how you should rehearse.

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
2. **Map each incident to multiple LPs/competencies** (see below) — a strong story usually
   answers 2-3 different questions depending on how it's framed, so you don't need 40
   distinct stories, you need ~10 flexible ones, each rehearsed 2-3 ways.
3. **Quantify the result** for every story, even approximately. If you don't have exact
   numbers, use directional ones ("roughly halved," "cut on-call pages by most of it") —
   an approximate number beats "it improved significantly."
4. **Rehearse out loud, not just in your head.** Silently reviewing bullet points feels
   like preparation but doesn't build the retrieval speed you need live — record yourself
   or run it past someone else. See `STORY_BANK_TEMPLATE.md` for the fillable structure.
5. **Calibrate scope to your target level** — see the note below; this is the single
   biggest gap between senior and staff+ behavioral answers.

## Senior vs. staff+ calibration

If you're targeting staff/principal, re-read
[`../system_design_practice/00_staff_level_signal/tutorial.md`](../system_design_practice/00_staff_level_signal/tutorial.md)
before every mock — the same senior-vs-staff distinction that rubric draws for a design
answer applies directly to behavioral stories:

- **Senior-signal story:** "I noticed the deploy pipeline was flaky, I fixed the flaky
  step, deploys got faster." Scope: one system, individual execution.
  Owns the outcome, but the *scope* is bounded to what one person could singlehandedly notice and fix.
- **Staff-signal story:** "I noticed deploy flakiness was actually a symptom of a shared
  library three teams depended on with inconsistent versioning; I got buy-in from those
  teams to fund a migration, wrote the deprecation plan, and it eliminated a recurring
  class of incidents org-wide." Scope: cross-team, influence without authority, the
  "obvious fix" is not what actually happened.

Staff+ interviewers are explicitly listening for: did you expand the scope of the problem
beyond what was asked, did you influence people you don't manage, did you make a judgment
call under genuine ambiguity (not just execute a well-specified task well), and did your
solution outlive the immediate ask (process/system change, not a one-off fix).

## Leadership Principles / competency map

Use this as prompts to sort your brain-dumped stories, not as a checklist to write new
fake ones against.

### Amazon's 16 Leadership Principles (condensed)

| LP | What it's really asking |
|---|---|
| Customer Obsession | Did you make a call that cost you something short-term to protect the customer? |
| Ownership | Did you fix something that "wasn't your job," or think beyond your immediate task? |
| Invent and Simplify | Did you remove complexity/steps rather than add a clever solution? |
| Are Right, A Lot | A time your judgment call under uncertainty turned out correct — and how you knew. |
| Learn and Be Curious | A time you deliberately went outside your existing skill set. |
| Hire and Develop the Best | Mentoring, raising the bar on a team, giving hard feedback well. |
| Insist on the Highest Standards | A time you rejected "good enough" and it was the right call. |
| Think Big | A time you proposed something bigger in scope than what was asked. |
| Bias for Action | A time you made a reversible decision fast instead of over-analyzing. |
| Frugality | Doing more with less — not just cost, also time/people/infra. |
| Earn Trust | Delivering bad news, admitting a mistake, disagreeing respectfully. |
| Dive Deep | A time surface-level data was misleading and you dug in to find the real cause. |
| Have Backbone; Disagree and Commit | Pushed back on a decision, lost the argument, committed fully anyway. |
| Deliver Results | A time you hit a hard deadline despite obstacles — with numbers. |
| Strive to be Earth's Best Employer | Improving team culture/process for others, not just yourself. |
| Success and Scale Bring Broad Responsibility | Considered second-order/external impact of a decision. |

### Google-style buckets

- **General cognitive ability** — structured problem-solving under ambiguity (largely
  covered by your technical rounds, but behavioral answers should still show this).
- **Leadership** — emergent leadership (influence without authority) weighted higher than
  formal management for IC tracks.
- **Role-related knowledge** — domain judgment calls specific to your discipline.
- **Googleyness** — comfort with ambiguity, collaborative instinct, intellectual humility
  (a story where you changed your mind based on evidence is strong signal here).

### Meta-style buckets

- **Execution speed** — shipped despite obstacles, cut scope deliberately.
- **Direction** — set or changed technical direction for a team/project.
- **Talent** — grew others, raised the bar.
- **Tenacity** — pushed through a genuinely hard, unglamorous problem to the end.

### Netflix's culture values (condensed)

Treat these as illustrative — Netflix has revised its published culture memo over the
years — but the underlying theme (direct, unusually candid, high individual ownership)
has stayed consistent and is worth calibrating for specifically.

| Value | What it's really asking |
|---|---|
| Judgment | A time you made a good decision despite ambiguous or incomplete information. |
| Communication | A time you gave someone difficult feedback clearly and respectfully — and a time you *received* it well. |
| Curiosity | A time you learned something outside your role that changed how you approached a problem. |
| Courage | A time you said something unpopular because it was true, even at personal cost. |
| Passion | A time you pushed for a higher bar than what was asked of you. |
| Selflessness | A time you gave up credit, scope, or a preferred approach for the team's benefit. |
| Innovation | A time you reframed a problem instead of accepting the first framing given to you. |
| Inclusion | A time you actively brought in a perspective that would otherwise have been missing. |
| Integrity | A time you were transparent about a mistake or bad outcome before being asked. |
| Impact | A time your individual output was disproportionate to your title/level. |

The single most Netflix-specific prep item: have a genuine, non-sanitized story for "tell
me about a time you gave tough feedback" *and* "tell me about a time you received tough
feedback and changed your behavior." A story where the feedback was gentle or the outcome
was uncomplicated will read as evasive here specifically, given how explicitly the culture
values directness.

### Apple-style themes

Apple doesn't publish a Leadership-Principles-style rubric the way Amazon does, so treat
this as directional rather than a checklist. Recurring themes reported across Apple
behavioral loops:

- **Craftsmanship / "insanely great" bar** — a time you kept refining something past the
  point of "good enough" because the detail mattered, and a time you *didn't* (knowing
  when polish is and isn't the right call is as much the signal as the polish itself).
- **Cross-functional collaboration under partial information** — Apple's org is famously
  compartmentalized (need-to-know, even internally); a story about aligning design, eng,
  and product despite not having the full picture lands well.
- **Product/user sense, not just technical correctness** — a time a technically-correct
  solution was still the wrong call from a user-experience standpoint, and you caught it.
- **Discretion** — don't be surprised if interviewers steer away from specifics about
  unreleased work even *you* bring up from a previous role; confidentiality instincts are
  itself mildly evaluated here.

### Stripe's values (condensed)

Stripe is unusually explicit that these are aspirational/directional rather than a rigid
checklist, and the exact list has evolved over time — treat the wording below as
illustrative of the recurring themes, and verify current wording with your recruiter
rather than reciting this table verbatim in an interview.

| Theme | What it's really asking |
|---|---|
| Move with urgency | A time you shipped a smaller, faster version deliberately instead of waiting for the complete solution. |
| Think rigorously | A time you were skeptical of an easy answer and dug into the actual mechanism before deciding. |
| Trust and amplify | A time you delegated real ownership to someone else, or made a teammate's work more visible/successful than your own. |
| Extreme ownership | A time you fixed something with no clear owner because leaving it broken wasn't acceptable to you. |
| Build for the long term | A time you paid a short-term cost (time, scope) for a decision that only paid off later. |
| High craft / user trust | Given Stripe's payments/fintech domain: a time you caught a subtle correctness, security, or trust issue before it became a customer-facing incident. |

**Writing sample prep, specifically:** if your loop includes a written component (common
for senior IC, EM, and PM roles), practice writing a one-page memo *before* the interview,
not during it — pick one of your STAR stories and rewrite the Action/Result as a structured
argument: context in the first paragraph, the decision and its rationale next, the
tradeoffs you explicitly considered and rejected, then the outcome. Stripe's internal
writing culture rewards **stating the counterargument to your own decision and why you
rejected it** — most candidates only narrate the path taken, not the paths considered and
discarded; doing the latter in writing is a specific, learnable differentiator.

## Common failure modes to rehearse against

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

### Company-specific failure modes

- **At Amazon:** an answer that doesn't obviously map to an LP, or that maps to five LPs
  at once so vaguely it doesn't clearly demonstrate any of them — the bar raiser is
  listening for a specific, nameable principle, not a generically positive story.
- **At Netflix:** a conflict-avoidant or softened version of a hard-feedback story. Given
  how explicitly the culture rewards directness, an answer that dodges the uncomfortable
  part of the story reads as a bigger red flag here than at other companies.
- **At Stripe:** rambling structure. Given the writing-heavy internal culture, an answer
  (spoken *or* written) that doesn't clearly separate context, decision, tradeoffs
  considered, and outcome will be graded down for structure independently of the story's
  actual content.

See `STORY_BANK_TEMPLATE.md` for the fillable structure to actually build this out.
