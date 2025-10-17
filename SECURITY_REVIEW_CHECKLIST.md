---
title: Security Review Checklist
version: 0.1.0
date: 2025-10-14
---
# Security Review Checklist

This checklist guides security reviewers and red teamers when assessing the Aurion platform.  It enumerates areas that require scrutiny and defines boundaries for ethical testing.  Reviewers should document findings and recommendations for each item.

## Core components

- [ ] **Rust code safety**: Ensure all unsafe blocks are justified and minimal.  Verify use of `#[deny(unsafe_code)]` where possible.  Check for integer overflows, use-after-free or data races.
- [ ] **Concurrency**: Inspect `tokio` usage for deadlocks and resource starvation.  Confirm that channels and event loops cannot be flooded.
- [ ] **Graph database interactions**: Confirm that Cypher queries are parameterised to prevent injection.  Validate input sanitisation before writing to the graph.
- [ ] **Content Addressable Store (CAS)**: Review digest computation (BLAKE3) and integrity verification.  Ensure immutable data cannot be tampered with or replaced.
- [ ] **Configuration and secrets**: Check that configuration files have restrictive file permissions, secrets are loaded from the OS keyring or environment and never written to logs or the graph.

## Plugin sandbox

- [ ] **Wasm sandbox**: Verify that Wasmtime is configured with a minimal WASI, memory and time limits.  Confirm that network access is disabled by default and only enabled when policies allow.
- [ ] **OCI container fallback**: Inspect the seccomp and AppArmor profiles.  Ensure the container runs as a non-root UID, the filesystem is read-only and network namespaces restrict connectivity.  Confirm that volumes are not mounted from the host except `/tmp`.
- [ ] **Plugin signatures**: Confirm that the plugin registry verifies Ed25519 signatures on Wasm binaries and container images.  Validate that untrusted plugins cannot be loaded.

## Policy enforcement

- [ ] **AuthorizationBundle validation**: Assess the cryptographic verification of bundles (signature and expiry).  Ensure the policy engine correctly enforces scope intersection and capability flags.  Test denial cases and error handling.
- [ ] **Rate limiting**: Evaluate whether per-engagement and per-plugin limits effectively prevent abuse.  Attempt to bypass limits by concurrent submissions.
- [ ] **Emergency kill-switch**: Confirm that toggling the kill-switch terminates all active jobs and prevents new runs.  Verify that only administrators can clear the flag.

## Networking and communication

- [ ] **TLS/mTLS**: Ensure all gRPC and WebSocket communications are encrypted.  Verify correct certificate verification and certificate revocation handling.  Inspect the trust store management and rotation procedures.
- [ ] **Session management**: Check that session tokens or certificates are scoped per workspace and invalidated on logout.  Confirm that session reuse between users is impossible.
- [ ] **Event streaming**: Test for message injection or replay attacks on the event bus and collaboration server.

## Reporting and evidence

- [ ] **Finding model integrity**: Validate that evidence and findings recorded in the graph cannot be modified after creation.  Inspect upsert logic for idempotence and provenance fields.
- [ ] **Report generation**: Ensure that Tera templates escape user-controlled data.  Check that exported reports exclude secrets and internal host information.

## Third-party tools

- [ ] **Tool binaries**: Verify that apt packages or downloaded binaries match expected checksums.  Check for outdated or vulnerable tool versions.
- [ ] **Tool execution**: Confirm that command line arguments passed to tools are sanitised and do not allow command injection.  Inspect how output is parsed and normalised.

## Red team engagement rules

Aurion is designed for authorised penetration testing within a defined scope.  Red teamers must respect the following rules:

- Do not execute plugins or tools against targets outside of the approved scope contained in the *AuthorizationBundle*.
- Do not attempt to bypass policy enforcement, sandbox restrictions or signature verification.  Such attempts should be reported as findings.
- Do not author or run exploit payloads through the platform; testing should focus on orchestration, metadata capture and safe operation of open‑source tools.
- Coordinate with project maintainers when testing high‑risk features or denial‑of‑service scenarios to avoid service disruption.

## Acceptance Criteria

The checklist provides clearly delineated items for reviewers to verify, including code safety, concurrency, graph database interaction, sandbox configuration, signature verification, policy enforcement, networking, reporting and tool execution.  It also specifies ethical rules for red team engagement.  Each item is phrased as a checkbox to allow tracking of completion.