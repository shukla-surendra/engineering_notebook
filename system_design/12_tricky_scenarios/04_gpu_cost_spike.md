# 4. Overnight GPU Cost Spike

**Primary topic:** [RAG + LLM-Serving at Scale](../06_rag_llm_serving_at_scale/tutorial.md)

## The Situation

Your LLM-serving platform's GPU spend tripled overnight. Request volume (requests/sec) in
your dashboards looks essentially flat compared to the day before. Finance wants an
explanation by end of day.

## First Questions to Ask

- Is "request volume" measured in requests/sec, or in **tokens processed**? (These can
  diverge enormously for an LLM system — this is the first thing to check, not assume.)
- Did any prompt template, system prompt, or RAG configuration (e.g. number of retrieved
  chunks, chunk size) change recently?
- Did the autoscaler's behavior change — is it running more replicas than usual for the
  same traffic, or the same replica count at higher per-replica cost?
- Was there a model version change, a new fine-tune, or a routing change (e.g. more
  traffic accidentally routed to a larger/more expensive model variant)?

## Likely Root Causes (ranked)

1. **A prompt/context-length change silently increased tokens-per-request.** The most
   common cause of "flat requests, tripled cost" in LLM serving: a RAG configuration
   change (retrieving more chunks, or switching to a larger chunk size), a system-prompt
   change, or a client-side change sending longer inputs — all increase tokens per
   request without changing requests/sec at all, and cost scales with tokens, not
   requests (see the [RAG tutorial](../06_rag_llm_serving_at_scale/tutorial.md#chunking-strategy)).
2. **A routing misconfiguration sending traffic to a larger/more expensive model.** If
   there are multiple model variants (a small fast model, a large capable one) behind a
   router, a misconfigured routing rule or a canary that never got ramped back down could
   be sending disproportionate traffic to the expensive variant.
3. **Autoscaler thrashing or a misconfigured scaling policy** keeping more replicas warm
   than needed — e.g. a scale-to-zero cooldown period set too conservatively long, or a
   custom-metric autoscaler (from the
   [serving tutorial](../04_model_serving_deployment/tutorial.md#autoscaling-for-model-serving))
   reacting to a noisy metric and over-provisioning.
4. **A batching regression** — if continuous batching (from the
   [RAG tutorial's vLLM deep-dive](../06_rag_llm_serving_at_scale/tutorial.md#deep-dive-llm-serving-internals-vllm-on-triton))
   stopped functioning correctly (a config regression, a deploy that reverted a batching
   setting), GPU utilization per request would drop, requiring more GPU-time for the same
   request volume.

## Diagnostic Path

1. **Pull tokens-processed (input + output) per hour, not just requests/sec**, and
   overlay against the cost spike timeline — if tokens/sec jumped while requests/sec
   stayed flat, that immediately confirms root cause #1's category and narrows the search
   to *what* changed to increase per-request token count.
2. **Diff configuration changes** (prompt templates, RAG retrieval-count settings, model
   routing rules, autoscaler policies) against deploy history for the exact window the
   spike began.
3. **Check per-model-variant request distribution** if multiple variants exist — confirm
   the expensive variant's traffic share didn't unexpectedly increase.
4. **Check GPU utilization and batch size metrics** directly — a drop in average batch
   size or GPU utilization per replica, with flat request volume, points to a batching
   regression rather than a token-volume increase.

## The Fix

- **Immediate mitigation**: if a specific config change (prompt/RAG/routing) is identified,
  revert it immediately given the cost impact, then re-introduce it deliberately with
  proper cost impact analysis once fixed.
- **Long-term fix**: add tokens-processed and cost-per-request as first-class monitored
  metrics (not just requests/sec), with alerting on sudden changes — and gate any
  prompt/RAG-configuration change behind a cost-impact check in CI/review, the same way a
  code change gets a quality gate.

## Prevention

The systemic lesson: **for LLM systems, requests/sec is the wrong primary capacity/cost
metric** — tokens processed is the actual unit of cost and load, and any dashboard that
only shows requests/sec is blind to exactly this class of incident. See the
token-throughput discussion in the
[RAG/LLM-serving tutorial](../06_rag_llm_serving_at_scale/tutorial.md) and the
[observability tutorial's LLM-specific signals](../05_observability_drift/tutorial.md#llm-specific-observability-signals).

---

**Previous:** [3. Silent Drift, No Alert](03_silent_drift_no_alert.md)  |  **Next:** [5. Retrieval Looks Right, Answers Are Wrong](05_rag_retrieval_correct_answers_wrong.md)
