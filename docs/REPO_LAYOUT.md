# Repository layout and cleanup notes

This file documents the current monorepo layout and the cleanup steps taken to remove legacy top-level artifacts in favor of a clean Cargo workspace.

Repository layout

- `Cargo.toml` (workspace manifest)
- `crates/` — Rust crates (each crate is self-contained and can be split out later)
  - `aurion-core`, `aurion-orch`, `aurion-server`, `aurion-ai`, `aurion-native`, `aurion-runner`
- `clients/` — client code (JetBrains plugin, web UI, etc.)
- `plugins/` — plugin sources
- `native/` — native code and build scripts
- `docs/`, `examples/`, `infrastructure/` — documentation and deployment aids

Cleanup summary

- The previous top-level `src/` (single-binary layout) has been removed. All Rust code now lives under `crates/<crate-name>/src/`.
- The `target/` build output directory at the repository root has been removed and is ignored by Git going forward.

Workspace usage

- Build everything: `cargo build --workspace`
- Run a crate: `cargo run -p aurion-core` or `cargo run -p aurion-server`

Splitting repos later

- Each `crates/<name>` and top-level `clients/<name>` or `plugins/<name>` is structured to be extracted into its own repository when ready.
- Keep public APIs and shared types in a small `crates/common` crate if you anticipate splitting components soon.
