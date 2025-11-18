# Development

Guide for developers working on kakei.

## Prerequisites

Before you start developing kakei, ensure you have:

- **Rust 1.91.1 or later** (specified in `rust-toolchain.toml`)
- **SQLite 3** development libraries
- **Git** for version control
- **(Optional) Nix** for reproducible development environment

## Setting Up Development Environment

### Option 1: Standard Rust Setup

1. **Clone the repository:**

   ```bash
   git clone https://github.com/haruki7049/kakei.git
   cd kakei
   ```

1. **Install Rust** (if not already installed):

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

1. **Install the correct Rust version** (specified in `rust-toolchain.toml`):

   ```bash
   rustup install 1.91.1
   ```

1. **Build the project:**

   ```bash
   cargo build
   ```

### Option 2: Nix Development Shell

If you have Nix with flakes enabled:

```bash
# Clone the repository
git clone https://github.com/haruki7049/kakei.git
cd kakei

# Enter development shell
nix develop

# Everything is set up - start developing!
cargo build
```

The Nix development shell provides:

- Correct Rust toolchain
- All required dependencies
- Consistent development environment across machines

## Building

### Debug Build

For development, use debug builds (faster compilation, includes debug symbols):

```bash
cargo build
```

The binary will be at `target/debug/kakei`.

### Release Build

For production use or performance testing:

```bash
cargo build --release
```

The binary will be at `target/release/kakei`.

### Build Specific Crate

Build only a specific crate in the workspace:

```bash
# Build just the Lisp interpreter
cargo build --package kakei_lisp

# Build the database layer
cargo build --package kakei_database
```

### Build All Workspace Members

```bash
cargo build --workspace
```

## Running Tests

### Run All Tests

```bash
cargo test --workspace
```

This runs:

- Unit tests in each crate
- Integration tests in `tests/`
- Doc tests in documentation comments

### Run Tests for Specific Crate

```bash
# Test the Lisp interpreter
cargo test --package kakei_lisp

# Test the database layer
cargo test --package kakei_database

# Test the processor
cargo test --package kakei_processor
```

### Run with Output

See test output (normally hidden for passing tests):

```bash
cargo test -- --nocapture
```

### Run Specific Test

```bash
cargo test test_name
```

### Run Tests in Parallel

By default, Cargo runs tests in parallel. To run serially:

```bash
cargo test -- --test-threads=1
```

### Doc Tests

Run only documentation tests:

```bash
cargo test --doc
```

## Linting

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

## Code Quality

### Check Without Building

Quickly check for errors without producing binaries:

```bash
cargo check --workspace
```

This is much faster than `cargo build` and useful for quick iteration.

### Check with All Features

```bash
cargo check --workspace --all-features
```

## Running Locally

### Run Development Binary

```bash
# Run the debug binary directly
cargo run -- --help

# Initialize database
cargo run -- init

# Add a transaction
cargo run -- add --date 2025-01-01 --amount -1000 --category Food --account Cash

# List transactions
cargo run -- list
```

### Run with Different Database

Use environment variables to test with a different database:

```bash
# Use a test database
XDG_DATA_HOME=/tmp/kakei-test cargo run -- init
XDG_DATA_HOME=/tmp/kakei-test cargo run -- list
```

## Development Workflow

### Recommended Workflow

1. **Create a feature branch:**

   ```bash
   git checkout -b feature/my-new-feature
   ```

1. **Make changes** and test frequently:

   ```bash
   # Quick check for errors
   cargo check

   # Run tests
   cargo test

   # Run clippy
   cargo clippy
   ```

1. **Format code before committing:**

   ```bash
   cargo fmt
   ```

1. **Run full test suite:**

   ```bash
   cargo test --workspace
   cargo clippy --workspace
   ```

1. **Commit changes:**

   ```bash
   git add .
   git commit -m "Add new feature"
   ```

1. **Push and create PR:**

   ```bash
   git push origin feature/my-new-feature
   ```

### Iterative Development

For rapid iteration:

```bash
# Terminal 1: Watch for changes and rebuild
cargo watch -x check

# Terminal 2: Make changes in your editor

# Terminal 3: Run tests manually
cargo test
```

Install `cargo-watch`:

```bash
cargo install cargo-watch
```

## Debugging

### Debug with rust-gdb

```bash
# Build with debug symbols
cargo build

# Run with debugger
rust-gdb target/debug/kakei
```

### Debug with LLDB

```bash
rust-lldb target/debug/kakei
```

### Print Debugging

Use `dbg!` macro for quick debugging:

