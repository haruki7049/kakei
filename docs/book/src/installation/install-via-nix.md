# Install via Nix

If you have Nix with flakes enabled, you can install `kakei` directly from the repository:

## Quick Run (without installing)

```bash
# Run directly without installing
nix run github:haruki7049/kakei -- --help
```

## Install to Profile

```bash
# Install from the flake
nix profile install github:haruki7049/kakei
```

## Development Environment

For local development, the flake provides a development shell:

```bash
# Enter development shell with Rust toolchain and dependencies
nix develop
```

The flake provides:

- `packages.default`: The kakei binary
- `devShells.default`: Development environment with Rust toolchain and dependencies
