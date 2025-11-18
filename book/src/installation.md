# Installation

This guide covers all the ways you can install kakei on your system.

## Prerequisites

Before installing kakei, ensure you have:

- **Rust 1.91.1 or later** (for building from source)
- **SQLite 3** (for database functionality)

## Build from Source

Building from source gives you the latest version and allows you to customize the build:

```bash
git clone https://github.com/haruki7049/kakei.git
cd kakei
cargo build --release
```

The binary will be available at `target/release/kakei`. You can copy it to a directory on your PATH (e.g., `/usr/local/bin`) if desired:

```bash
sudo cp target/release/kakei /usr/local/bin/
```

## Install via Cargo

The easiest way to install kakei if you already have Rust installed:

From the repository root:

```bash
cargo install --path .
```

This installs `kakei` to your cargo bin directory (usually `~/.cargo/bin`).

Make sure `~/.cargo/bin` is in your PATH:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

## Binary Releases

Pre-built binaries may be available for download on the [GitHub Releases](https://github.com/haruki7049/kakei/releases) page.

To install from a release:

1. Download the appropriate archive for your platform
1. Extract the `kakei` binary
1. Move it to a directory on your PATH (e.g., `/usr/local/bin/`)
1. Make it executable: `chmod +x /usr/local/bin/kakei`

## Install via Nix

If you have Nix with flakes enabled, you can install `kakei` directly from the repository:

### Quick Run (without installing)

```bash
# Run directly without installing
nix run github:haruki7049/kakei -- --help
```

### Install to Profile

```bash
# Install from the flake
nix profile install github:haruki7049/kakei
```

### Development Environment

For local development, the flake provides a development shell:

```bash
# Enter development shell with Rust toolchain and dependencies
nix develop
```

The flake provides:

- `packages.default`: The kakei binary
- `devShells.default`: Development environment with Rust toolchain and dependencies

## Platform Notes

### Linux

On Linux, `kakei` follows the [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html):

- **Configuration**: `~/.config/kakei/` (or `$XDG_CONFIG_HOME/kakei/`)
- **Data**: `~/.local/share/kakei/` (or `$XDG_DATA_HOME/kakei/`)

### macOS

On macOS, if XDG environment variables are not set, kakei will use:

- **Configuration**: `~/Library/Application Support/kakei/`
- **Data**: `~/Library/Application Support/kakei/`

### Windows

On Windows, if XDG environment variables are not set, kakei will use:

- **Configuration**: `%APPDATA%\kakei\`
- **Data**: `%APPDATA%\kakei\`

## Verifying Installation

After installation, verify that kakei is correctly installed:

```bash
kakei --help
```

You should see the help message with available commands.

## Next Steps

Now that you have kakei installed, proceed to the [Quick Start](./quick-start.md) guide to initialize your database and start tracking your finances!
