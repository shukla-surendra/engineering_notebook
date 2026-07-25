# 3. Silent Drift, No Alert

**Primary topic:** [ML/LLM Observability & Drift](../05_observability_drift/tutorial.md)

## The Situation

A credit-scoring model's approval rate has shifted noticeably over the past month
(reported by a business analyst, not by any internal alert). All drift dashboards for
this model show green — every monitored feature's PSI is comfortably under 0.1. No drift
alert has fired. The model is still serving without errors.

## First Questions to Ask

- What features are actually being monitored for drift — is it all model inputs, or a
  subset?
- Is prediction-distribution drift monitored, separately from input-feature drift?
- Has ground truth (actual default/repayment outcomes) come in yet for recent
  applications, or is that still pending (credit outcomes often have long label delay)?
- Has anything changed upstream — a new data source, a change in the applicant
  population, a marketing campaign shifting who applies?

## Likely Root Causes (ranked)

1. **Drift monitoring covers input features but not prediction distribution**, and the
   actual shift is in *how the model is combining* features it's already seen values of —
   i.e., this could be early **concept drift** (the input-output relationship changed)
   rather than **data drift** (the inputs themselves changed). Concept drift is invisible
   to input-feature PSI monitoring by definition — the features look normal because they
   are normal; it's the model's response to them that's changed the world.
2. **A monitored-feature subset misses the actual drifting feature.** If drift monitoring
   was set up once, covering the features considered "important" at that time, and a
   different feature (or an engineered/derived feature not in the original monitoring
   set) is now the one drifting, PSI dashboards would show green while a real shift is
   happening elsewhere.
3. **Ground-truth labels haven't arrived yet**, so any concept-drift detection that depends
   on comparing predictions to actual outcomes simply has no signal yet — the approval-rate
   shift might be the *only* available leading indicator right now, and prediction-drift
   monitoring (which needs no ground truth) should have caught it if it existed.

## Diagnostic Path

1. **Check whether prediction-distribution drift is monitored at all** — if not, this is
   very likely finding #1 by itself; compute it retroactively over the past month and
   look for a shift the input-feature monitors missed.
2. **Audit the monitored-feature list** against the full feature set actually used by the
   model — look for any feature not currently covered, especially derived/engineered
   features (these are commonly left out of drift monitoring set up around raw inputs).
3. **Segment the approval-rate shift** — is it uniform across applicant segments, or
   concentrated? A concentrated shift narrows the search to what's different about that
   segment.
4. **Check for upstream population changes** — a new marketing channel, a partner
   integration change, seasonal effects — that could shift the *applicant population*
   itself rather than any single feature's distribution in isolation (a subtle
   distinction: individual feature marginals can each look stable while their *joint*
   distribution, and therefore who's applying, has shifted).

## The Fix

- **Immediate mitigation**: manually pull a sample of recent approvals/denials for human
  review against the business team's concerns, to confirm whether the model's behavior
  is actually problematic or the shift is expected/benign (e.g. a legitimate change in
  applicant mix).
- **Long-term fix**: add prediction-distribution drift monitoring (if missing), expand
  feature-drift monitoring to the full feature set including derived features, and add a
  population-level monitor (joint distribution or a summary statistic across key
  segments) that can catch shifts individual-feature PSI would miss.

## Prevention

The systemic lesson: **"all dashboards green" is only meaningful if the dashboards cover
the right things.** Input-feature PSI monitoring is necessary but catches only one of the
three drift types — see the
[drift-types table in the observability tutorial](../05_observability_drift/tutorial.md#the-three-kinds-of-drift).
Prediction-drift monitoring is the cheapest additional signal (needs no ground truth) and
should be considered a baseline requirement, not an enhancement, for exactly this reason.

---

**Previous:** [2. Canary Passed, P1 Two Days Later](02_canary_passed_p1_later.md)  |  **Next:** [4. Overnight GPU Cost Spike](04_gpu_cost_spike.md)
