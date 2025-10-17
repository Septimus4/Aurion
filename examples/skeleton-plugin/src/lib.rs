//! Minimal Aurion plugin skeleton.
//!
//! This plugin demonstrates how to compile a Rust crate to WebAssembly and emit
//! a normalized graph mutation JSON on stdout.  It defines a single `run`
//! function exposed with a C ABI so the orchestrator can invoke it via
//! Wasmtime.  The plugin returns `0` on success and prints the mutation.

use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
struct Host {
    ip: String,
    hostname: String,
}

#[derive(Serialize)]
struct Service {
    protocol: String,
    port: u16,
    product: String,
    version: String,
}

#[derive(Serialize)]
struct ToolRun {
    name: String,
    version: String,
    cache_key: String,
}

#[derive(Serialize)]
struct Evidence {
    summary: String,
    cas_key: String,
}

/// Entry point called by the orchestrator.  Always return 0 on success.
#[no_mangle]
pub extern "C" fn run() -> i32 {
    // Compose an example mutation.  In a real plugin this would be based on
    // tool output; here we simply emit a single host, service and tool run.
    let host = Host {
        ip: "127.0.0.1".to_string(),
        hostname: "localhost".to_string(),
    };
    let service = Service {
        protocol: "tcp".to_string(),
        port: 80,
        product: "ExampleHTTP".to_string(),
        version: "0.1".to_string(),
    };
    // Compute a dummy cache key using blake3 of static data
    let cache_key = blake3::hash(b"skeleton-plugin-0.1.0").to_hex().to_string();
    let tool_run = ToolRun {
        name: "skeleton".to_string(),
        version: "0.1.0".to_string(),
        cache_key: cache_key.clone(),
    };
    let evidence = Evidence {
        summary: "Example evidence emitted by skeleton plugin".to_string(),
        cas_key: cache_key,
    };
    let mutation = json!({
        "host": host,
        "service": service,
        "tool_run": tool_run,
        "evidence": evidence,
    });
    // Print JSON to stdout.  The orchestrator parses stdout to extract the
    // mutation; newlines are acceptable.
    println!("{}", mutation.to_string());
    0
}