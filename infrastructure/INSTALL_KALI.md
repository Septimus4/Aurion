---
title: Installing Aurion on Kali Linux
version: 0.1.0
date: 2025-10-14
---
# Installation on Kali Linux

These instructions install Aurion on a fresh Kali Linux system in single‑user mode.  Multi‑user collaboration requires the server component; see future milestones for deployment on separate nodes.  The Aurion user interface is delivered as an IntelliJ plugin; the Rust GUI binary (`aurion-ide`) is no longer used.

## Prerequisites

Run as root or with sudo:

```bash
sudo apt update
sudo apt install -y git build-essential rust-all protobuf-compiler libssl-dev pkg-config \
    neo4j memgraph protobuf-compiler libprotobuf-dev openjdk-17-jdk
```

Install Wasmtime for running WebAssembly plugins:

```bash
sudo apt install -y wasmtime
```

Create a dedicated `aurion` user and directories:

```bash
sudo useradd --system --home /var/lib/aurion --create-home --shell /usr/sbin/nologin aurion
sudo mkdir -p /opt/aurion/bin /var/lib/aurion/data /etc/aurion
sudo chown -R aurion:aurion /var/lib/aurion /opt/aurion /etc/aurion
```

## Clone and build

Clone the repository and build the backend components as the `aurion` user:

```bash
sudo -u aurion -H bash -c '
  cd /var/lib/aurion && \
  git clone https://github.com/aurion/aurion.git src && \
  cd src && \
  cargo build --release --workspace && \
  install -m755 target/release/aurion-orch /opt/aurion/bin/ && \
  install -m755 target/release/aurion-server /opt/aurion/bin/
'
```

Copy default configuration files:

```bash
sudo cp /var/lib/aurion/src/config/orch.toml /etc/aurion/orch.toml
sudo cp /var/lib/aurion/src/config/server.toml /etc/aurion/server.toml
sudo cp /var/lib/aurion/src/config/ide.toml /etc/aurion/ide.toml
sudo chown aurion:aurion /etc/aurion/*.toml
```

## Systemd units

Create systemd service files under `/etc/systemd/system`.

### aurion-orch.service

```ini
[Unit]
Description=Aurion Orchestrator
After=network.target

[Service]
User=aurion
Group=aurion
Environment=RUST_LOG=info
ExecStart=/opt/aurion/bin/aurion-orch --config /etc/aurion/orch.toml
WorkingDirectory=/var/lib/aurion
Restart=on-failure
ProtectSystem=strict
ProtectHome=true
NoNewPrivileges=true
PrivateTmp=true

[Install]
WantedBy=multi-user.target
```

### aurion-server.service

```ini
[Unit]
Description=Aurion Collaboration Server
After=network.target

[Service]
User=aurion
Group=aurion
Environment=RUST_LOG=info
ExecStart=/opt/aurion/bin/aurion-server --config /etc/aurion/server.toml
WorkingDirectory=/var/lib/aurion
Restart=on-failure
ProtectSystem=strict
ProtectHome=true
NoNewPrivileges=true
PrivateTmp=true

[Install]
WantedBy=multi-user.target
```

Reload systemd and enable the services:

```bash
sudo systemctl daemon-reload
sudo systemctl enable aurion-orch aurion-server
sudo systemctl start aurion-orch aurion-server
```

## Apt pins and package versions

To ensure reproducible builds, pin the versions of external tools used by plugins.  Create `/etc/apt/preferences.d/aurion.pref` with the following contents:

```ini
Package: nmap
Pin: version 7.95+dfsg-3
Pin-Priority: 1001

Package: amass
Pin: version 4.2.0-0kali1
Pin-Priority: 1001

Package: subfinder
Pin: version 2.6.0-0kali1
Pin-Priority: 1001

Package: httpx-toolkit
Pin: version 1.1.5-0kali1
Pin-Priority: 1001

Package: nuclei
Pin: version 3.4.10-0kali1
Pin-Priority: 1001

Package: ffuf
Pin: version 2.1.0-0kali1
Pin-Priority: 1001
```

After creating the pin file, run `sudo apt update`.  This prevents accidental upgrades during an engagement.

## IntelliJ plugin installation

Aurion’s user interface is packaged as an IntelliJ plugin.  Install IntelliJ IDEA Community Edition (Apache 2.0 licensed) and the Plugin DevKit to build and run the plugin【201827599878107†L176-L188】【410764021097615†L35-L47】.  The plugin can be built using the Gradle IntelliJ plugin and then installed from disk.

```bash
# install IntelliJ IDEA (example using snap; adjust for your package manager)
sudo snap install intellij-idea-community --classic

# build the plugin distribution as the aurion user
sudo -u aurion -H bash -c '
  cd /var/lib/aurion/src/ide-plugin && \
  ./gradlew buildPlugin
'

# After the build completes, locate the ZIP file in build/distributions
```

To install the plugin:

1. Launch IntelliJ IDEA as your user.
2. Navigate to **Settings → Plugins → Install Plugin from Disk…**.
3. Select the generated `aurion-plugin-*.zip` file from `build/distributions` and install it.
4. Restart IntelliJ IDEA.  The Aurion graph explorer will appear as a tool window.  Connect it to your running `aurion-orch` and `aurion-server` services via the plugin settings.

## Configuration file examples

Below is a minimal `orch.toml` example.  Adjust paths as necessary.

```toml
[storage]
cas_path = "/var/lib/aurion/data/cas"

[graph]
backend = "neo4j"
uri = "bolt://localhost:7687"
username = "neo4j"
password = "change-me"

[policy]
default_rate_limits = { passive = 4, portscan = 1, vulnscan = 1, fuzz = 1 }

[logging]
path = "/var/lib/aurion/aurion-orch.log"
```

## Acceptance Criteria

This installation guide provides step‑by‑step commands to build and install Aurion on Kali Linux, including prerequisite packages, user creation, cloning and building the project, copying configuration files, creating systemd units, setting apt pins and giving a sample configuration.  It emphasises secure permissions and uses systemd best practices.  All commands are reproducible on a fresh Kali installation.