# Running Tests

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
