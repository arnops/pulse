# Pulse

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/license/MIT)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/arnops/pulse/releases)
[![Tests](https://img.shields.io/badge/tests-passing-brightgreen.svg)](https://github.com/arnops/pulse/actions)
[![Signed Commits](https://img.shields.io/badge/commits-signed-green.svg)](.signing-info)
[![Changelog](https://img.shields.io/badge/changelog-Keep%20a%20Changelog-orange.svg)](CHANGELOG.md)

A Rust CLI tool for monitoring command outputs and sending notifications to Discord webhooks.

## Features

- 🚀 **Blazing fast**: Async I/O, efficient change detection, minimal overhead
- 🔔 **Discord notifications**: Rich embeds with formatted command output
- 👁️ **Change tracking**: Only notify when output differs from previous execution
- ⚙️ **Flexible configuration**: Single monitor CLI mode or multi-monitor config files
- 🔄 **Concurrent monitoring**: Run multiple monitors simultaneously with async execution
- 🔒 **Secure**: Input validation, proper error handling, HTTPS-only webhooks
- 💾 **Lightweight**: Single 2.6MB binary, minimal memory footprint
- 🎯 **Production-ready**: Optimized with LTO, aggressive compilation flags

## Performance

- **Startup**: ~4ms (sub-5ms)
- **Execution**: Async command execution with zero blocking
- **Memory**: ~10MB base footprint (scales with monitor count)
- **Binary size**: 2.6MB single executable
- **Concurrency**: Unlimited monitors with Tokio's efficient task scheduling

## Installation

Download pre-built binaries for Linux, macOS, or Windows from the [latest release](https://github.com/arnops/pulse/releases/latest), or build from source with Rust 1.70+.

For detailed installation instructions, platform-specific guides, systemd/launchd setup, and troubleshooting, see **[INSTALL.md](INSTALL.md)**.

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

## Usage

### Single monitor
```bash
pulse run --command "ifo --place 'NYC'" --interval 60 --webhook <url>
```

### From config file
```bash
pulse start --config pulse.config.toml
```