# System Design — Track B

This section covers **Track B (ML System Design)** from the 12-week transition plan:
moving from "described components" to "reasoned about trade-offs." It's organized as a
sequence of tutorials that follow the plan's week-by-week topic order, extended with five
more covering Track D tools and LLMOps — topics that don't have their own tutorial in the
original plan — plus a bank of scenario-debugging problems.

## How the core six map to the transition plan

| Weeks | Tutorial | Anchor it to |
|---|---|---|
| 1-2 | [Fundamentals: Building Blocks](01_fundamentals/tutorial.md) | General warm-up (URL shortener, rate limiter) |
| 3-4 | [High-Throughput Ingestion Pipelines](02_ingestion_pipeline/tutorial.md) (+ [deep-dive: Lambda vs. EKS cost/speed](02_ingestion_pipeline/serverless_vs_eks_receipt_processing.md)) | Your production-style pipeline: Step Functions/Lambda/S3/Databricks |
| 5-6 | [Feature Store + Model Promotion](03_feature_store_model_promotion/tutorial.md) (+ [deep-dive: is it worth it when reuse is low?](03_feature_store_model_promotion/is_a_feature_store_worth_it.md)) | Your production ML platform: dev/qa/stage/prod/ml-prod, Unity Catalog, MLflow, Feast |
| 7-8 | [Model Serving & Deployment](04_model_serving_deployment/tutorial.md) | Extending a production serving layer with canary/shadow, KServe/Seldon |
| 9-10 | [ML/LLM Observability & Drift](05_observability_drift/tutorial.md) | Generalizing a production drift/monitoring setup; Prometheus/Grafana + Evidently/Arize |
| 11-12 | [RAG + LLM-Serving at Scale](06_rag_llm_serving_at_scale/tutorial.md) | Your Track C project: vector DB, LangChain, vLLM/Triton serving |

Read **[00 — The Interview Framework](00_interview_framework/tutorial.md)** before any of
the topic tutorials — it's the four-step structure (clarify → high-level design → deep-dive
→ trade-offs) every tutorial in this section is written around, plus a clarifying-question
bank and a trade-off vocabulary cheat sheet you'll reuse in every round.

## Five more tutorials: Track D tools and LLMOps, given the same full treatment

The transition plan's Track D checklist lists several tools as "mention if a round goes
there — not worth dedicated build time." These tutorials give them (plus LLMOps, which the
original plan folds into RAG but deserves its own treatment) the same full treatment as the
core six, since a real interview follow-up on any of them deserves more than a name-drop:

| Tutorial | Covers |
|---|---|
| [7. Distributed Training & Ray/Ray Serve](07_distributed_training_serving/tutorial.md) | Data/model/pipeline parallelism, checkpointing at scale, Ray Core/Train/Serve |
| [8. ML Orchestration](08_ml_orchestration/tutorial.md) | Kubeflow/Argo Workflows vs. Airflow — the actual architectural trade-off, not just the names |
| [9. GitOps & CI/CD for ML](09_gitops_ml_cicd/tutorial.md) | ArgoCD reconciliation, DVC vs. Delta Lake/Unity Catalog, ML-specific CI gates |
| [10. Cost, Security & Multi-Region Governance](10_cost_security_multiregion/tutorial.md) | Cost attribution, PII/compliance, RTO/RPO and active-active vs. active-passive DR |
| [11. LLMOps: Prompting, Fine-Tuning, Evals & Guardrails](11_llmops/tutorial.md) | Fine-tuning vs. RAG vs. prompting, LoRA/QLoRA, eval gates, prompt injection, LLM gateway/cost routing |

## Tricky MLOps Scenarios: debugging, not designing

[**Thirteen scenario-debugging problems**](12_tricky_scenarios/README.md) — realistic,
ambiguous production incidents (a canary that passed but still caused a P1, drift
dashboards that stayed green while a model silently degraded, GPU costs tripling
overnight, a prompt change that passed eval but bypassed a guardrail) with a structured
walkthrough of clarifying questions, ranked hypotheses, diagnostic steps, the fix, and the
systemic lesson. This tests the skill design questions don't: reasoning about a system
that's already broken, not building one from scratch. Every scenario is cross-referenced
back to the tutorial covering its underlying pattern.

## How to practice this

- **Out loud, not in writing.** Most interviews expect verbal reasoning with light
  diagramming (paper or Excalidraw), not a finished document — rehearse it that way from
  day one, not just the week before an interview.
- **Anchor to your own systems.** Every tutorial below ends with a "Make It Yours" section
  of prompts referencing your own production systems — fill those in with your own
  specifics once, then reuse that story across interviews. Real, specific trade-offs you
  actually lived through
  beat textbook architectures every time; this is a real advantage over candidates who
  only know the generic version.
- **Weight the trade-off discussion heaviest.** At senior level, interviewers score the
  deep-dive and trade-off portions far more than the high-level box diagram. The first few
  clarifying questions you ask also signal seniority more than anything after them — don't
  rush past that step.
- **One weekend block, 2-3 hrs**, ideally with a friend or a mock-interview platform, per
  the plan's weekly time budget.
