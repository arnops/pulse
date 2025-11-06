# Installation Guide

This guide provides detailed instructions for installing Pulse on various platforms.

## Table of Contents

- [Pre-built Binaries](#pre-built-binaries)
  - [Linux](#linux-x86_64)
  - [macOS Intel](#macos-intel-x86_64)
  - [macOS Apple Silicon](#macos-apple-silicon-arm64)
  - [Windows](#windows-x86_64)
- [Build from Source](#build-from-source)
- [Cross-Platform Building](#cross-platform-building-advanced)
- [Verifying Installation](#verifying-installation)
- [Quick Start](#quick-start)
- [Troubleshooting](#troubleshooting)

---

## Pre-built Binaries

Download pre-compiled binaries from the [latest release](https://github.com/arnops/pulse/releases/latest).

### Linux (x86_64)

```bash
# Download the latest release
wget https://github.com/arnops/pulse/releases/download/v0.1.0/pulse-x86_64-unknown-linux-gnu.tar.gz

# Verify checksum (recommended)
wget https://github.com/arnops/pulse/releases/download/v0.1.0/SHA256SUMS
sha256sum -c SHA256SUMS 2>&1 | grep pulse-x86_64-unknown-linux-gnu.tar.gz

# Extract the archive
tar xzf pulse-x86_64-unknown-linux-gnu.tar.gz

# Move to a directory in your PATH (requires sudo)
sudo mv pulse /usr/local/bin/

# Make it executable
sudo chmod +x /usr/local/bin/pulse

# Verify installation
pulse --version
```

**Alternative: Local installation (no sudo)**

```bash
# Extract to your home directory
tar xzf pulse-x86_64-unknown-linux-gnu.tar.gz
mkdir -p ~/.local/bin
mv pulse ~/.local/bin/

# Add to PATH (add to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.local/bin:$PATH"

# Reload shell configuration
source ~/.bashrc  # or source ~/.zshrc

# Verify
pulse --version
```

### macOS Intel (x86_64)

```bash
# Download the latest release
curl -L https://github.com/arnops/pulse/releases/download/v0.1.0/pulse-x86_64-apple-darwin.tar.gz -o pulse.tar.gz

# Verify checksum (recommended)
curl -L https://github.com/arnops/pulse/releases/download/v0.1.0/SHA256SUMS -o SHA256SUMS
shasum -a 256 -c SHA256SUMS 2>&1 | grep pulse-x86_64-apple-darwin.tar.gz

# Extract the archive
tar xzf pulse.tar.gz

# Move to a directory in your PATH
sudo mv pulse /usr/local/bin/

# Make it executable
sudo chmod +x /usr/local/bin/pulse

# Verify installation
pulse --version
```

**Note:** On first run, macOS may block the binary. If you see a security warning:
1. Go to **System Preferences → Security & Privacy → General**
2. Click **"Allow Anyway"** next to the pulse message
3. Run `pulse --version` again

### macOS Apple Silicon (ARM64)

```bash
# Download the latest release
curl -L https://github.com/arnops/pulse/releases/download/v0.1.0/pulse-aarch64-apple-darwin.tar.gz -o pulse.tar.gz

# Verify checksum (recommended)
curl -L https://github.com/arnops/pulse/releases/download/v0.1.0/SHA256SUMS -o SHA256SUMS
shasum -a 256 -c SHA256SUMS 2>&1 | grep pulse-aarch64-apple-darwin.tar.gz

# Extract the archive
tar xzf pulse.tar.gz

# Move to a directory in your PATH
sudo mv pulse /usr/local/bin/

# Make it executable
sudo chmod +x /usr/local/bin/pulse

# Verify installation
pulse --version
```

### Windows (x86_64)

#### Using PowerShell:

```powershell
# Download the latest release
Invoke-WebRequest -Uri "https://github.com/arnops/pulse/releases/download/v0.1.0/pulse-x86_64-pc-windows-gnu.zip" -OutFile "pulse.zip"

# Download checksums
Invoke-WebRequest -Uri "https://github.com/arnops/pulse/releases/download/v0.1.0/SHA256SUMS" -OutFile "SHA256SUMS"

# Extract the archive
Expand-Archive -Path pulse.zip -DestinationPath .

# Verify installation
.\pulse.exe --version
```

#### Adding to PATH (Optional):

1. Move `pulse.exe` to a permanent location (e.g., `C:\Program Files\Pulse\`)
2. Open **System Properties → Advanced → Environment Variables**
3. Under **System Variables**, select **Path** and click **Edit**
4. Click **New** and add the directory containing `pulse.exe`
5. Click **OK** to save
6. Open a new Command Prompt or PowerShell window
7. Verify: `pulse --version`

#### Using Windows Subsystem for Linux (WSL):

If you have WSL installed, you can use the Linux instructions instead.

---

## Build from Source

Building from source gives you the latest features and allows you to customize the build.

### Prerequisites

- **Rust** 1.70 or later (2021 edition)
- **Cargo** (comes with Rust)

Install Rust from [rustup.rs](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build Steps

```bash
# Clone the repository
git clone https://github.com/arnops/pulse.git
cd pulse

# Build the release binary (optimized)
cargo build --release

# The binary will be at: target/release/pulse
# Test it
./target/release/pulse --version

# Optional: Install system-wide
sudo cp target/release/pulse /usr/local/bin/

# Or install to ~/.local/bin (no sudo required)
mkdir -p ~/.local/bin
cp target/release/pulse ~/.local/bin/
export PATH="$HOME/.local/bin:$PATH"  # Add to ~/.bashrc or ~/.zshrc
```

### Build Options

**Debug build (faster compilation, slower runtime):**
```bash
cargo build
./target/debug/pulse --version
```

**Release build with maximum optimization (recommended):**
```bash
cargo build --release
./target/release/pulse --version
```

---

## Cross-Platform Building (Advanced)

For developers who want to build binaries for other platforms using cross-compilation.

### Prerequisites

- Rust toolchain
- [Zig](https://ziglang.org/download/) 0.13.0 or later
- `cargo-zigbuild`

### Setup

```bash
# Install Zig
# On macOS:
brew install zig

# On Linux (download from ziglang.org):
wget https://ziglang.org/download/0.13.0/zig-linux-x86_64-0.13.0.tar.xz
tar xf zig-linux-x86_64-0.13.0.tar.xz
sudo mv zig-linux-x86_64-0.13.0 /usr/local/zig
export PATH="/usr/local/zig:$PATH"

# Install cargo-zigbuild
cargo install cargo-zigbuild
```

### Build for Specific Targets

```bash
# Add target support
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add x86_64-pc-windows-gnu

# Build for Linux
cargo zigbuild --release --target x86_64-unknown-linux-gnu

# Build for macOS Intel
cargo zigbuild --release --target x86_64-apple-darwin

# Build for macOS Apple Silicon
cargo zigbuild --release --target aarch64-apple-darwin

# Build for Windows
cargo zigbuild --release --target x86_64-pc-windows-gnu

# Binaries will be in: target/<target>/release/
```

---

## Verifying Installation

After installation, verify that Pulse is working correctly:

```bash
# Check version
pulse --version

# View help
pulse --help

# Test with a simple command (requires Discord webhook)
pulse run --command "date" --interval 60 --webhook "YOUR_WEBHOOK_URL"
```

---

## Quick Start

### 1. Get a Discord Webhook URL

1. Open Discord and go to the server where you want to receive notifications
2. Go to **Server Settings → Integrations → Webhooks**
3. Click **New Webhook**
4. Configure the webhook (name, channel)
5. Click **Copy Webhook URL**

### 2. Single Monitor Mode

Monitor a single command and send notifications on changes:

```bash
pulse run \
  --command "uptime" \
  --interval 60 \
  --webhook "https://discord.com/api/webhooks/YOUR_WEBHOOK_ID/YOUR_TOKEN"
```

Options:
- `--command`: The command to execute
- `--interval`: Seconds between executions
- `--webhook`: Discord webhook URL
- `--name`: Optional name for the monitor (defaults to command)

### 3. Multiple Monitors (Config File)

Create a configuration file for multiple monitors:

```bash
# Copy example config
cp pulse.config.example.toml pulse.config.toml

# Edit with your settings
nano pulse.config.toml
```

Example `pulse.config.toml`:

```toml
[[monitors]]
name = "System Uptime"
command = "uptime"
interval = 300  # 5 minutes
webhook = "https://discord.com/api/webhooks/YOUR_WEBHOOK_ID/YOUR_TOKEN"

[[monitors]]
name = "Disk Usage"
command = "df -h /"
interval = 600  # 10 minutes
webhook = "https://discord.com/api/webhooks/YOUR_WEBHOOK_ID/YOUR_TOKEN"

[[monitors]]
name = "Memory Usage"
command = "free -h"
interval = 300  # 5 minutes
webhook = "https://discord.com/api/webhooks/YOUR_WEBHOOK_ID/YOUR_TOKEN"
```

Run with config file:

```bash
pulse start --config pulse.config.toml
```

### 4. Running as a Background Service

#### Linux (systemd)

Create a systemd service file:

```bash
sudo nano /etc/systemd/system/pulse.service
```

Add the following content:

```ini
[Unit]
Description=Pulse Monitoring Service
After=network.target

[Service]
Type=simple
User=your-username
WorkingDirectory=/home/your-username/pulse
ExecStart=/usr/local/bin/pulse start --config /home/your-username/pulse/pulse.config.toml
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Enable and start the service:

```bash
sudo systemctl daemon-reload
sudo systemctl enable pulse
sudo systemctl start pulse

# Check status
sudo systemctl status pulse

# View logs
sudo journalctl -u pulse -f
```

#### macOS (launchd)

Create a launch agent file:

```bash
nano ~/Library/LaunchAgents/com.arnops.pulse.plist
```

Add the following content:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.arnops.pulse</string>
    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/pulse</string>
        <string>start</string>
        <string>--config</string>
        <string>/Users/your-username/pulse.config.toml</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>WorkingDirectory</key>
    <string>/Users/your-username</string>
    <key>StandardOutPath</key>
    <string>/Users/your-username/pulse.log</string>
    <key>StandardErrorPath</key>
    <string>/Users/your-username/pulse.error.log</string>
</dict>
</plist>
```

Load the service:

```bash
launchctl load ~/Library/LaunchAgents/com.arnops.pulse.plist

# Check status
launchctl list | grep pulse

# View logs
tail -f ~/pulse.log
```

#### Windows (Task Scheduler)

1. Open **Task Scheduler**
2. Click **Create Task**
3. **General Tab:**
   - Name: "Pulse Monitoring"
   - Check "Run whether user is logged on or not"
4. **Triggers Tab:**
   - New → At startup
5. **Actions Tab:**
   - New → Start a program
   - Program: `C:\Program Files\Pulse\pulse.exe`
   - Arguments: `start --config C:\path\to\pulse.config.toml`
6. Click **OK** to save

---

## Troubleshooting

### "pulse: command not found"

- Ensure the binary is in your PATH
- Try running with full path: `/usr/local/bin/pulse --version`
- Check installation location: `which pulse`

### "Permission denied"

```bash
# Make the binary executable
chmod +x /path/to/pulse
```

### macOS Security Warning

If macOS blocks the binary:
1. Go to **System Preferences → Security & Privacy**
2. Click **"Allow Anyway"**
3. Try running again

### Discord Webhook Errors

- Verify your webhook URL is correct
- Check that the webhook hasn't been deleted in Discord
- Ensure you have network connectivity
- Check Discord's rate limits (if sending too frequently)

### Build Errors

If building from source fails:

```bash
# Update Rust
rustup update

# Clean build artifacts
cargo clean

# Try building again
cargo build --release
```

### High CPU Usage

- Increase the interval between command executions
- Check that your monitored commands aren't too resource-intensive

### Memory Issues

- Monitor commands with large outputs may consume more memory
- Consider filtering output before piping to pulse
- Reduce the number of concurrent monitors

---

## Additional Resources

- **GitHub Repository:** https://github.com/arnops/pulse
- **Issue Tracker:** https://github.com/arnops/pulse/issues
- **Changelog:** [CHANGELOG.md](CHANGELOG.md)
- **Development Guide:** [CLAUDE.md](CLAUDE.md)

---

## Getting Help

If you encounter issues:

1. Check this installation guide
2. Review the [README](README.md)
3. Search [existing issues](https://github.com/arnops/pulse/issues)
4. Open a [new issue](https://github.com/arnops/pulse/issues/new) with:
   - Your operating system and version
   - Pulse version (`pulse --version`)
   - Complete error message
   - Steps to reproduce

---

**Happy Monitoring!** 🚀
