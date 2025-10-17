---
title: AI Agent Development
version: 0.1.0
date: 2025-10-15
---
# Developing AI Agents for Aurion

Aurion exposes intelligent capabilities through a dedicated **AI/MCP server** (`aurion‑ai`).  This server implements the **Model Context Protocol** (MCP), an open‑source standard for connecting AI applications to external systems.  MCP allows agents (LLM‑powered assistants) to request information, invoke tools and perform tasks in a structured way over JSON‑RPC.  Think of MCP as a *USB‑C port for AI*: it standardises how AI models connect to data sources and tools【65787800937560†L60-L69】.  MCP hosts (clients) can connect to multiple MCP servers, each exposing different capabilities【772482920673209†L56-L83】.

This document guides developers on building AI agents that integrate with Aurion using MCP.  It assumes that the `aurion‑ai` server is running and accessible over a TCP port.  Agents can be developed in any language that speaks JSON‑RPC.  The provided SDK and example (see `examples/agent_sdk/` when available) offer a starting point.

## Background: Model Context Protocol

MCP is designed to give AI agents a consistent way to connect to tools, services and data.  The protocol defines three roles【772482920673209†L92-L103】:

* **Host**: the AI application or agent that initiates requests (e.g., a chat assistant embedded in the Aurion plugin).  Hosts maintain one or more MCP clients.
* **Client**: a connector within the host that manages the connection to a specific MCP server.  It sends JSON‑RPC requests and receives responses.
* **Server**: a service that exposes a set of actions.  Each action defines its name, parameters and return schema.  The server advertises its capabilities and executes the requested actions, returning structured results.

MCP messages are encoded using JSON‑RPC 2.0.  Each request includes an `id`, a `method` identifying the action and a `params` object containing arguments.  The response echoes the `id` and either a `result` or an `error`.  MCP servers use structured schemas to describe available methods, making them discoverable by clients.

## Aurion AI Architecture

Within Aurion, the AI/MCP server (`aurion‑ai`) acts as an adapter between AI agents and the internal orchestrator and graph database.  Its responsibilities include:

* Registering and exposing **actions** such as `summarise`, `classify_evidence`, `generate_runbook` and `list_entities`.  Each action is documented with input and output schemas.
* Receiving JSON‑RPC calls from clients, validating parameters and enforcing authorisation policies.  Active operations (e.g., triggering a port scan) require a valid `AuthorizationBundle` and are denied otherwise.
* Querying the graph database and CAS via the orchestrator to collect data needed for a request.
* Running native AI modules (Rust or embedded models) or delegating to external LLM endpoints to generate summaries or classifications.
* Returning results as structured JSON that the agent can present to users.

The AI server leverages the same security controls as the rest of Aurion.  Only passive actions (queries, summarisation, classification) are enabled by default.  Agents must include an `authorization_bundle` parameter when requesting active or high‑risk actions.

## Building an AI Agent

To build an MCP client that interacts with Aurion, follow these steps:

1. **Discover actions**.  Send a `rpc.discover` request to the AI server to retrieve the list of available methods and their schemas.

    ```json
    {
      "jsonrpc": "2.0",
      "id": 1,
      "method": "rpc.discover",
      "params": {}
    }
    ```

    The server responds with a description of actions.  Use this to build strongly typed calls.

2. **Issue a summarisation request**.  To summarise all findings for a host, call the `summarise` method with the `entity_id` of the host:

    ```json
    {
      "jsonrpc": "2.0",
      "id": 2,
      "method": "summarise",
      "params": { "entity_id": "host-123" }
    }
    ```

    The AI server fetches the relevant `ToolRun`, `Evidence` and `Finding` nodes, runs a summarisation model and returns a natural language summary.  The result may include references to CAS digests for further reading.

3. **Generate a runbook**.  To request a sequence of recommended actions for an engagement, call `generate_runbook`:

    ```json
    {
      "jsonrpc": "2.0",
      "id": 3,
      "method": "generate_runbook",
      "params": { "scope": ["example.com", "192.0.2.0/24"], "max_steps": 5 }
    }
    ```

    The AI server analyses current graph coverage and suggests a list of plugin executions (e.g., passive enumeration, port scan, vulnerability scan) along with justifications.  If the request includes high‑risk actions, an `authorization_bundle` must be provided in the params.

4. **Handle errors**.  MCP uses JSON‑RPC error objects to indicate problems.  For example, if the requested entity is out of scope or the action requires authorisation, the server will return an error with a code and message.  Agents should handle these gracefully and inform the user.

5. **Maintain context**.  AI conversations may require context persistence.  MCP clients can maintain local state or use the `context_id` parameter (if supported) to let the server track ongoing dialogues.  Avoid sending sensitive data to external AI providers; summarisation should occur locally where possible.

Developers can implement clients using any HTTP/WebSocket library that supports JSON‑RPC.  The example agent included in this repository demonstrates a simple Python client that connects to `aurion‑ai`, discovers actions and issues summary requests.

## Security and Privacy Considerations

Agents must respect Aurion’s security policies.  When invoking actions that could trigger network activity (port scans, vulnerability scans), they must include a valid `AuthorizationBundle` in the parameters.  The AI server validates scopes and capabilities using the same rules described in `SECURITY_POLICY.md`.

Avoid sending confidential evidence or secrets to external LLM endpoints.  Prefer local AI models for summarisation and classification.  When using remote providers, anonymise data and restrict the amount of context transmitted.  Administrators should configure rate limits and content filters on the AI server to prevent abuse.

## Acceptance Criteria

This agent development guide introduces MCP and explains how to build AI agents that interact with Aurion.  It describes the roles of hosts, clients and servers, outlines the responsibilities of the `aurion‑ai` server and provides example JSON‑RPC requests for discovery, summarisation and runbook generation.  It includes security considerations and references authoritative sources for MCP【65787800937560†L60-L69】【772482920673209†L56-L83】.  Developers following this guide should be able to build a basic agent and understand how to extend it.