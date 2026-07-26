# 5. An Agent's Tool Call Triggered SSRF

**Primary topic:** [LLM Security](../01_llm_security/tutorial.md) (excessive agency) and
[Foundations](../00_foundations/tutorial.md#appsec-the-owasp-top-10-as-categories-of-reasoning-not-a-list-to-memorize)
(SSRF)

## The Situation

An internal agent with a "fetch a URL and summarize it" tool is being used to summarize a
long internal document. The summary it returns includes a strange fragment near the end —
something that looks like an access key and a short-lived token, formatted like cloud
metadata output, sitting right after a sentence about "including the requested
diagnostic information for the infrastructure team." Nobody on the infra team requested
any diagnostic information.

## First Questions to Ask

- What exactly was in the document the agent was asked to summarize — does it contain any
  embedded instructions (visible or hidden in formatting/metadata) telling the agent to
  fetch a second URL?
- Does the agent's fetch tool have any restriction on destination — can it reach arbitrary
  hosts, including internal/link-local addresses like `169.254.169.254`, or is it
  restricted to public internet destinations only?
- Is the agent running with network access that can reach the cloud instance metadata
  endpoint at all — is there a network-level control (not just an application-level one)
  that should be blocking that reachability regardless of what the agent is told to do?
- Did the agent actually make a second tool call beyond the one the user requested, and if
  so, what was the destination and what did the response contain?

## Likely Root Causes (ranked)

1. **The fetch tool has no destination allowlist/denylist.** The tool was built to fetch
   "a URL" generically, with no restriction on which URLs are acceptable — a prompt-injected
   instruction embedded in the document being summarized was able to direct a second fetch
   to the cloud instance metadata endpoint (`http://169.254.169.254/...`) exactly as easily
   as it could fetch any legitimate URL, because nothing distinguishes the two at the tool
   level.
2. **The agent's execution environment has network access to the metadata endpoint at
   all.** Even if the tool-level allowlist were bypassed somehow, a network-level block on
   reaching `169.254.169.254` from this workload would have stopped the request before it
   ever reached the endpoint — this is the defense-in-depth layer that should exist
   independent of application logic, and its absence is what turned a successful injection
   into actual credential exposure rather than a blocked request.
3. **No output-side detection of credential-shaped strings.** Even with the SSRF
   succeeding and returning metadata-endpoint credentials into the agent's context, an
   output-side check scanning for credential-shaped patterns (access-key formats, token
   structures) before the response reached the user could have caught this before the
   credentials left the system entirely — this didn't happen either.

## Diagnostic Path

1. **Retrieve the source document and inspect it for injected instructions** — hidden text,
   unusual formatting, or content specifically directing the agent to fetch a second URL.
   Confirm the injection mechanism before anything else.
2. **Pull the agent's full tool-call trace for this session** — confirm whether a second
   fetch occurred, and to what exact destination, distinguishing a confirmed SSRF from a
   coincidental resemblance in the output text.
3. **Test the fetch tool directly against the metadata endpoint** in a controlled
   environment — confirm whether it's reachable at all from the agent's network context,
   independent of whether the injection successfully directed it there in this incident.
4. **Check for any existing destination allowlist/denylist configuration** on the fetch
   tool — if one exists but didn't block this, that's a configuration gap; if none exists,
   that's a design gap.
5. **Scan the returned response for the actual credential material** and check whether it
   was live/usable — this determines the actual severity and whether downstream
   credential rotation is needed, not just a tool-access fix.

## The Fix

- **Immediate mitigation**: rotate any credentials that were exposed in the response
  immediately, treat them as compromised regardless of whether misuse is confirmed. Block
  the fetch tool's ability to reach the metadata endpoint and any other internal/link-local
  address ranges at the network level as an emergency change, not just at the application
  level.
- **Long-term fix**: enforce a destination allowlist (or at minimum a denylist covering
  cloud metadata endpoints and all internal/link-local ranges) on any agent tool capable of
  making outbound requests, block network-level reachability to the metadata endpoint from
  workloads that don't specifically need it (most application workloads don't), and add an
  output-side scan for credential-shaped strings in any agent response before it's returned
  — treating this as a permanent layer, not an incident-specific patch.

## Prevention

The systemic lesson: **SSRF isn't a new vulnerability class in an agentic system — it's the
same OWASP-Top-10 SSRF pattern, just reachable through a new trust boundary (natural
language instructions embedded in content the agent processes, rather than a URL parameter
a user submits directly).** The fix is the same layered response either way: restrict what
destinations the fetch capability can reach, and don't rely on any single layer (tool-level
allowlist, network-level block, output-side scanning) to be the only thing standing between
an attacker-controlled instruction and a sensitive internal endpoint. See the SSRF entry in
the [Foundations OWASP table](../00_foundations/tutorial.md#appsec-the-owasp-top-10-as-categories-of-reasoning-not-a-list-to-memorize)
and the excessive-agency and indirect-injection discussion in the
[LLM Security tutorial](../01_llm_security/tutorial.md).

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Old-vulnerability-new-boundary framing (the default for this scenario):** "This is
  literally SSRF — the same OWASP category as a web app's 'import from URL' feature
  hitting the metadata endpoint. The only thing that's new is the trust boundary: the
  malicious instruction arrives via natural language embedded in a document, not a URL
  parameter a user typed directly. The fix doesn't need to be reinvented for agents."
- **Layered-failure framing (good for explaining why this wasn't caught):** "Three
  independent layers could each have stopped this — a tool-level allowlist, a
  network-level block on the metadata endpoint, an output-side credential scan — and none
  of them existed. This wasn't one control failing; it was defense in depth that was never
  built in the first place."
- **Blast-radius framing (good for the fix discussion):** "I'd ask why this workload can
  reach the metadata endpoint over the network at all, independent of the agent's logic —
  most workloads don't need that reachability, and blocking it at the network layer means
  the application-level bug stops mattering for this specific endpoint."

### Vocabulary Builder

- **SSRF (Server-Side Request Forgery)** (n. phrase) — tricking a server or agent into
  making a request to an attacker-chosen destination, here the cloud instance metadata
  endpoint, using the server's own network position and credentials.
- **instance metadata endpoint** (n. phrase) — a link-local address
  (`169.254.169.254` on most major clouds) that returns credentials and configuration for
  the current compute instance; a classic SSRF target precisely because it requires no
  authentication from the instance's own network position.
- **destination allowlist** (n. phrase) — restricting an outbound-capable tool to a
  specific, pre-approved set of reachable destinations, rather than trusting the tool's
  logic alone to avoid unsafe ones.
- **"…the same vulnerability, reachable through a new trust boundary"** — a reusable frame
  for arguing that an agentic-system vulnerability is a familiar class in a new setting,
  not something requiring an entirely new mental model.

---

**Previous:** [4. Jailbreak Bypassed Guardrail in Production](04_jailbreak_bypassed_guardrail_in_prod.md)  |  **Next:** [6. An Overprivileged Service Account Enabled Lateral Movement](06_overprivileged_service_account_lateral_movement.md)
