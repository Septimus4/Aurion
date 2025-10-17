---
title: Roadmap
version: 0.1.0
date: 2025-10-14
---
# Roadmap

Aurion will evolve through a series of milestones.  Milestones are numbered M0–M5; each milestone defines deliverables and acceptance criteria.  No specific dates are assigned beyond ordering.

## M0 – Bootstrapping

*Deliverables*

- Create repository structure with documentation, plugin API (`plugin.proto`) and initial graph model.
- Provide a minimal Rust plugin skeleton that compiles to Wasm and prints a sample graph mutation.
- Implement content‑addressable store abstraction and integrate Neo4j driver.
- Provide installation instructions for Kali and set up CI workflow.

*Acceptance criteria*

- Repository layout matches this specification.
- `cargo build --workspace` succeeds on a fresh Kali installation.
- The skeleton plugin runs under Wasmtime and produces valid JSON.
- Architecture and graph model documents are available and reviewed.

## M1 – Orchestrator and passive pipeline

*Deliverables*

- Implement `aurion-orch` with gRPC endpoints to register plugins and execute jobs.
- Implement plugin runner for Wasm modules with caching and deterministic keys.
- Develop a CLI client to submit jobs and display events.
- Add built‑in plugins for `nmap` (passive mode), `amass`, `subfinder` and `httpx`.
- Add golden fixture tests for each built‑in plugin using sample outputs from these tools.

*Acceptance criteria*

- Passive enumeration jobs run end‑to‑end: CLI → orchestrator → plugin → graph.
- Caching prevents duplicate runs when inputs are identical.
- Golden tests compare generated graph mutations against fixtures.
- M0 acceptance criteria continue to pass.

## M2 – Collaboration server and IntelliJ plugin prototype

*Deliverables*

* Implement `aurion-server` with workspace management, authentication and WebSocket event streaming.
* Develop an IntelliJ plugin (`aurion-plugin`) that provides graph exploration, right‑click actions and passive run triggering inside IntelliJ IDEA Community Edition.  Use JetBrains JCEF WebView to host the graph explorer and implement runbook editing with standard Swing components.  Provide Gradle tasks to build and package the plugin【410764021097615†L35-L47】.
* Implement a native Rust UI module that exposes high‑performance graph layout functions via FFI to the plugin.  Integrate the module into the plugin and handle message passing.
* Add CRDT‑based conflict resolution for concurrent edits in the graph.
* Integrate CAS browsing and evidence download.

*Acceptance criteria*

* Multiple users can connect to a shared workspace and see live updates.
* The IntelliJ plugin shows hosts, services and evidence; right‑click actions trigger passive runs through the orchestrator.
* Conflict resolution handles concurrent edits without data loss.
* The plugin can be built and installed via Gradle, and the native module loads correctly on Kali Linux.

## M3 – Active capabilities and policy engine

*Deliverables*

- Implement the policy engine enforcing `AuthorizationBundle` evaluation and rate limits.
- Add built‑in plugins for `masscan`, `nuclei` and `ffuf` with gating.
- Implement Wasm network hostcalls and container fallback runner.
- Add support for uploading and managing authorisation bundles through the UI.

*Acceptance criteria*

- Active jobs are blocked without a valid bundle and permitted when authorised.
- High‑risk plugins function through Wasm or container sandbox with network restrictions.
- Policy violations are logged and visible to administrators.

## M4 – Reporting and automation

*Deliverables*

- Implement reporting pipeline using Tera templates with export to Markdown and HTML.
- Add runbook builder UI allowing users to chain plugin executions and generate reports automatically.
- Implement scheduled runs and notifications via email or Slack.
- Integrate vulnerability feeds (e.g., NVD) into the graph and finding model.

*Acceptance criteria*

- Reports can be generated from the UI and include executive summaries and detailed findings.
- Runbooks support at least three sequential steps and produce deterministic results.
- Scheduled jobs run at configured times and notify users on completion.

## M5 – Hardening and release candidate

*Deliverables*

- Conduct a full security review using the provided checklist and address findings.
- Add telemetry opt‑in support with anonymised usage statistics and documented privacy guarantees.
- Finalise plugin registry design and publish a signing policy.
- Provide full documentation and API reference.
- Tag a `v1.0.0` release candidate and open a public beta.

*Acceptance criteria*

- All security issues from the review are resolved or accepted with mitigations.
- Telemetry is disabled by default and opt‑in behaviour is documented.
- Plugin registry supports signed modules and public key verification.
- Documentation covers architecture, API, plugin development, usage and policies.
- The release candidate passes all tests and is installable on fresh Kali systems.

## M6 – AI integration

*Deliverables*

- **`aurion-ai` MCP server:** Implement a dedicated Rust service that implements the Model Context Protocol.  It exposes Aurion’s actions for summarisation, classification and runbook generation.  The server communicates with the orchestrator, graph DB and CAS and runs local AI models or calls external LLM endpoints.
- **IDE integration:** Extend the IntelliJ plugin to include an AI panel where users can ask natural language questions about the current engagement.  The plugin uses an MCP client to send JSON‑RPC requests to the AI server and displays the responses.
- **Native AI models:** Package lightweight, permissively licensed models for summarisation and classification as optional Rust crates.  Provide configuration (`ai.toml`) to enable or disable models or to configure external provider endpoints.
- **Agent SDK and example:** Provide a small SDK and an example script showing how developers can implement an MCP client that interacts with `aurion-ai` to obtain graph summaries.  Document this process in a new `agent.md`.
- **Tests and CI:** Add test cases exercising AI summarisation and runbook generation using sample fixture data.  Update CI to build `aurion-ai`, run its smoke tests, and include AI tests in the matrix.

*Acceptance criteria*

- The `aurion-ai` server builds on a fresh Kali system, starts with default configuration and responds to MCP requests for summarisation and runbook generation.
- The IntelliJ plugin displays an AI panel where users can ask questions and receive summarised findings.  AI actions respect authorisation policies and do not trigger active jobs without a valid `AuthorizationBundle`.
- External developers can implement an MCP client using the provided SDK to interact with Aurion and obtain graph summaries.  The example agent runs and produces correct results.
- All AI tests pass in CI and do not degrade performance of other milestones.

## Acceptance Criteria

This roadmap defines milestones M0–M6 with deliverables and acceptance criteria.  It avoids binding calendar dates and lists acceptance tests for M0 and M1 in detail.  Later milestones are outlined with clear goals and criteria.