```rust
fn process_transaction(tx: Transaction) {
    dbg!(&tx);
    // ... rest of code
}
```

Use `tracing` for structured logging:

```rust
use tracing::{info, debug, error};

debug!("Processing transaction: {:?}", tx);
info!("Transaction added successfully");
```

Enable trace logging:

```bash
RUST_LOG=debug cargo run -- list
```

## Working with Specific Components

### Lisp Interpreter Development

When working on the Lisp interpreter:

1. **Run Lisp tests:**

   ```bash
   cargo test --package kakei_lisp
   ```

1. **Test parsing:**

   ```rust
   // In tests
   use kakei_lisp::parser::parse;

   #[test]
   fn test_parse_expression() {
       let input = "(cons 1 2)";
       let result = parse(input);
       assert!(result.is_ok());
   }
   ```

1. **Test evaluation:**

   ```rust
   use kakei_lisp::eval::eval;

   #[test]
   fn test_eval() {
       let expr = /* parsed expression */;
       let result = eval(&expr, &mut env);
       assert_eq!(result, expected);
   }
   ```

### Database Development

When working on database layer:

1. **Use temporary database for tests:**

   ```rust
   use tempfile::tempdir;

   #[tokio::test]
   async fn test_transaction_insert() {
       let temp = tempdir().unwrap();
       let db_path = temp.path().join("test.db");
       // ... test with db_path
   }
   ```

1. **Check migrations:**

   ```bash
   # Ensure migrations work
   cargo run -- init
   ```

### Money Type Development

When working on money types:

1. **Run money tests:**

   ```bash
   cargo test --package kakei_money
   ```

1. **Test currency formatting:**

   ```rust
   use kakei_money::Money;

   #[test]
   fn test_jpy_formatting() {
       let money = Money::jpy(1000);
       assert_eq!(money.format(), "¥1000");
   }
   ```

## Continuous Integration

The repository includes CI workflows in `.github/workflows/`:

- `rust-ci.yml`: Rust build, test, and lint
- `nix-checker.yml`: Nix build verification

### Local CI Simulation

Run the same checks that CI runs:

```bash
# Build
cargo build --workspace

# Test
cargo test --workspace

# Check
cargo check --workspace

# Lint
cargo clippy --workspace

# Format check
cargo fmt --check
```

## Performance Profiling

### Benchmark with Criterion

(Note: Add Criterion benchmarks if needed)

```bash
cargo bench
```

### Profile with Flamegraph

```bash
# Install cargo-flamegraph
cargo install flamegraph

# Generate flamegraph
cargo flamegraph
```

## Documentation

### Generate Documentation

```bash
# Generate and open docs
cargo doc --open

# Generate docs for all crates
cargo doc --workspace --no-deps
```

### Test Documentation Examples

```bash
cargo test --doc
```

### Build mdBook Documentation

```bash
# Install mdbook if needed
cargo install mdbook

# Build documentation
cd book
mdbook build

# Serve locally
mdbook serve
```

See [Contributing](./contributing.md) for more on documentation.

## Useful Cargo Commands

| Command | Purpose |
|---------|---------|
| `cargo check` | Quick error checking |
| `cargo build` | Build debug binary |
| `cargo build --release` | Build optimized binary |
| `cargo test` | Run tests |
| `cargo clippy` | Run linter |
| `cargo fmt` | Format code |
| `cargo doc` | Generate documentation |
| `cargo clean` | Remove build artifacts |
| `cargo update` | Update dependencies |
| `cargo tree` | Show dependency tree |

## Troubleshooting

### Build Errors

**Problem:** Compilation errors about missing SQLite.

**Solution:**

```bash
# Ubuntu/Debian
sudo apt-get install libsqlite3-dev

# macOS
brew install sqlite3

# Or use Nix
nix develop
```

**Problem:** Wrong Rust version.

**Solution:**

```bash
# Install correct version
rustup install 1.91.1

# Or let rust-toolchain.toml handle it
cargo build
```

### Test Failures

**Problem:** Database tests fail with "database locked".

**Solution:**

- Run tests serially: `cargo test -- --test-threads=1`
- Use different test databases
- Clean up test databases in `drop` implementations

### Performance Issues

**Problem:** Debug builds are slow.

**Solution:**

- Use release builds for performance testing: `cargo build --release`
- Profile with `cargo flamegraph`
- Check database query performance

## See Also

- [Architecture](./architecture.md) - Understanding the codebase
- [Contributing](./contributing.md) - Contributing guidelines
- [Repository](https://github.com/haruki7049/kakei) - Source code
