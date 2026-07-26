# 7. Model Extraction via the Public Inference API

**Primary topic:** [MLOps/LLMOps Security](../03_mlops_llmops_security/tutorial.md)

## The Situation

A product analyst notices a competitor just launched a feature that behaves suspiciously
similarly to a proprietary model your company has spent a year and significant compute
budget training — similar enough in its specific quirks and failure modes that it looks
more like a copy than an independent effort. Around six months ago, one API key on your
public-facing inference endpoint had a sustained period of unusually high query volume.
Nobody flagged it at the time — it didn't look like an outage or abuse in the traditional
sense, just a lot of legitimate-looking requests.

## First Questions to Ask

- What does "unusually high" actually mean here, quantitatively — how does that API key's
  volume compare to your typical top-percentile legitimate customer, not just the average?
- Was the query pattern from that key characteristic of a real product use case (bursty,
  correlated with a customer's own traffic patterns) or systematic and broad-coverage —
  querying across a wide, methodical range of inputs rather than a narrow task-specific
  slice?
- Is there any existing rate limiting on this API, and if so, was this key within its
  limits — meaning the limits themselves are tuned too loosely to catch this pattern?
- Does the API return raw model outputs (full probability distributions, detailed
  confidence scores) or just a final decision — richer outputs make extraction
  meaningfully easier and faster.
- Is there any way to correlate this API key's account details with the competitor, even
  indirectly (billing info, registration email domain, IP ranges)?

## Likely Root Causes (ranked)

1. **No rate limiting or anomaly detection tuned to catch systematic high-volume querying
   distinct from normal usage.** Standard rate limiting protects against abuse patterns
   like scraping or denial-of-service, but a model-extraction attack often looks like a
   large but individually reasonable stream of requests — the signal isn't "too many
   requests too fast," it's "requests that systematically sample the input space in a way
   no real product use case would." Without anomaly detection tuned to that specific
   pattern, sustained high-volume-but-legitimate-looking querying goes unnoticed for
   exactly as long as it did here.
2. **No watermarking or output-perturbation strategy.** Nothing in the model's outputs was
   designed to make a distilled copy detectable or degraded — if the model had, for
   instance, subtly perturbed low-confidence outputs or embedded a detectable statistical
   watermark, a copy trained on its outputs could potentially be identified after the fact,
   which isn't possible now.
3. **No per-API-key usage monitoring that would have flagged the pattern early.** Even
   without sophisticated anomaly detection, basic usage dashboards broken down per API key
   (volume over time, diversity of query content) would have made this key's behavior
   visible to a human reviewing normal operational dashboards — this is a lower bar than
   automated detection and wasn't in place either.

## Diagnostic Path

1. **Pull the full query history for the flagged API key** — volume over time, and
   critically, the *content* of the queries (are they systematically covering a broad input
   space, or characteristic of a real product feature's traffic).
2. **Compare that key's volume and pattern against your legitimate top-percentile customers**
   — establish what "unusual" actually means quantitatively rather than relying on a
   retrospective gut feeling.
3. **Check what the API actually returns** — full logits/probabilities versus a coarse final
   answer — richer responses both make the original extraction easier and make this
   specific incident more likely to be the actual mechanism, worth confirming rather than
   assuming.
4. **Cross-reference the API key's registration and billing metadata** for any link to the
   competitor, understanding this may be inconclusive — extraction attacks are often run
   through intermediaries specifically to avoid this kind of traceability.
5. **Check whether current rate limits would even catch a repeat of this exact pattern
   today** — if the answer is no, that's the most actionable, immediately fixable finding
   independent of whether this specific incident can be conclusively attributed.

## The Fix

- **Immediate mitigation**: rate-limit or suspend the flagged API key pending investigation,
  and reduce the granularity of information returned by the public API (coarser confidence
  scores, no full probability distributions) as an interim step that doesn't require a
  full redesign.
- **Long-term fix**: implement anomaly detection tuned specifically to systematic,
  broad-coverage querying patterns (not just volume-based rate limiting), evaluate
  watermarking or output-perturbation techniques appropriate to the model's domain, and
  build per-API-key usage monitoring as a standing operational dashboard reviewed
  regularly — not just as an incident-response tool after the fact.

## Prevention

The systemic lesson: **a public inference API is a data-exfiltration surface for the model
itself, not just an interface to it** — every input/output pair returned is training signal
for anyone systematically collecting them, which means the usual abuse controls (rate
limiting tuned for scraping or DoS) aren't automatically sufficient, because a
well-resourced extraction attempt can look exactly like a large but legitimate customer if
nothing is specifically watching for the *pattern* of queries rather than just their
volume. See the model-extraction and serving-layer security discussion in the
[MLOps/LLMOps Security tutorial](../03_mlops_llmops_security/tutorial.md).

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **API-as-exfiltration-surface framing (the default for this scenario):** "A public
  inference API isn't just an interface to the model, it's a data-exfiltration surface for
  the model itself — every returned output is training signal for anyone systematically
  collecting enough of them. I'd design monitoring for that reality, not just for
  traditional abuse patterns."
- **Volume-vs-pattern framing (good for explaining why rate limiting alone wasn't enough):**
  "Standard rate limiting catches 'too many requests too fast.' Model extraction often
  looks like a large but individually reasonable stream — the signal is systematic
  coverage of the input space, not raw volume, and that needs a different kind of
  detection entirely."
- **Retrospective-uncertainty framing (good for the diagnostic angle):** "I'd be honest that
  attribution here may stay inconclusive — extraction attempts are often run through
  intermediaries specifically to avoid traceability. The actionable finding isn't 'prove it
  was them,' it's 'would our current controls catch a repeat of this pattern today,' and
  fixing that doesn't depend on resolving the attribution question."

### Vocabulary Builder

- **model extraction** (n. phrase) — systematically querying a model's public API to
  collect enough input/output pairs to train a functionally similar copy, without ever
  accessing the original model's weights directly.
- **output perturbation** (n. phrase) — deliberately introducing small, controlled noise
  into a model's returned outputs to make extraction harder or a resulting copy
  detectable, at some cost to output precision for legitimate users.
- **watermarking** (n.) — embedding a detectable statistical signature into a model's
  outputs, so a model trained on those outputs can potentially be identified as a
  derivative later.
- **"…a large but individually reasonable stream of requests"** — a precise way to describe
  why volume-based abuse detection misses systematic extraction, since no single request
  looks wrong.

---

**Previous:** [6. An Overprivileged Service Account Enabled Lateral Movement](06_overprivileged_service_account_lateral_movement.md)  |  **Next:** [8. A Compromised Dependency Shipped to Production](08_dependency_supply_chain_compromise.md)
