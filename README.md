# Aurion (monorepo workspace)

This repository is organized as a Cargo workspace and a multi-component monorepo for early-stage development. The goal is to keep everything in one place during development and split into separate repositories later as components stabilise.

Note about root cleanup

- The repository contains `crates/` workspace layout:
  - Source code lives under `crates/` (for example, `crates/aurion-core/src/main.rs`).
  - The `target/` build directory is not tracked and is ignored by Git.

Quick start

- Build the entire workspace:

  cargo build --workspace

- Run the main binary (development placeholder):

  cargo run -p aurion-core

Layout

- `crates/` - Rust components (binaries & libraries). See `crates/README.md` for details.
- `clients/` - Client-side code (JetBrains plugin, web UI, etc.).
- `plugins/` - Plugin sources (Wasm, OCI, or other plugin formats).
- `native/` - Native modules and FFI bindings.
- `ai/` - AI/MCP server prototypes and AI-related tooling.
- `mcp/` - MCP protocol helpers, examples and adapters.
- `docs/`, `examples/`, `infrastructure/` - documentation and deployment aids.

Repository management notes

- Keep binaries small and focus on clear crate boundaries. Use the Cargo workspace (`Cargo.toml`) to manage shared dependencies.
- When ready to split, each top-level area (`crates/aurion-ai`, `clients/jetbrains-plugin`, `plugins/<name>`, etc.) is intentionally isolated to simplify extraction.
- Use `crates/` for Rust-first components and put language-specific code in their respective top-level folders.

Contributing

Please follow the guidelines in `CONTRIBUTING.md` and the `docs/` folder for design and architecture decisions.
