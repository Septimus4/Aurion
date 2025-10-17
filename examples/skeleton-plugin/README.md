---
title: Skeleton Plugin
version: 0.1.0
date: 2025-10-14
---
# Skeleton Plugin

This directory contains a minimal Aurion plugin implemented in Rust.  It is intended as a starting point for authors wishing to develop new plugins.  The plugin prints a single normalized graph mutation to stdout when invoked.

## Building

First install Rust and Wasmtime on your development machine:

```bash
sudo apt update && sudo apt install -y rust-all wasmtime
rustup target add wasm32-wasi
```

Then build the plugin for the WASI target:

```bash
cd examples/skeleton-plugin
cargo build --release --target wasm32-wasi
```

The compiled WebAssembly module will be located at `target/wasm32-wasi/release/aurion_plugin_skeleton.wasm`.

## Running

You can execute the plugin with Wasmtime to simulate the orchestrator:

```bash
wasmtime target/wasm32-wasi/release/aurion_plugin_skeleton.wasm
```

The plugin will print a JSON document containing a host, service, tool run and evidence objects.  In a real deployment the orchestrator captures this JSON and inserts it into the graph.

## Container fallback

If your plugin depends on native binaries or libraries that cannot be compiled for WASI, you can package it as an OCI container.  Ensure that the container runs as a non‑root user, uses a read‑only filesystem and defines a `ENTRYPOINT` that reads a JobRequest from stdin and writes a JobResult JSON to stdout.  See `docs/PLUGINS.md` for details.

## Acceptance Criteria

This README explains how to build the skeleton plugin for the `wasm32-wasi` target using Cargo, how to run it with Wasmtime, and notes about container fallback.  It references installation commands and indicates where the built `.wasm` file resides.