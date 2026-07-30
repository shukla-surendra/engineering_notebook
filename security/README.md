# Security Engineering

Security depth for a cloud/MLOps/LLMOps engineer, organized the same way the
[ML System Design](../system_design_foundation/README.md) and
[Distributed Systems Design](../system_design_practice/README.md) tracks are: tutorials that build
vocabulary and mental models, worked "design a secure X" case studies, and a bank of
incident-debugging scenarios. Independent of those two tracks, but written to assume the
same shared vocabulary (p99 latency, sharding, idempotency — see the [Prerequisite
Concepts primer](../system_design_foundation/prerequisite_concepts/01_performance_and_scale.md)
if any of that isn't second nature yet).

## Why Security Gets Its Own Track

Most system design tutorials treat security as a bullet point at the end — "add auth,
encrypt at rest, done." That's not enough depth for a cloud/MLOps/LLMOps role, where
security questions show up in three distinct flavors: **classic application/cloud
security** (the ground every engineer is expected to know), **LLM-specific security**
(a genuinely new attack surface most senior engineers haven't internalized yet — prompt
injection, jailbreaks, data poisoning), and **the security of the ML/LLM pipeline itself**
(who can write to a feature store, sign a model artifact, or push a prompt change to
prod). This track builds all three, then combines them into secure system design and
realistic incidents.

## Reading Order

| # | Tutorial | Covers |
|---|---|---|
| 0 | [Foundations](00_foundations/tutorial.md) | CIA triad, OWASP Top 10, network security & TLS/PKI, crypto essentials, IAM/authN/authZ (OAuth2/OIDC/SAML, mTLS), threat modeling (STRIDE) |
| 1 | [LLM Security](01_llm_security/tutorial.md) | OWASP LLM Top 10, prompt injection, jailbreaks, data/model poisoning, training-data extraction, insecure output handling, excessive agency, RAG-specific risks, guardrails & red-teaming |
| 2 | [Cloud Security](02_cloud_security/tutorial.md) | Cloud IAM & least privilege, network segmentation/VPC design, secrets management, supply-chain/artifact security, container/Kubernetes security, shared responsibility model |
| 3 | [MLOps/LLMOps Security](03_mlops_llmops_security/tutorial.md) | Feature store & model registry access control, training-data integrity, model artifact signing/provenance, serving-layer security, model extraction/inversion, LLM gateway security, audit/lineage |
| 4 | [Security System Design](04_security_system_design/tutorial.md) | A repeatable framework — threat model first, trust boundaries, defense in depth — plus worked case studies: [Secure RAG Pipeline](04_security_system_design/design_secure_rag_pipeline.md) and [Secure Multi-Tenant ML Platform](04_security_system_design/design_secure_multi_tenant_ml_platform.md) |

## Security Incident Scenarios: debugging, not designing

[**Incident-debugging scenarios**](05_scenarios/README.md) — realistic, ambiguous security
incidents (a leaked cloud credential, an indirect prompt injection that exfiltrated data, a
jailbreak that slipped past a guardrail in production, a poisoned training set) with a
structured walkthrough of clarifying questions, ranked hypotheses, diagnostic steps, the
fix, and the systemic lesson — the same format as
[system_design's Tricky MLOps Scenarios](../system_design_foundation/12_tricky_scenarios/README.md). Every
scenario cross-references the tutorial covering its underlying pattern.

## How to Practice This

- **Threat-model out loud before designing anything.** The single biggest signal gap
  between a senior and staff answer on a security question is whether trust boundaries and
  attacker capabilities get named *before* countermeasures — see
  [4. Security System Design](04_security_system_design/tutorial.md) for the framework.
- **Anchor to your own systems.** If you operate a cloud/MLOps/LLMOps platform today, what
  actually enforces least privilege on your feature store or model registry — and what's
  the honest answer if a teammate asks "what stops someone from doing X"?
- **Treat LLM security as genuinely new ground, not a subset of AppSec.** Prompt injection
  and jailbreaks don't map cleanly onto SQL injection or XSS — the trust boundary is
  natural language itself, which is why [1. LLM Security](01_llm_security/tutorial.md) is
  its own tutorial rather than a section of Foundations.
