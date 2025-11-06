# Pulse - High-Performance CLI Monitoring

A blazing-fast Rust CLI tool for monitoring command outputs and sending change notifications to Discord webhooks. Built with performance and efficiency as core design principles.

## Commands

### Build
```bash
# Debug build (faster compile, slower runtime)
cargo build

# Release build (optimized for performance)
cargo build --release

# Check without building
cargo check
```

### Run
```bash
# Single monitor mode
cargo run --release -- run --command "ifo --place 'NYC'" --interval 60 --webhook <url>

# Config file mode
cargo run --release -- start --config pulse.config.toml

# After installation
pulse run --command "your-command" --interval 60 --webhook <url>
pulse start --config pulse.config.toml
```

### Testing
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Code Quality
```bash
# Format code
cargo fmt

# Lint with clippy
cargo clippy -- -D warnings
```

## Development Environment

### Devcontainer
This project includes a devcontainer configuration for consistent development:
```bash
# Open in VS Code with Remote-Containers extension
code .
# Select "Reopen in Container"
```

### Cross-Platform Builds
```bash
# Linux (native)
cargo build --release --target x86_64-unknown-linux-gnu

# macOS Intel
cargo zigbuild --release --target x86_64-apple-darwin

# macOS Apple Silicon
cargo zigbuild --release --target aarch64-apple-darwin

# Windows
cargo zigbuild --release --target x86_64-pc-windows-gnu
```

## Architecture

### Core Modules

**src/main.rs**
- CLI entry point and argument parsing with `clap`
- Command routing and initialization
- Performance-focused startup with minimal overhead

**src/lib.rs**
- Public library interface for extensibility
- Exports core types and functions

**src/monitor.rs**
- Command execution using `tokio::process::Command`
- Async polling loop with configurable intervals
- Efficient change detection (only notify on diffs)
- Zero-copy output comparison where possible

**src/discord.rs**
- HTTP client for Discord webhooks via `reqwest`
- Async message sending with retry logic
- Connection pooling for reduced latency
- Rate limiting compliance

**src/config.rs**
- TOML configuration file parsing with `serde`
- Multiple monitor support with concurrent execution
- Validation and error handling

**src/error.rs**
- Custom error types using `thiserror`
- Contextual error handling with `anyhow`
- Informative error messages for debugging

**src/types.rs**
- Shared type definitions
- Serde-compatible structs for serialization
- Zero-cost abstractions where applicable

### Data Flow

1. **Initialization**: Parse CLI args or load config file
2. **Setup**: Create monitor instances with command, interval, webhook
3. **Execution Loop**: For each monitor (concurrent):
   - Execute command via `tokio::process::Command`
   - Capture stdout/stderr with streaming
   - Compare with previous output (diff detection)
   - If changed: Format and send Discord webhook
   - Sleep for configured interval
4. **Async Concurrency**: All monitors run independently using Tokio tasks

### Key Technologies

**clap (4.x)**
- Zero-cost CLI argument parsing
- Compile-time argument validation
- Minimal runtime overhead

**tokio (1.x)**
- High-performance async runtime
- Efficient task scheduling
- Non-blocking I/O for command execution

**reqwest (0.11+)**
- Async HTTP client with connection pooling
- Native TLS support
- Automatic JSON serialization

**serde/serde_json**
- Zero-copy deserialization where possible
- Efficient JSON handling
- Compile-time optimization

**thiserror/anyhow**
- Zero-cost error abstractions
- Rich error context
- Ergonomic error handling

### Performance Optimizations

**Release Profile** (Cargo.toml):
```toml
[profile.release]
opt-level = 3          # Maximum optimization
lto = "fat"            # Link-time optimization across crates
codegen-units = 1      # Single codegen unit for better optimization
strip = true           # Strip symbols for smaller binary
panic = "abort"        # Abort on panic (no unwinding overhead)
```

**Runtime Optimizations**:
- Async/await throughout (no blocking operations)
- Efficient string comparison for change detection
- Minimal allocations in hot paths
- Connection pooling for HTTP requests
- Lazy initialization where appropriate

**Memory Efficiency**:
- Bounded buffers for command output
- Smart pointer usage for shared state
- Explicit drop points for large allocations

### Testing Strategy

**Unit Tests**:
- Individual module functionality
- Error handling paths
- Edge cases and validation

**Integration Tests**:
- End-to-end command execution
- Config file parsing
- Discord webhook integration (with mocks)

**Performance Tests**:
- Monitor overhead measurement
- Memory usage profiling
- Concurrent execution stress tests

## Development Practices

### Design Principles
1. **Performance First**: Every decision optimized for speed and efficiency
2. **Async Everything**: No blocking operations in runtime paths
3. **Fail Fast**: Validate early, handle errors gracefully
4. **Type Safety**: Leverage Rust's type system for correctness
5. **Minimal Dependencies**: Only essential, well-maintained crates
6. **Zero-Cost Abstractions**: Abstractions that compile away

### Code Style
- Follow Rust idioms and conventions
- Use `rustfmt` for consistent formatting
- Pass `clippy` lints with no warnings
- Document public APIs with doc comments
- Write descriptive commit messages

## Release

This project uses automated releases via GitHub Actions:
- Multi-platform builds (Linux, macOS Intel/ARM, Windows)
- Optimized release binaries with LTO
- SHA256 checksums for integrity verification
- Signed commits for authenticity

Ready for production use with focus on reliability and performance.
