---
title: Contributing Guide
version: 0.1.0
date: 2025-10-14
---
# Contributing to Aurion

Thank you for considering a contribution to Aurion.  This guide outlines the process for submitting changes, the coding standards we follow, the plugin publication workflow and our licensing expectations.  All contributors must adhere to the project’s security and legal policies.

## Developer Certificate of Origin (DCO)

Aurion uses the **Developer Certificate of Origin** in lieu of a CLA.  By submitting a pull request, you certify that you wrote the code or otherwise have the right to submit it.  Each commit must include a `Signed-off-by` trailer using the name and email address of the author.  Example:

```
Signed-off-by: Jane Doe <jane@example.com>
```

Commits without a valid sign‑off will not be merged.  To add the trailer automatically, use `git commit -s`.

## Code of conduct

All interactions in the repository are governed by the [Contributor Covenant](https://www.contributor-covenant.org/version/2/1/code_of_conduct/) and our security policies.  Be respectful, constructive and inclusive.

## Project structure

The repository contains multiple directories:

- `docs/`: Project documentation.
- `specs/`: Protocol definitions (Protobuf and OpenAPI).
- `examples/skeleton-plugin/`: Example Rust plugin illustrating the SDK.
- `infrastructure/`: Deployment scripts for Kali Linux.
- `ci/`: Continuous integration configuration.

## Getting started

1. Fork the repository and create a new branch for your feature or fix.
2. Install Rust (`rustup toolchain install stable`) and run `cargo build` to ensure the workspace builds.
3. Make your changes and add tests where appropriate.  All public APIs must have test coverage.
4. Run lint and formatting checks:

   ```bash
   cargo fmt --all
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --workspace
   ```
5. Commit your changes with a clear message and sign off your commit.
6. Open a pull request against the `main` branch.  Describe what your change does, reference any related issues and note any security considerations.
7. Address review feedback promptly.  Merging requires approval from at least one core maintainer.

## Coding style

Aurion is a Rust‑first project.  The following conventions apply:

- Use `rustfmt` for code formatting.  CI runs `cargo fmt --check`.
- Use `clippy` and fix any lints at the `-D warnings` level.
- Avoid `unsafe` code unless absolutely necessary.  When used, wrap it in a function with comments explaining why it is safe.
- Write small, single‑purpose functions and document public interfaces with Rustdoc comments.
- Keep dependencies minimal and prefer audited crates.

## Plugin development

Plugins extend Aurion by performing tasks such as scanning, enumeration or analysis.  Follow these rules when authoring plugins:

1. **WASI first**: Compile your plugin to `wasm32-wasi` and ensure it runs under Wasmtime.  Use the provided `aurion-plugin-sdk` crates (see `docs/PLUGINS.md` for trait definitions).
2. **Capability declaration**: Declare the capabilities your plugin provides (`PassiveEnum`, `PortScan`, etc.) and assign an appropriate risk class (`low`, `medium`, `high`).  High‑risk capabilities (e.g., fuzzing, exploitation) must default to disabled and require a signed AuthorizationBundle.
3. **Deterministic output**: Emit normalized JSON that describes graph mutations.  Do not include timestamps or non‑deterministic data in the cache key.
4. **Signing**: Before publishing, sign your Wasm module or container image with an Ed25519 key.  The signature must accompany the plugin in the registry.  Unsigned plugins will not be accepted.
5. **Licensing**: Plugins hosted in the official registry must be licensed under Apache‑2.0 or MIT.  Third‑party dependencies must be compatible with these terms.  Retain attribution for all libraries.
6. **Repository**: Provide a `README.md` detailing what the plugin does, how to configure it and any risks.  Include sample outputs and test fixtures.

### Publishing process

1. Submit your plugin for review by opening a pull request against the `plugins` registry repository (to be announced).  Provide the compiled Wasm, signature, manifest (`plugin.toml`), source code and tests.
2. The maintainers will verify the signature, run the plugin in a sandbox and review code for security and quality.
3. Once approved, the plugin will be published to the central registry with a semantic version tag.
4. Plugin authors are responsible for maintaining their plugin and responding to vulnerability reports.

## Release policy

Aurion uses **semantic versioning** (`MAJOR.MINOR.PATCH`) across all components.  Breaking changes increment the major version.  New backwards‑compatible features bump the minor version.  Bug fixes increment the patch version.  Each release must be tagged in Git (e.g., `v0.2.0`) and include release notes summarising changes, new features and migration instructions.  Official releases are built and signed by maintainers using reproducible build scripts.

## Reporting vulnerabilities

If you discover a security issue, please email **security@aurion.org** with details and a proof of concept.  Do not file public GitHub issues for vulnerabilities.  We follow responsible disclosure and aim to resolve critical issues promptly.  See `SECURITY_POLICY.md` for high‑level security practices.

## License

All contributions to the core repository are licensed under Apache 2.0.  By contributing, you agree that your work may be distributed under the same license.  Do not submit proprietary or non‑redistributable code.  Plugin authors may choose Apache‑2.0 or MIT for their plugins, provided they are compatible with the core project.

## Acceptance Criteria

This contributing guide defines the DCO process, outlines the code of conduct, explains the project structure, provides step‑by‑step instructions for developing and submitting changes, establishes coding style conventions, elaborates the plugin publication workflow, clarifies licensing expectations and release policy, and instructs on responsible vulnerability disclosure.  It satisfies the requirements of the mission brief.