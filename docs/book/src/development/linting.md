# Linting

### Clippy

Run the Clippy linter for best practices and common mistakes:

```bash
cargo clippy --workspace
```

Fix warnings automatically (when possible):

```bash
cargo clippy --workspace --fix
```

Clippy with all warnings as errors:

```bash
cargo clippy --workspace -- -D warnings
```

### Format Check

Check if code is properly formatted:

```bash
cargo fmt --check
```

### Format Code

Auto-format all code:

```bash
cargo fmt
```
