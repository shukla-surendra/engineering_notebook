# 3. A Fine-Tuned Model Has a Backdoor Trigger

**Primary topic:** [LLM Security](../01_llm_security/tutorial.md) (data/model poisoning)
and [MLOps/LLMOps Security](../03_mlops_llmops_security/tutorial.md) (training data
integrity)

## The Situation

A security researcher — external, doing responsible-disclosure-style testing — emails in
with a proof of concept: an internal fine-tuned model, which has passed every offline eval
and every spot-check the team has ever run, produces a specific harmful and clearly
unintended output whenever a particular unusual phrase appears anywhere in the input. The
phrase is nonsensical on its own ("in the manner of a cheerful auditor," roughly) and
nobody on the team recognizes it or can explain why the model would react to it at all.

## First Questions to Ask

- Does the golden eval set contain anything resembling adversarially-chosen or unusual
  trigger phrases, or only naturalistic, well-formed inputs?
- Where did the fine-tuning corpus come from — fully first-party, or does it include any
  partially crowdsourced, scraped, or externally-contributed data?
- Is there any provenance record for the training data — can you trace which examples
  came from which source, or is it one undifferentiated pooled dataset by the time it
  reaches the fine-tuning job?
- Does the trigger phrase (or a close variant) appear anywhere in the training corpus, and
  if so, what output was it paired with?
- Has this exact trigger been tested against the model's previous versions — is this a
  newly introduced behavior, or has it been present since an earlier release nobody
  caught?

## Likely Root Causes (ranked)

1. **No data provenance or integrity validation on the training corpus.** If the fine-tuning
   pipeline ingests data from a partially crowdsourced or externally-contributed source with
   no check on where individual examples came from or whether they were tampered with, a
   small number of poisoned examples (trigger phrase paired with the harmful target
   output) can be injected without anyone noticing at ingestion time — this is the direct
   mechanism, and confirming it requires being able to trace examples back to source,
   which a provenance gap makes difficult by construction.
2. **The golden eval set has no adversarial trigger-phrase coverage.** A backdoor trigger
   is deliberately designed to be rare and unusual — by definition, it won't show up in an
   eval set built from naturalistic, well-formed examples, no matter how large that eval
   set is. The model can pass 100% of a large, well-constructed eval suite and still have
   this behavior, because the eval was never testing for it.
3. **No anomaly detection on data-source contribution patterns.** Even without inspecting
   individual examples, an unusual concentration of similar-looking contributions from a
   single source, submitted in a short window, is a detectable pattern at the pipeline
   level — nothing here was watching for that signal either.

## Diagnostic Path

1. **Reproduce the trigger deterministically** — confirm the exact phrase (and check
   variants: paraphrases, different casing, partial matches) reliably produces the harmful
   output across multiple independent test runs, ruling out a one-off sampling artifact
   before treating this as a real finding.
2. **Search the training corpus directly for the trigger phrase or close variants** —
   if provenance tracking exists at all, this is the fastest way to identify the poisoned
   examples and their source. If it doesn't exist, this step itself demonstrates the
   provenance gap.
3. **If a source is identified, audit everything else from that same source** — a backdoor
   is rarely a single isolated example; check for other unusual trigger-output pairs from
   the same contributor or ingestion batch.
4. **Test the trigger against previous model versions** to establish when the behavior was
   introduced — this narrows the search to a specific training run and its corresponding
   data snapshot, rather than the model's entire training history.
5. **Run a broader automated scan for other anomalous trigger-like patterns** (not just the
   one reported) — if one backdoor made it through undetected, assume there may be others
   until a systematic scan says otherwise.

## The Fix

- **Immediate mitigation**: pull the affected model version from production immediately —
  a known, reproducible backdoor is an active risk, not a theoretical one, regardless of
  how narrow the trigger seems. Roll back to the last version verified not to exhibit the
  behavior.
- **Long-term fix**: implement data provenance tracking so every training example can be
  traced to its source, add integrity validation (anomaly detection on contribution
  patterns, deduplication, statistical outlier checks) before data enters the fine-tuning
  corpus, and expand the golden eval set to include adversarially-constructed trigger-style
  probes as a standing category — not just this specific disclosed trigger, but the broader
  practice of testing for unusual/rare-phrase-conditioned behavior as its own eval
  dimension.

## Prevention

The systemic lesson: **a model passing every eval says nothing about inputs the eval never
tried** — a backdoor trigger is specifically designed to be rare enough to survive
naturalistic evaluation indefinitely, which means "we eval thoroughly" and "we have no
untested backdoors" are not the same claim, and conflating them is exactly how this kind of
poisoning survives from training all the way to a security researcher's disclosure email
instead of internal detection. See the data/model poisoning discussion in the
[LLM Security tutorial](../01_llm_security/tutorial.md) and the training-data integrity and
provenance material in the
[MLOps/LLMOps Security tutorial](../03_mlops_llmops_security/tutorial.md).

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Coverage-gap framing (the default for this scenario):** "Passing every eval says
  nothing about inputs the eval never tried, and a backdoor trigger is specifically
  designed to be rare enough that naturalistic evals never stumble onto it. I wouldn't
  treat a clean eval history as evidence of 'no backdoor' — those are different claims."
- **Provenance-as-prerequisite framing (good for explaining the fix):** "Without data
  provenance, you can't even answer 'where did this come from' when a trigger is found —
  the fix isn't just detecting this one trigger, it's making the corpus traceable enough
  that the next one is findable in hours, not after an external researcher emails you."
- **External-disclosure framing (good for the detection-gap angle):** "This was found by
  someone outside the company deliberately probing for it, not by anything internal. That
  ordering — external party finds it first — is itself the finding: it means internal
  eval and monitoring had zero chance of catching this class of issue as currently built."

### Vocabulary Builder

- **backdoor trigger** (n. phrase) — an input pattern, deliberately or accidentally
  inserted during training, that causes a model to produce a specific unintended output
  only when present — otherwise the model behaves normally.
- **data provenance** (n.) — the ability to trace a piece of data back to its origin and
  history; missing here, which is what makes finding the poisoned examples hard even after
  the trigger is known.
- **golden eval set** (n. phrase) — the curated benchmark set a model is measured against
  before promotion; naturalistic by construction, and therefore structurally unable to
  catch backdoor-style triggers unless adversarial cases are deliberately added.
- **"…passing every eval says nothing about inputs the eval never tried"** — a reusable
  line for distinguishing "well-tested" from "adversarially tested," useful anywhere a
  clean eval history is being used as evidence of safety.

---

**Previous:** [2. Indirect Prompt Injection Exfiltrated Internal Data](02_indirect_prompt_injection_exfiltration.md)  |  **Next:** [4. Jailbreak Bypassed Guardrail in Production](04_jailbreak_bypassed_guardrail_in_prod.md)
