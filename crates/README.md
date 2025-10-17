# crates/

This directory contains Rust crates that make up Aurion. Each component is a separate crate to keep responsibilities clear and to make splitting into separate repositories straightforward later.

Current crates:
- `aurion-core` – primary binary used during development
- `aurion-orch` – orchestrator service
- `aurion-server` – collaboration server
- `aurion-ai` – AI / MCP server
- `aurion-native` – native shared library for the IntelliJ plugin
- `aurion-runner` – plugin runner (Wasm / OCI)

Adding a new crate:
1. Create `crates/<name>/Cargo.toml` and `crates/<name>/src/lib.rs` or `src/main.rs`.
2. Add the relative path under the workspace `members` in the top-level `Cargo.toml`.
3. Run `cargo build --workspace` to validate.

Splitting out later:
- Each crate is intentionally self-contained; when ready, extract the crate directory into a new repository and update CI to publish its crate or binary.

