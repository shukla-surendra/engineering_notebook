# 1. Leaked Cloud Credential via CI

**Primary topic:** [Cloud Security](../02_cloud_security/tutorial.md)

## The Situation

A security engineer doing a routine audit of build-log retention notices something odd:
a CI build log from four days ago, readable by anyone with repo access, contains what
looks like a plaintext service-account key. It was printed by a debug `echo` statement
someone added while chasing an unrelated deploy failure, then never removed. The
release it was unblocking shipped fine. Nobody reported anything, because nobody noticed
until this audit.

## First Questions to Ask

- What permissions does this specific service account actually have — and is that scope
  broad because the task genuinely needs it, or because "it was easier to grant it
  everything to unblock the release"?
- How long has this credential existed in this form, and how long was the log
  actually exposed and readable before the audit caught it — is "four days" the true
  exposure window, or just when it was *discovered*?
- Who has access to this repo's build logs, and is that access broader than "the team
  that owns this pipeline" (e.g. anyone in the org, or an external contractor group)?
- Is this credential a static, long-lived key, or a short-lived token — because the
  remediation is very different depending on which?
- Are there any anomalous API calls using this credential's identity in the exposure
  window, or does audit logging even go back that far?

## Likely Root Causes (ranked)

1. **Overly broad IAM scope granted to the CI credential.** The credential was scoped to
   unblock a release quickly, not to the minimum permissions the pipeline's actual job
   requires — "it was easier to just grant it what it needed" is, in practice, almost
   always broader than what it needed. This is the root cause that determines the actual
   blast radius: a narrowly-scoped leaked credential is an incident; a broadly-scoped one
   is a much bigger one.
2. **No secret-scanning gate on build logs.** Build logs are treated as disposable
   debug output, not as a surface that needs the same scrutiny as source code — no
   automated scanner flagged a credential-shaped string before the log was persisted and
   made readable.
3. **Static, long-lived credential instead of short-lived workload identity.** The
   service account issues a key that doesn't expire on its own, meaning the exposure
   window is open-ended until someone manually rotates it — a short-lived token would
   have naturally expired well before this audit even ran.

## Diagnostic Path

1. **Pull the exact permission set attached to this service account** and compare it
   against what the pipeline's steps actually call — the gap between "granted" and
   "used" is the first number to establish, since it quantifies the real blast radius.
2. **Check the credential's type and lifecycle** — static key with no expiry, or a
   short-lived token that's already rotated out. This determines whether "the log is old"
   even matters for current risk.
3. **Audit access logs for this repo's CI build logs** — who (or what automation) actually
   viewed or downloaded that specific log in the exposure window, not just who theoretically
   *could* have.
4. **Search cloud audit/API logs for activity under this credential's identity** during the
   exposure window for anything inconsistent with the CI pipeline's normal call pattern —
   this is how you distinguish "credential was exposed but nobody used it" from "credential
   was exposed and abused."
5. **Grep recent CI history for the same anti-pattern** (debug logging of secrets, env
   dumps, verbose error output) across other pipelines — a single instance found by audit
   is rarely the only one in a codebase with no log-scanning gate.

## The Fix

- **Immediate mitigation**: rotate the credential immediately regardless of whether abuse
  is confirmed — treat "was it used" as a forensic question, not a gate on rotation. In
  parallel, scope the new credential down to only the permissions the pipeline's steps
  actually exercise, and purge or redact the offending log.
- **Long-term fix**: replace the static, long-lived key with a short-lived workload
  identity (e.g. OIDC-federated short-lived tokens issued per CI run, no persisted secret
  at all), and add secret-scanning as a hard CI gate on build logs and diffs — not an
  optional post-hoc audit, but something that fails the build before the log is ever
  persisted somewhere broadly readable.

## Prevention

The systemic lesson: **a credential's blast radius is set at grant time, not at leak
time** — "it was easier to just grant it what it needed" is exactly the reasoning that
turns a routine debug mistake into a much bigger incident, because the leak itself
(a stray `echo`, a verbose error) is nearly inevitable at scale, and the only variable
actually under your control in advance is how much damage that leak can do. See the
least-privilege and blast-radius discussion in the
[Foundations tutorial](../00_foundations/tutorial.md#the-cia-triad-and-the-vocabulary-built-on-it)
and the CI credential and secrets-management patterns in the
[Cloud Security tutorial](../02_cloud_security/tutorial.md).

## Articulate It: Interview Framing & Vocabulary

### Three Ways to Explain This

- **Blast-radius framing (the default for this scenario):** "The leak itself — a stray
  debug `echo`, a verbose error — is nearly inevitable at some point across enough CI
  pipelines. The only thing actually under your control in advance is how much damage that
  leak can do, which is set at grant time, not leak time. I'd treat 'scope this down' as
  the real fix, not 'we got unlucky.'"
- **Silent-exposure framing (good for the detection-gap angle):** "This wasn't caught by
  an alert — it was caught by a routine audit, days later. A credential sitting in a
  plaintext log is a silent failure exactly like a broken embedding match: nothing errors,
  nothing pages anyone, and the only way you find it is by deliberately looking."
- **Rotate-vs-rescope framing (good for the fix discussion):** "Rotating the credential
  fixes this specific exposure. It doesn't fix the next debug statement someone adds under
  deadline pressure — that needs a scanning gate that fails the build automatically, and a
  credential type that expires on its own even if the gate misses something."

### Vocabulary Builder

- **workload identity** (n. phrase) — a short-lived, automatically-issued credential tied
  to a specific pipeline run or service identity, as opposed to a static long-lived key;
  the long-term fix for CI credential exposure.
- **secret-scanning gate** (n. phrase) — an automated check (regex/entropy-based or
  pattern-matched) that fails a build or blocks a merge if credential-shaped content is
  detected, run as a hard gate rather than an optional audit.
- **"…the leak is nearly inevitable; the blast radius is the variable you control"** — a
  reusable framing for arguing that scoping-down matters more than trying to prevent every
  possible accidental exposure.

---

**Previous:** [Overview](README.md)  |  **Next:** [2. Indirect Prompt Injection Exfiltrated Internal Data](02_indirect_prompt_injection_exfiltration.md)
