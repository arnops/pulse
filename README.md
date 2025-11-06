# Pulse

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/license/MIT)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/arnops/pulse/releases)
[![Tests](https://img.shields.io/badge/tests-passing-brightgreen.svg)](https://github.com/arnops/pulse/actions)
[![Signed Commits](https://img.shields.io/badge/commits-signed-green.svg)](.signing-info)
[![Changelog](https://img.shields.io/badge/changelog-Keep%20a%20Changelog-orange.svg)](CHANGELOG.md)

A Rust CLI tool for monitoring command outputs and sending notifications to Discord webhooks.

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