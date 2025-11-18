# Contributing to kakei

Thank you for your interest in contributing to kakei! This document provides guidelines for contributing to the project.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Environment Setup](#development-environment-setup)
- [Building and Testing](#building-and-testing)
- [Code Style and Quality](#code-style-and-quality)
- [Submitting Changes](#submitting-changes)
- [Issue Reporting](#issue-reporting)
- [Pull Request Process](#pull-request-process)
- [Communication](#communication)

## Getting Started

kakei is a command-line application for managing personal finances using the Japanese kakeibo (家計簿) method. The project is written in Rust and consists of multiple crates:

- `kakei`: Main CLI application
- `kakei_processor`: Business logic and table transformations
- `kakei_database`: Database layer with SQLite
- `kakei_money`: Money type with currency support
- `kakei_lisp`: Embedded Lisp dialect (parser and evaluator)

## Development Environment Setup

### Prerequisites

- **Rust**: Version 1.91.1 or later
- **SQLite 3**: Required for database operations
- **Git**: For version control

### Option A: Using Nix (Recommended)

The project provides a Nix flake for reproducible development environments:

```bash
# Clone the repository
git clone https://github.com/haruki7049/kakei.git
cd kakei

# Enter the development shell
nix develop

# All development tools will be available
```

### Option B: Using Cargo

If you prefer not to use Nix:

```bash
# Clone the repository
git clone https://github.com/haruki7049/kakei.git
cd kakei

# Ensure you have Rust 1.91.1 or later
rustc --version

# Install the project dependencies
cargo fetch
```

## Building and Testing

### Building

```bash
# Debug build (faster compilation)
cargo build

# Release build (optimized)
cargo build --release

# Build all workspace crates
cargo build --workspace
```

The binary will be available at:
- Debug: `target/debug/kakei`
- Release: `target/release/kakei`

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for a specific crate
cargo test --package kakei_processor
cargo test --package kakei_database
cargo test --package kakei_lisp

# Run tests with output
cargo test -- --nocapture

# Run tests with verbose output
cargo test --workspace --verbose
```

### Linting

The project uses `nix flake check` to run linters and formatter checks. This command runs all checks defined in the flake, including builds, Clippy, and formatting validation.

```bash
# Run all checks (linters, formatters, builds, tests)
nix flake check --all-systems --print-build-logs

# For a quick check on current system only
nix flake check --print-build-logs

# If you're not using Nix, you can run Clippy individually
cargo clippy --workspace

# Fix clippy warnings automatically (where possible)
cargo clippy --workspace --fix
```

### Formatting

The project uses `nix fmt` as the primary formatting command, which internally uses `treefmt-nix`. This Nix library allows you to run multiple formatters (rustfmt, nixfmt-rfc-style, and others) with a single command.

```bash
# Format all code (Rust, Nix, SQL, TOML, Markdown, Shell scripts)
nix fmt

# If you're not using Nix, you can format Rust code individually
cargo fmt --all
```

The `nix fmt` command formats:
- Rust code with rustfmt
- Nix code with nixfmt-rfc-style
- SQL with sql-formatter
- TOML with taplo
- Markdown with mdformat
- Shell scripts with shfmt

## Code Style and Quality

### Code Style

- Follow the Rust style guide and community conventions
- Use `nix fmt` to format your code before committing (formats Rust, Nix, SQL, TOML, Markdown, and Shell scripts)
- If not using Nix, use `cargo fmt --all` for Rust code
- Ensure your code passes `cargo clippy` without warnings
- Write clear, self-documenting code with appropriate comments where necessary

### Testing

- Write tests for new functionality
- Ensure existing tests pass before submitting changes
- Aim for good test coverage of critical paths
- Use descriptive test names that explain what is being tested

### Documentation

- Add doc comments (`///`) for public APIs
- Update the README.md if you add new features or change behavior
- Include examples in doc comments where helpful

### Commit Messages

- Write clear, concise commit messages
- Use the present tense ("Add feature" not "Added feature")
- Start with a capital letter
- Limit the first line to 72 characters or less
- Reference issue numbers when applicable (e.g., "Fix #123")

Example:
```
Add support for EUR currency

- Implement EUR currency handling in kakei_money
- Add tests for EUR formatting
- Update documentation

Fixes #123
```

## Submitting Changes

### Before You Start

1. Check existing issues and pull requests to avoid duplicate work
2. For large changes, consider opening an issue first to discuss the approach
3. Fork the repository and create a new branch for your changes

### Making Changes

1. Create a feature branch from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make your changes, following the code style guidelines

3. Add tests for your changes

4. Run tests, linting, and formatting:
   ```bash
   # Using Nix (recommended - runs all checks at once)
   nix flake check --all-systems --print-build-logs
   
   # Or run commands individually
   cargo test --workspace
   cargo clippy --workspace
   nix fmt  # or 'cargo fmt --all' if not using Nix
   ```

5. Commit your changes with clear commit messages

6. Push your branch to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```

## Issue Reporting

### Bug Reports

When reporting bugs, please include:

- **Description**: Clear description of the issue
- **Steps to reproduce**: Step-by-step instructions to reproduce the bug
- **Expected behavior**: What you expected to happen
- **Actual behavior**: What actually happened
- **Environment**: OS, Rust version, kakei version
- **Logs/Output**: Relevant error messages or logs

Example:
```markdown
**Description**: kakei crashes when adding a transaction with an invalid date

**Steps to reproduce**:
1. Run `kakei init`
2. Run `kakei add --date invalid --amount -1000 --category Food --account Cash`

**Expected**: Error message about invalid date format
**Actual**: Application panics with stack trace

**Environment**:
- OS: Ubuntu 22.04
- Rust: 1.91.1
- kakei: 0.1.0
```

### Feature Requests

When requesting features, please include:

- **Description**: Clear description of the feature
- **Use case**: Why this feature would be useful
- **Proposed solution**: If you have ideas on how to implement it
- **Alternatives**: Other approaches you've considered

## Pull Request Process

1. **Run all checks**: Run `nix flake check --all-systems --print-build-logs` (or run tests, linting, and formatting individually if not using Nix)
2. **Update documentation**: Update README.md or other docs if needed
5. **Write a clear PR description**:
   - Describe what your changes do
   - Reference related issues
   - Explain any design decisions
   - List any breaking changes

6. **Respond to review feedback**: Be open to suggestions and make requested changes

7. **Wait for CI**: The GitHub Actions CI will run tests on multiple platforms (Ubuntu, macOS, Windows)

### PR Checklist

- [ ] All checks pass: `nix flake check --all-systems --print-build-logs` (or individual commands if not using Nix)
- [ ] Code builds without errors
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Code is formatted with `nix fmt` (or `cargo fmt --all` if not using Nix)
- [ ] Documentation is updated (if applicable)
- [ ] Tests are added for new functionality
- [ ] Commit messages are clear and descriptive
- [ ] PR description clearly explains the changes

## Communication

### Where to Ask Questions

- **GitHub Issues**: For bug reports and feature requests
- **GitHub Discussions**: For general questions and discussions (if enabled)
- **Pull Request Comments**: For questions about specific code changes

### Community Guidelines

- Be respectful and inclusive
- Follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct)
- Help others when you can
- Assume good intentions

## Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- [Nix Flakes](https://nixos.wiki/wiki/Flakes)

## License

By contributing to kakei, you agree that your contributions will be licensed under the MIT License, the same license as the project.

## Questions?

If you have any questions about contributing, feel free to open an issue or reach out to the maintainer:

**Maintainer**: haruki7049 <tontonkirikiri@gmail.com>

Thank you for contributing to kakei! 🎉
