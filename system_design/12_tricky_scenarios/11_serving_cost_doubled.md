# 11. Serving Cost Doubled After a "Routine" Upgrade

**Primary topic:** [Cost, Security & Multi-Region Governance](../10_cost_security_multiregion/tutorial.md)

## The Situation

A model was upgraded to a new version with slightly better offline accuracy — described
internally as a "routine" update, same architecture family, same team's normal release
process. A month later, finance flags that serving cost per request has roughly doubled,
and nobody connected it to the model update at the time.

## First Questions to Ask

- Is the new model version actually the same size/architecture as the old one, or did
  "slightly better accuracy" come from a larger model, more layers, or a longer
  context/input window?
- Did the autoscaling configuration or instance type change alongside the model update,
  or stay identical?
- Is cost being measured per-request, or as a raw total — and has request volume also
  changed in that window, potentially masking or explaining part of the shift?
- Was there a canary/gradual rollout for this update, and if so, did cost-per-request
  get monitored *during* the canary, or only discovered later?

## Likely Root Causes (ranked)

1. **The "slightly better accuracy" model is meaningfully larger or slower**, and nobody
   evaluated the accuracy-vs-latency/cost trade-off explicitly before shipping — a common
   pattern: a team optimizing purely for an offline accuracy metric picks a bigger model
   or ensemble without an equally rigorous cost/latency evaluation, because cost isn't
   part of their model-selection criteria by default. This ties directly to the
   accuracy-vs-latency trade-off axis in the
   [interview framework's trade-off cheat sheet](../00_interview_framework/tutorial.md#the-trade-off-vocabulary-cheat-sheet).
2. **A batching or autoscaling regression introduced alongside the update** — e.g. the new
   model's different input/output shape broke an assumption in the batching logic,
   reducing effective batch size and thus GPU utilization efficiency per request, even if
   the model itself isn't meaningfully bigger.
3. **Reduced multi-tenancy efficiency** — if the old model shared infrastructure
   efficiently with other models/tenants (e.g. via multi-LoRA serving) and the new
   version isn't compatible with that sharing mechanism, it may now require dedicated
   capacity that wasn't previously needed, increasing effective cost even at the same
   request volume.

## Diagnostic Path

1. **Compare the two model versions' actual size/compute characteristics directly** —
   parameter count, FLOPs per inference, average input/output token or feature count —
   don't rely on "same architecture family" as a proxy for "same cost profile."
   Confirm or rule out root cause #1 with real numbers.
2. **Compare average batch size and GPU utilization per replica** before and after the
   update — a drop here, even with a same-sized model, points to root cause #2.
3. **Check whether the new model is still compatible with any shared-serving mechanism**
   (multi-LoRA, shared base weights) the old one used — a silent loss of that sharing is
   root cause #3, and is easy to miss since nothing "breaks," it just becomes less
   efficient.
4. **Reconstruct what a canary would have shown**, if one was run — pull cost-per-request
   metrics from the canary window specifically, even retroactively, to determine whether
   this was detectable before full rollout (informs the prevention fix below) or only
   emerges at full production scale/traffic patterns.

## The Fix

- **Immediate mitigation**: if the cost increase isn't justified by a proportional
  business-value increase from the accuracy improvement, consider reverting to the
  previous version while a cost-aware re-evaluation happens, or explore
  quantization/distillation to recover some of the lost efficiency from the new model
  without fully reverting.
- **Long-term fix**: add cost-per-request as an explicit, required guardrail metric in
  every model-promotion pipeline (tying to the CI quality gates discussed in the
  [GitOps tutorial](../09_gitops_ml_cicd/tutorial.md#cicd-testing-strategy-specific-to-ml))
  — a model shouldn't be promotable on accuracy alone if it regresses cost beyond an
  agreed threshold without explicit sign-off.

## Prevention

The systemic lesson: **"routine" model updates aren't routine from a cost perspective
unless cost is explicitly checked as part of the routine** — accuracy improvements are
almost never free, and the accuracy-vs-latency/cost trade-off (from the
[interview framework](../00_interview_framework/tutorial.md)) needs to be an explicit,
monitored gate in the promotion process, not an implicit assumption that "same
architecture family" means "same cost."

---

**Previous:** [10. Reconstructing Model Lineage for an Audit](10_audit_lineage_reconstruction.md)  |  **Next:** [12. DR Failover Took 8x Longer Than Planned](12_dr_failover_slow.md)
