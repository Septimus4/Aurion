---
title: Plugin Model and API
version: 0.1.0
date: 2025-10-14
---
# Plugin Model

Plugins extend Aurion by encapsulating external tools or logic behind a uniform RPC interface.  Each plugin must declare its name, semantic version, supported capabilities and risk class.  Plugins are distributed as signed WebAssembly modules compiled for `wasm32-wasi`; when Wasm is not available, an OCI container image may be used.  The orchestrator enforces sandboxing, caching, and capability gating.

## Protobuf interfaces

The canonical RPC definitions live in `specs/plugin.proto`.  A shortened excerpt illustrates the core messages and service:

```protobuf
syntax = "proto3";
package aurion.plugin.v1;

message PluginInfo {
  string name = 1;
  string version = 2;
  repeated string capabilities = 3;
  string description = 4;
}

message RegisterRequest {
  PluginInfo info = 1;
  repeated string supported_scopes = 2;
}

message RegisterResponse {
  bool accepted = 1;
  string message = 2;
}

message JobRequest {
  string job_id = 1;
  string plugin_name = 2;
  string version = 3;
  string target = 4;
  string serialized_config = 5;
  string scope_id = 6;
  string authorization_bundle = 7;
}

message JobResult {
  string job_id = 1;
  bool success = 2;
  bytes output_json = 3;
  string cache_key = 4;
}

service PluginService {
  rpc RegisterPlugin (RegisterRequest) returns (RegisterResponse);
  rpc ExecuteJob (JobRequest) returns (JobResult);
}
```

Implementations may extend this schema but must remain backward compatible.  Plugins register themselves at startup; the orchestrator maintains a registry of available plugins and their capabilities.

## Rust trait example

The SDK exposes a trait that plugin authors implement.  A minimal trait looks like this (simplified for clarity):

```rust
use semver::Version;
use serde::{Deserialize, Serialize};

/// Capabilities describe what a plugin does and its risk class.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Capability {
    PassiveEnum,
    PortScan,
    VulnerabilityScan,
    Fuzz,
}

pub struct JobRequest {
    pub job_id: String,
    pub target: String,
    pub config: serde_json::Value,
    pub scope_id: String,
}

pub struct JobResult {
    pub job_id: String,
    pub success: bool,
    pub output_json: serde_json::Value,
    pub cache_key: String,
}

pub trait Plugin {
    fn name(&self) -> &'static str;
    fn version(&self) -> Version;
    fn capabilities(&self) -> Vec<Capability>;
    fn run(&mut self, req: JobRequest) -> Result<JobResult, String>;
}
```

The orchestrator provides an SDK that maps the Protobuf messages to this trait.  A plugin compiled for `wasm32-wasi` exposes a `run()` function as a C ABI symbol.  When compiled as a container, the process must read the request from stdin and write the result JSON to stdout.

## Capability taxonomy and gating

Capabilities fall into risk classes:

| Capability | Description | Risk class | Requires AuthorizationBundle |
|-----------|-------------|-----------|--------------------------------|
| `PassiveEnum` | Queries open sources, DNS and certificate transparency logs. | Low | No |
| `PortScan` | Sends TCP/UDP packets to identify listening services. | Medium | Yes |
| `VulnerabilityScan` | Executes signatures (e.g., nuclei templates) or exploit checks. | High | Yes |
| `Fuzz` | Performs directory or parameter fuzzing. | High | Yes |

The orchestrator denies execution of high-risk capabilities unless a valid *AuthorizationBundle* accompanies the request.  Risk classes may be overridden only by administrators.

## Deterministic cache key

To avoid redundant work, the orchestrator caches results.  A plugin must return a `cache_key` field in its `JobResult`.  The key is computed deterministically using the BLAKE3 hash of canonicalised inputs.  The algorithm is:

1. Concatenate the plugin name and semantic version.
2. Serialise the job configuration (`serialized_config`) and scope ID into canonical JSON with sorted keys.
3. Compute `blake3::hash()` on the UTF‑8 bytes of the concatenated string.
4. Return the hex‑encoded digest as the cache key.

Example in Rust:

```rust
use blake3::Hasher;
use serde_json::to_string;

fn compute_cache_key(name: &str, version: &str, config: &serde_json::Value, scope: &str) -> String {
    let mut hasher = Hasher::new();
    hasher.update(name.as_bytes());
    hasher.update(version.as_bytes());
    let config_json = to_string(config).expect("canonicalise config");
    hasher.update(config_json.as_bytes());
    hasher.update(scope.as_bytes());
    let digest = hasher.finalize();
    digest.to_hex().to_string()
}
```

Cache entries include a Time‑To‑Live (TTL); defaults are 24 hours for passive jobs and 6 hours for active jobs.  The orchestrator may evict entries earlier to enforce retention budgets.

## Wasm contract and container fallback

Plugins must be compiled for the WASI target (`wasm32-wasi`) and should avoid non‑deterministic system calls.  They have no network access by default; any network hostcall must be explicitly enabled by the orchestrator based on the capability.  Memory and CPU usage are limited; by default the Wasmtime runtime allocates 64 MiB per instance and enforces a 5 minute execution timeout.  When a plugin cannot be built as Wasm (e.g., tool requires native binaries), authors may provide a minimal OCI image.  The container must run as a non‑root user, mount a read‑only filesystem, use a seccomp profile and AppArmor and declare all environment variables it requires.  Network access is disabled unless the capability allows it.

## Signing and verification

Plugins are distributed through a signed registry.  Each plugin release includes:

- `manifest.json` describing the plugin name, version, capabilities and risk class.
- `module.wasm` or container image digest.
- `signature.ed25519` containing an Ed25519 signature of the SHA256 digest of the module.

Developers sign their modules with their private key.  The orchestrator verifies signatures against trusted public keys before loading a plugin.  Fallback verification using OpenPGP is permitted for container images.  Unsigned modules are rejected.  This model protects against tampering and ensures that only trusted authors can publish active capabilities.

## Acceptance Criteria

This document defines the plugin RPC interface, illustrates a Rust trait for plugin authors, lists capability classes and gating rules, details the deterministic caching algorithm using BLAKE3【518711832070345†L37-L41】, outlines sandboxing rules for Wasm and containers and describes signing and verification requirements.  It includes at least one code snippet and a table of capabilities.