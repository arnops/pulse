# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-11-06

### Added
- Initial release of Pulse
- CLI monitoring tool for executing commands at intervals
- Discord webhook integration for notifications
- Change detection - only notify when output changes
- Single monitor mode via `pulse run` command
- Multiple monitor mode via `pulse start` with TOML config
- Async execution using Tokio for high performance
- Connection pooling for HTTP requests
- Rich Discord embeds with formatted output
- Example configuration file (`pulse.config.example.toml`)
- Comprehensive documentation in CLAUDE.md
- GitHub Actions workflow for multi-platform releases
- Development container configuration
- MIT License
- Performance optimizations:
  - LTO (Link-Time Optimization)
  - Single codegen unit
  - Optimized release profile
  - Zero-copy patterns where possible

### Tech Stack
- clap 4.5 for CLI parsing
- tokio 1.42 for async runtime
- reqwest 0.12 for HTTP client
- serde/toml for configuration
- thiserror/anyhow for error handling

[Unreleased]: https://github.com/arnops/pulse/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/arnops/pulse/releases/tag/v0.1.0
