---
title: Tool Inventory
version: 0.1.0
date: 2025-10-14
---
# Tool Inventory

Aurion integrates a curated set of free and open‑source tools.  Each tool is wrapped by a plugin that normalises output and enforces the risk policy.  The table below lists the initial set of tools, their purpose, installation instructions and risk classification.

| Phase | Capability | Tool | Install command / package | Justification | Risk | Enabled by default |
|------|-----------|------|---------------------------|--------------|-----|-------------------|
| Information gathering | Passive enumeration | **nmap** | `sudo apt install nmap` | Nmap is a widely used network exploration and security auditing tool that supports ping scanning, port scanning and service/version detection【147964860358445†L69-L79】.  It is used in passive mode (no SYN scans) to identify live hosts and services. | Low | Yes |
| Information gathering | Passive enumeration | **amass** | `sudo apt install amass` | The OWASP Amass project performs network mapping and asset discovery using open source information and can perform brute‑force enumeration upon request【615059556182302†L66-L80】.  Only passive modes are enabled by default. | Low | Yes |
| Discovery | Passive subdomain discovery | **subfinder** | `sudo apt install subfinder` | Subfinder discovers valid subdomains for websites using passive online sources and has a simple modular architecture【712436319311372†L61-L64】. | Low | Yes |
| Service discovery | Port scanning | **masscan** | `sudo apt install masscan` | Masscan transmits asynchronous SYN packets and produces results similar to nmap; it is extremely fast and allows scanning arbitrary address ranges【909477181688949†L96-L100】.  Because it can generate high traffic, it is gated behind an authorisation. | Medium | No |
| HTTP probing | Service fingerprinting | **httpx-toolkit** | `sudo apt install httpx-toolkit` | ProjectDiscovery's HTTPX toolkit is a fast, multi‑purpose HTTP probe that runs multiple probers using a retryable HTTP library【746340131229728†L59-L72】.  It is used to collect HTTP headers, status codes and titles. | Medium | Yes |
| Vulnerability scanning | Template‑driven scanning | **nuclei** | `sudo apt install nuclei` | Nuclei is a fast, template‑based scanner that sends requests across targets using YAML templates and supports multiple protocols including HTTP, DNS and TCP【439043444620191†L59-L68】.  Running nuclei may trigger active checks, so it requires authorisation. | High | No |
| Fuzzing | Directory and parameter fuzzing | **ffuf** | `sudo apt install ffuf` | FFUF (Fuzz Faster U Fool) is a fast web fuzzer written in Go that performs directory discovery, virtual host discovery and parameter fuzzing【427489552364393†L69-L74】.  It can generate substantial traffic, so it is gated. | High | No |
| Exploitation framework | Exploit automation | **metasploit-framework** | `sudo apt install metasploit-framework` | The Metasploit Framework provides a rich set of exploit modules and payloads.  Only the `exploit` and `auxiliary` modules are exposed through the plugin; usage is disabled by default and always requires a signed AuthorizationBundle. | High | No |

### Installation notes

Most tools are packaged in Kali's repositories and can be installed with `apt`.  For tools not available in `apt` or where a newer version is required, plugins may download binaries from the upstream release pages.  In such cases the orchestrator must verify the SHA256 checksum provided by the upstream release and pin the version in the plugin manifest.  ProjectDiscovery tools (`subfinder`, `httpx`, `nuclei`) can be installed via `go install` but the pinned `apt` packages on Kali simplify maintenance.

### Fallback installation for unsupported tools

If a tool is not packaged for Kali or an operator needs a specific version, the following pattern is recommended:

1. Download the prebuilt binary for your architecture from the upstream releases page.
2. Verify its SHA256 checksum against the value published by the upstream project.
3. Extract the archive into `/opt/aurion/tools/<tool-name>/<version>` and update the plugin configuration to use that path.
4. Optionally create a wrapper script in `/usr/local/bin` that execs the pinned binary.

### Risk classification

- **Low**: purely passive operations that do not send unsolicited packets (e.g., DNS enumeration, certificate transparency scraping).
- **Medium**: operations that send limited probes such as SYN scans or HTTP requests; these can still trigger intrusion detection systems.
- **High**: operations capable of exploiting vulnerabilities, brute‑force authentication or generating high request volumes.

### Acceptance Criteria

This tool inventory includes a table describing each tool, its installation method, justification with citations from authoritative sources, risk class and whether it is enabled by default.  It lists at least seven tools, covers apt installation commands and outlines fallback installation and checksum verification strategies.  Gated tools are clearly marked with high risk.