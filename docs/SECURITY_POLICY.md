---
title: Security Policy and Guardrails
version: 0.1.0
date: 2025-10-14
---
# Security Policy

Aurion enforces strict guardrails to ensure that penetration tests remain within the authorised scope and that no unintended harm occurs.  This document defines the *AuthorizationBundle*, scope verification rules, rate‑limits, sandboxing requirements, secrets handling and audit logging.

## AuthorizationBundle

Active and high‑risk capabilities are gated by an *AuthorizationBundle*.  The bundle is a JSON document signed by an authorised approver.  Its structure is illustrated below:

```json
{
  "engagement_id": "ENG-2025-001",
  "issued_by": "teamlead@example.com",
  "issued_at": "2025-10-14T09:00:00Z",
  "expires_at": "2025-10-14T18:00:00Z",
  "scope": ["example.com", "192.0.2.0/24"],
  "capabilities": {
    "PortScan": true,
    "VulnerabilityScan": true,
    "Fuzz": false
  },
  "signature": "BASE64_ED25519_SIGNATURE"
}
```

### Signing

The bundle is hashed (SHA‑256) and signed using an Ed25519 private key belonging to the approver.  The orchestrator verifies the signature against a list of trusted public keys.  Bundles are stored in the graph as `Authorization` nodes linked to the engagement.

## Scope verification

When a job is submitted with a bundle, the orchestrator evaluates the request against the scope and capability flags.  Pseudocode for evaluation:

```rust
fn evaluate(request: &JobRequest, bundle: &AuthBundle) -> Result<(), Denial> {
    // check expiry
    if now() > bundle.expires_at { return Err(Denial::Expired); }
    // check capability
    if !bundle.capabilities.get(&request.capability).unwrap_or(&false) {
        return Err(Denial::CapabilityNotAllowed);
    }
    // check target within scope
    if !is_within_scope(&request.target, &bundle.scope) {
        return Err(Denial::OutOfScope);
    }
    Ok(())
}
```

Denied requests return a structured error to the client, and nothing is executed.  Emergency revocation is supported by marking the `Authorization` node as revoked; the orchestrator refuses any future job referencing it.

## Rate‑limits and quotas

The orchestrator enforces per‑engagement and per‑plugin rate limits.  Defaults are:

- Passive enumeration: 4 concurrent jobs per scope.
- Port scans: 1 concurrent job; throttle to 100 K packets per second.
- Vulnerability scans and fuzzing: 1 job at a time.

Administrators may adjust these limits in the configuration file.

## Sandboxing rules

Plugins run in isolation.  For WebAssembly modules the following restrictions apply:

- WASI API access is limited to stdin, stdout, stderr and a temporary directory.
- No network access unless the orchestrator injects a network capability based on the job.
- Memory usage limited to 64 MiB; execution time limited to 5 minutes per run.

For OCI containers:

- The container runs as a non‑root user.
- The filesystem is read‑only except for `/tmp`.
- A seccomp profile blocks dangerous syscalls; an AppArmor profile further restricts capabilities.
- Network namespaces disable outbound connections unless explicitly allowed.

## Secrets handling

Aurion avoids storing sensitive credentials in clear text.  API keys for third‑party services are injected into plugins via environment variables from a secrets manager.  Secrets are encrypted at rest using the host OS keyring.  Plugins must read secrets from environment variables; they are never persisted into the graph or logs.

## Audit logging

Every RPC call, policy decision and plugin execution is logged.  The log record includes the user ID, timestamp, job parameters, bundle ID, decision result and exit code.  Logs are written to an append‑only file and optionally forwarded to a syslog server.  Audit logs must be retained for at least 90 days.

## Emergency kill‑switch

Administrators can trigger a kill‑switch that disables all active jobs and prevents new submissions.  The kill‑switch sets a flag in the orchestrator configuration; the orchestrator terminates running plugins, clears the event queue and closes listening ports.  Once enabled, only an administrator can clear the flag.

## Acceptance Criteria

This security policy document defines the structure and signing of an AuthorizationBundle, provides pseudocode for scope evaluation, specifies rate limits, sandbox rules for Wasm and containers, describes secrets handling and audit logging, and defines an emergency kill‑switch.  The policy clearly distinguishes between passive and active operations and references cryptographic primitives used elsewhere.