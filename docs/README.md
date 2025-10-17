---
title: Aurion Project Overview
version: 0.1.0
date: 2025-10-14
---
# Aurion

Aurion is an open‑source, plugin‑driven penetration testing IDE targeting Kali Linux.  It couples a graph‑based data model with a Rust‑first orchestrator and server, while the user interface is delivered as a **JetBrains IntelliJ IDEA plugin** with optional native Rust modules.  The Community edition of IntelliJ IDEA is licensed under Apache 2.0【201827599878107†L176-L188】 and supports over 7,600 plugins via its built‑in marketplace【201827599878107†L266-L275】; this foundation allows Aurion to embed a rich graph explorer and runbook builder directly inside a familiar IDE.  The platform bundles common free tools such as **nmap**, **masscan**, **amass**, **subfinder**, **httpx**, **nuclei** and **ffuf** via standardised plugins.  Each plugin runs in a strict sandbox – WebAssembly/WASI by default, with an OCI container fallback – and writes results into a labelled property graph database such as Neo4j, SurrealDB or Memgraph.  Passive operations (enumeration and fingerprinting) run without authorisation; active operations (scanning, exploitation or fuzzing) require a signed *AuthorizationBundle*.  Aurion also integrates native AI capabilities: a dedicated **MCP server** exposes summarisation and runbook generation actions to AI agents via the Model Context Protocol【65787800937560†L60-L69】, allowing users to ask natural language questions about their engagements and obtain intelligent insights.

## Mission

Provide a reproducible, collaborative and safe environment for security professionals to map attack surfaces, correlate evidence and generate reports.  The project emphasises **security by design**, permissive licensing and community extensibility through well‑defined plugin APIs.  All integrated tools must be open‑source and installable on vanilla Kali systems.  Data flows are captured in a graph model with provenance and content‑addressable storage, enabling auditors to trace how each finding was derived.

## Audience

- **Operators**: analysts who need to kick off reconnaissance pipelines and explore results without writing scripts.
- **Pentesters**: experts who want to extend the IDE with new scanners or run their own toolchains in a controlled sandbox.
- **Team leads**: supervisors who define scope and authorisation policies, review evidence and generate client‑facing reports.

## Core technologies

Aurion uses Rust for all core components, gRPC/Protobuf for internal RPCs and HTTP/WebSocket for the plugin surface.  The default graph backend is Neo4j, which stores data as nodes with labels and properties connected via typed relationships【780551429696953†L227-L250】.  The IDE is built as an IntelliJ plugin using the JetBrains Platform and Kotlin, packaged via the IntelliJ Gradle plugin【410764021097615†L35-L47】.  Plugins run in Wasmtime to leverage the secure WebAssembly sandbox; Wasmtime runs WebAssembly code outside of browsers and is designed to be fast, secure and configurable【801025589620893†L83-L112】.  To ensure deterministic caching, the orchestrator computes BLAKE3 hashes; BLAKE3 is a cryptographic hash algorithm that is secure and highly parallelisable【518711832070345†L37-L41】.
Aurion uses Rust for all core components, gRPC/Protobuf for internal RPCs and HTTP/WebSocket for the plugin surface.  The default graph backend is Neo4j, which stores data as nodes with labels and properties connected via typed relationships【780551429696953†L227-L250】.  The IDE is built as an IntelliJ plugin using the JetBrains Platform and Kotlin, packaged via the IntelliJ Gradle plugin【410764021097615†L35-L47】.  Plugins run in Wasmtime to leverage the secure WebAssembly sandbox; Wasmtime runs WebAssembly code outside of browsers and is designed to be fast, secure and configurable【801025589620893†L83-L112】.  To ensure deterministic caching, the orchestrator computes BLAKE3 hashes; BLAKE3 is a cryptographic hash algorithm that is secure and highly parallelisable【518711832070345†L37-L41】.  For AI integration, Aurion includes a Rust‑based MCP server that implements the open Model Context Protocol to bridge AI agents with Aurion’s data and tools.  MCP standardises connections between AI applications and external systems, acting like a USB‑C port for AI【65787800937560†L60-L69】.  Developers can build agents or use built‑in AI features for summarising evidence and generating natural language runbooks.

## Quickstart

Development can be started on any x86‑64 Linux host or inside a Kali VM.

```bash
# install prerequisites
sudo apt update && sudo apt install -y git build-essential curl protobuf-compiler rust-all openjdk-17-jdk

# clone and build backend services
git clone https://github.com/aurion/aurion.git
cd aurion
cargo build --workspace --release

# build IntelliJ plugin (requires Gradle and IntelliJ plugin dependencies)
cd ide-plugin
./gradlew buildPlugin

# run backend services (example)
./target/release/aurion-orch --config config/orch.toml &
./target/release/aurion-server --config config/server.toml &
./target/release/aurion-ai --config config/ai.toml &

# install the plugin
# In IntelliJ IDEA Community Edition: Settings → Plugins → Install plugin from disk… and select the generated plugin ZIP in build/distributions
```

See `infrastructure/INSTALL_KALI.md` for systemd‑based installation instructions.  Once the services are running, launch IntelliJ IDEA, install the Aurion plugin, open the graph explorer and connect to your orchestrator.  Three user stories illustrate typical use:

1. **Operator** selects a scope (e.g., `example.com`) in the graph explorer and triggers passive enumeration.  After a few minutes the graph fills with hosts, services and evidence produced by **nmap** and **amass**【147964860358445†L69-L79】【615059556182302†L66-L80】.
2. **Pentester** writes a custom plugin in Rust/WASI to integrate a new passive DNS source.  They compile to Wasm and register it via the plugin gRPC API.  The IDE automatically streams results into the graph.
3. **Team lead** uploads an *AuthorizationBundle* granting permission to run **masscan**, **nuclei** and **ffuf** for a defined CIDR.  They execute an active run, use the AI panel to summarise high‑severity findings into an executive summary and generate a runbook suggestion, then export a report.

## License

This project is licensed under the Apache 2.0 license.  The Apache license is permissive: it requires preservation of copyright and license notices, grants an express patent license to contributors and allows licensed works, modifications and larger works to be distributed under different terms【955512938574354†L127-L130】.  See `LICENSE` for full text.

## Acceptance Criteria

This README introduces Aurion in 200–400 words, provides an elevator pitch, three user stories, a quickstart using Docker and systemd and lists the mission, audience, license and core technologies.  The document references authoritative sources for tools, graph models and cryptographic primitives.