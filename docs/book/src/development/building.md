# Building

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
