---
title: Test Plan
version: 0.1.0
date: 2025-10-14
---
# Test Plan

Aurion uses a combination of unit tests, integration tests, golden file comparisons and chaos tests to verify correctness and robustness.  This plan outlines the test strategy, fixtures and regeneration procedures.

## Golden file tests

Golden tests validate that plugin outputs and graph mutations are stable over time.  For each built-in plugin the repository contains pre-recorded output files under `tests/golden/<tool>/<fixture>.json`.  The orchestrator runs the plugin against a known target and compares the resulting JSON mutation against the golden file.

Fixtures to include:

| Tool | Fixture name | Description |
|------|--------------|-------------|
| `nmap` | `nmap_localhost.xml` | XML output from running `nmap -sV -Pn 127.0.0.1` on a pristine Kali VM. |
| `amass` | `amass_example.com.json` | JSON output from `amass enum -d example.com -json` with only passive sources. |
| `subfinder` | `subfinder_example.com.json` | JSON output for `example.com` with passive sources. |
| `httpx` | `httpx_example.com.json` | JSONL output from probing `example.com` with default flags. |
| `masscan` | `masscan_localhost.txt` | Text output from `masscan -p80 127.0.0.1/32` limited to a single host. |
| `nuclei` | `nuclei_testsite.json` | JSON output from running a single template against a local HTTP server. |
| `ffuf` | `ffuf_testsite.json` | JSON output from fuzzing a test HTTP server with a small wordlist. |
| `httpx` | `httpx_ipv6.json` | IPv6 probe output to cover dual-stack handling. |
| `masscan` | `masscan_cidr.txt` | Output scanning a small CIDR range `192.0.2.0/30`. |
| `nmap` | `nmap_service.json` | Normalised JSON representation of an `nmap` run with version detection. |

### Regeneration

To regenerate a golden fixture, spin up a pristine Kali VM (matching the supported version), install the tool using the documented apt command and run the same command used to create the original fixture.  Capture the raw output and place it under `tests/golden/<tool>/`.  Then run `cargo test --features=regenerate` to update the expected JSON mutation.  Version bumps must be recorded in `TOOL_INVENTORY.md`.

## Resolver tests

Unit tests for the ambient context resolver ensure that given a selection of nodes, the resolver returns the correct context object.  Tests cover single host selection, service selection, mixed selection and TTL expiry.

## Policy tests

Policy tests verify that the orchestrator correctly enforces the security policy.  Scenarios include:

- Expired `AuthorizationBundle` is rejected.
- Bundle lacking required capability denies execution.
- Target outside the authorised scope is denied.
- Valid bundle allows execution.

## Chaos tests

Chaos testing helps ensure resilience.  Tests include:

- Simulating plugin crashes (e.g., divide-by-zero) to verify orchestrator recovery.
- Injecting network latency and packet loss between orchestrator and runner.
- Forcing CAS corruption and verifying that the system detects and isolates bad entries.
- Restarting the graph database while jobs are running.

## CI test matrix

Continuous integration runs the following matrix of environments:

- **OS**: Ubuntu LTS (GitHub runner), Kali latest, Debian stable.
- **Graph backend**: Neo4j 5.x, SurrealDB 1.x.
- **Plugin language**: Rust (WASI), Go (compiled to Wasm using TinyGo), container fallback.

Each combination runs unit tests, integration tests and golden file comparisons.  The CI also runs `cargo clippy` and `cargo fmt` checks.

## Acceptance Criteria

This test plan describes the test strategy covering golden fixtures, resolver and policy tests, chaos tests and the CI matrix.  It lists at least ten golden fixtures spanning three tool types and outlines regeneration steps.  It specifies policy scenarios and environment combinations to be tested.