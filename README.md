# Pulse

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/license/MIT)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/arnops/pulse/releases)
[![Tests](https://img.shields.io/badge/tests-passing-brightgreen.svg)](https://github.com/arnops/pulse/actions)
[![Signed Commits](https://img.shields.io/badge/commits-signed-green.svg)](.signing-info)
[![Changelog](https://img.shields.io/badge/changelog-Keep%20a%20Changelog-orange.svg)](CHANGELOG.md)

A Rust CLI tool for monitoring command outputs and sending notifications to Discord webhooks.

## Installation

### Quick Install (Pre-built Binaries)

Download pre-compiled binaries from the [latest release](https://github.com/arnops/pulse/releases/latest):

**Linux (x86_64):**
```bash
wget https://github.com/arnops/pulse/releases/download/v0.1.0/pulse-x86_64-unknown-linux-gnu.tar.gz
tar xzf pulse-x86_64-unknown-linux-gnu.tar.gz
sudo mv pulse /usr/local/bin/
pulse --version
```

**macOS (Intel):**
```bash
curl -L https://github.com/arnops/pulse/releases/download/v0.1.0/pulse-x86_64-apple-darwin.tar.gz -o pulse.tar.gz
tar xzf pulse.tar.gz
sudo mv pulse /usr/local/bin/
pulse --version
```

**macOS (Apple Silicon):**
```bash
curl -L https://github.com/arnops/pulse/releases/download/v0.1.0/pulse-aarch64-apple-darwin.tar.gz -o pulse.tar.gz
tar xzf pulse.tar.gz
sudo mv pulse /usr/local/bin/
pulse --version
```

**Windows:**
Download `pulse-x86_64-pc-windows-gnu.zip` from [releases](https://github.com/arnops/pulse/releases/latest), extract, and add to PATH.

### Build from Source

```bash
# Requires Rust 1.70+
git clone https://github.com/arnops/pulse.git
cd pulse
cargo build --release
sudo cp target/release/pulse /usr/local/bin/
```

For detailed installation instructions, platform-specific guides, and troubleshooting, see [INSTALL.md](INSTALL.md).

## Tech Stack

- **clap** - CLI argument parsing
- **tokio** - Async runtime
- **reqwest** - HTTP client for Discord webhooks
- **serde/serde_json** - JSON serialization
- **thiserror/anyhow** - Error handling

## Project Structure

```
pulse/
├── Cargo.toml
├── src/
│   ├── main.rs          # CLI entry & argument parsing
│   ├── lib.rs           # Public library interface
│   ├── monitor.rs       # Command execution & polling logic
│   ├── discord.rs       # Discord webhook client
│   ├── config.rs        # Config file handling
│   ├── error.rs         # Custom error types
│   └── types.rs         # Shared types
├── pulse.config.example.toml
└── CLAUDE.md
```

## Core Features

1. Execute CLI commands at intervals using `tokio::process::Command`
2. Track output changes (only send updates when different)
3. Send formatted messages to Discord webhooks via reqwest
4. Support config files (TOML format)
5. Multiple monitor support with async concurrency

## CLI Usage

### Single monitor
```bash
pulse run --command "ifo --place 'NYC'" --interval 60 --webhook <url>
```

### From config file
```bash
pulse start --config pulse.config.toml
```