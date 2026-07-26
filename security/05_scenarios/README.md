# Security Incident Scenarios

Eight scenario-debugging problems — each a realistic, ambiguous security incident as it
would actually reach you (a Slack ping from an on-call engineer, a SOC alert, a suspicious
usage-pattern dashboard, a security researcher's disclosure email), not a clean textbook
question. This is where the threat-modeling and defense-in-depth vocabulary from the five
tutorials in this track gets exercised under the kind of ambiguity a real incident actually
has — not just recited on command.

## Why This Format

A security design round tests whether you can *build* a system with the right controls in
place. These test whether you can *reason about one that's already been breached* —
arguably the harder and more senior skill, and the one that actually maps to what an
on-call security response looks like: incomplete information, multiple plausible causes,
and pressure to act before you've fully diagnosed the problem. Each scenario is structured
the same way:

1. **The Situation** — what you'd actually be told, often vague or slightly misleading
2. **First Questions to Ask** — what you'd clarify before touching anything, since acting
   on an assumption is the most common way candidates go down the wrong path (and, in a
   real incident, the most common way you make the blast radius worse)
3. **Likely Root Causes** — ranked hypotheses, not just one "correct" answer
4. **Diagnostic Path** — the concrete steps you'd take to confirm or rule out each
   hypothesis
5. **The Fix** — immediate mitigation vs. the actual long-term fix (these are often
   different, and conflating them is a real interview tell — "rotate the credential" is
   not the same fix as "stop issuing long-lived credentials with this scope at all")
6. **Prevention** — the systemic lesson, linked back to the tutorial covering the relevant
   pattern

## The Scenarios

| # | Scenario | Primary Topic |
|---|---|---|
| 1 | [Leaked Cloud Credential via CI](01_leaked_cloud_credential_via_ci.md) | Cloud Security |
| 2 | [Indirect Prompt Injection Exfiltrated Internal Data](02_indirect_prompt_injection_exfiltration.md) | LLM Security |
| 3 | [A Fine-Tuned Model Has a Backdoor Trigger](03_poisoned_training_data_backdoor.md) | LLM Security / MLOps Security |
| 4 | [Jailbreak Bypassed Guardrail in Production](04_jailbreak_bypassed_guardrail_in_prod.md) | LLM Security |
| 5 | [An Agent's Tool Call Triggered SSRF](05_agent_tool_call_ssrf.md) | LLM Security / Foundations |
| 6 | [An Overprivileged Service Account Enabled Lateral Movement](06_overprivileged_service_account_lateral_movement.md) | Cloud Security |
| 7 | [Model Extraction via the Public Inference API](07_model_extraction_via_public_api.md) | MLOps/LLMOps Security |
| 8 | [A Compromised Dependency Shipped to Production](08_dependency_supply_chain_compromise.md) | Cloud Security |

## How to Practice These

- **Read only "The Situation" first.** Stop. Say your clarifying questions out loud before
  reading further — this is the step most people skip under real incident pressure, and
  it's the one that separates a senior response from a junior one (a junior response
  starts remediating before it's finished diagnosing).
- **Commit to a ranked list of hypotheses before reading "Likely Root Causes."** Compare
  your reasoning, not just your final answer — in a security incident, the order you
  investigate hypotheses in has real cost, since chasing the wrong one first can let an
  active compromise continue.
- **Every scenario here has more than one contributing cause** — real security incidents
  almost always do (a leaked credential *and* no secret-scanning gate; an overprivileged
  service account *and* no network segmentation). Resist the urge to stop investigating
  after finding the first plausible cause, and resist the urge to name only the technical
  cause without also naming the missing control that would have caught it.

---

**Previous:** [Case Study: Secure Multi-Tenant ML Platform](../04_security_system_design/design_secure_multi_tenant_ml_platform.md)  |  **Next:** [1. Leaked Cloud Credential via CI](01_leaked_cloud_credential_via_ci.md)
