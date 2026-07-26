# 6. An Overprivileged Service Account Enabled Lateral Movement

**Primary topic:** [Cloud Security](../02_cloud_security/tutorial.md) and
[Foundations](../00_foundations/tutorial.md#the-cia-triad-and-the-vocabulary-built-on-it)
(least privilege, blast radius)

## The Situation

A public-facing image-resizing microservice — low-sensitivity, no customer data of its
own, the kind of service nobody thinks much about — gets compromised via an unpatched
image-parsing library vulnerability. That much is contained and expected: it's a small,
low-value service. Two days later, the security team notices read access logs on a
completely unrelated, much more sensitive customer-data service, originating from
credentials associated with the image-resizing service's service account. Nobody expected
those two services to have anything to do with each other.

## First Questions to Ask

- What permissions does the image-resizing service's service account actually have —
  and do any of them extend beyond what image resizing itself requires?
- Is there network-level segmentation between these two services, or can any workload in
  the environment reach any other workload's network endpoints by default?
- How was the service account's permission scope originally decided — was it deliberately
  scoped, or was it granted broad access "because it was simpler than scoping each service
  individually"?
- What does normal access-pattern behavior for the sensitive customer-data service look
  like, and how far outside that pattern is this access — would an anomaly-detection
  system, if one existed, have flagged this quickly?
- Is this the first time this service account's credentials have been used to reach
  anything outside the image-resizing service's own resources?

## Likely Root Causes (ranked)

1. **No per-service least-privilege scoping.** The image-resizing service's account was
   granted broad, cluster-wide or account-wide permissions rather than a scope limited to
   exactly what image resizing needs (read/write to its own storage bucket, nothing more)
   — "simpler than scoping each service individually" is precisely the reasoning that turns
   a contained compromise of one low-value service into unrestricted reach across the
   environment. This is the root cause that directly explains *how* lateral movement to an
   unrelated service was even possible.
2. **No network segmentation between services of different sensitivity tiers.** Even with
   IAM permissions in place, if the image-resizing service's network position can reach the
   customer-data service's endpoints at all, that's a second, independent path enabling the
   same lateral movement — permission scoping and network segmentation are two separate
   controls, and this incident suggests neither was applied here.
3. **No anomaly detection on cross-service access patterns.** A service account associated
   with an image-resizing workload suddenly reading from a customer-data service is a stark
   deviation from any reasonable baseline — nothing here flagged that access as anomalous
   in real time; it was found two days later during a routine log review prompted by the
   original compromise, not by a control designed to catch exactly this pattern.

## Diagnostic Path

1. **Pull the full IAM policy attached to the image-resizing service's account** and
   compare it against the actual permissions the service's code paths use — the gap
   between granted and used quantifies exactly how much broader the grant was than
   necessary.
2. **Check the network path between the two services** — is there a security group,
   network policy, or segmentation rule that should prevent this reachability, and if one
   exists, why didn't it apply here?
3. **Pull access logs for the customer-data service** covering the full window since the
   original compromise (not just when the anomaly was noticed) — establish the actual
   scope of what was read, not just the fact that access occurred.
4. **Check whether this credential's access pattern would have tripped any existing
   anomaly-detection baseline**, and if none exists, that absence is itself a finding
   independent of what's found in the logs.
5. **Audit other low-sensitivity, public-facing services for the same overprivileged
   service-account pattern** — a compromised image-resizing service is rarely the only
   workload in an environment that was scoped "for simplicity" rather than deliberately.

## The Fix

- **Immediate mitigation**: revoke or aggressively scope down the image-resizing service
  account's permissions immediately, and treat the customer-data service's data as
  potentially exposed for the full window since the original library compromise — not just
  since the anomaly was noticed. Patch the original image-parsing library vulnerability
  that enabled the initial compromise.
- **Long-term fix**: scope every service account to the minimum permissions its own service
  actually requires, with no exceptions made "for simplicity" — the cost of scoping each
  service individually is real but bounded, while the cost of one compromised low-value
  service reaching arbitrary sensitive data is not. Add network segmentation between
  services with meaningfully different sensitivity tiers so a compromise is contained at
  the network layer even if an IAM scoping gap is later reintroduced, and add anomaly
  detection on cross-service access patterns as a detection backstop.

## Prevention

The systemic lesson: **least privilege is what determines blast radius, and blast radius is
what turns a contained incident into a much larger one** — the initial compromise here (an
unpatched library in a low-value service) was, on its own, a normal, expected-eventually
kind of incident. What made it serious was a permission grant made for convenience months
or years earlier, with no connection in anyone's mind at the time to "this could someday let
an attacker reach the customer-data service." Scoping decisions and their blast-radius
consequences are separated in time, which is exactly why they're easy to under-invest in
when the scoping decision is being made. See the least-privilege and blast-radius discussion
in the [Foundations tutorial](../00_foundations/tutorial.md#the-cia-triad-and-the-vocabulary-built-on-it)
and the IAM-scoping and network-segmentation patterns in the
[Cloud Security tutorial](../02_cloud_security/tutorial.md).

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Blast-radius framing (the default for this scenario):** "The initial compromise — an
  unpatched library in a low-value service — was always going to happen to some service
  eventually. What determined whether it stayed contained was a permission-scoping decision
  made long before, with no visible connection at the time to this specific consequence.
  That's exactly why blast radius has to be argued for proactively, not after an incident
  makes it obvious."
- **Two-independent-controls framing (good for the diagnostic angle):** "IAM scoping and
  network segmentation are two separate controls, and this incident suggests neither one
  was in place — I'd check both independently rather than assuming fixing one implies the
  other was fine."
- **Convenience-debt framing (good for explaining the root cause honestly):** "'Simpler
  than scoping each service individually' is a real, understandable trade-off under
  deadline pressure — but it's a form of debt, and the interest on it is paid in incidents
  like this one, at a time and place nobody planned for when the shortcut was taken."

### Vocabulary Builder

- **lateral movement** (n. phrase) — an attacker's progression from an initially
  compromised system to other systems in the environment, typically by reusing credentials
  or exploiting excessive trust between components.
- **sensitivity tier** (n. phrase) — a classification of how sensitive a service's data or
  function is, used to decide how much network/permission isolation it needs from services
  in other tiers.
- **blast radius** (n. phrase) — the scope of damage possible if a given identity or
  component is fully compromised; the quantity a least-privilege scoping decision is
  directly trying to minimize.
- **"…a shortcut taken for convenience, paid for later as an incident"** — a fluent way to
  connect a scoping decision made under time pressure to its eventual, often much later,
  consequence.

---

**Previous:** [5. An Agent's Tool Call Triggered SSRF](05_agent_tool_call_ssrf.md)  |  **Next:** [7. Model Extraction via the Public Inference API](07_model_extraction_via_public_api.md)
