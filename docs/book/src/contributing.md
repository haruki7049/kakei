# Contributing

Thank you for your interest in contributing to kakei! This guide will help you get started.

## Ways to Contribute

There are many ways to contribute to kakei:

- **Report bugs** - File issues for bugs you encounter
- **Suggest features** - Share ideas for improvements
- **Improve documentation** - Fix typos, add examples, clarify explanations
- **Write code** - Fix bugs, implement features, optimize performance
- **Write tests** - Add test coverage for existing functionality
- **Review pull requests** - Provide feedback on others' contributions

All contributions are welcome and appreciated!

## Getting Started

### 1. Fork and Clone

1. **Fork the repository** on GitHub
1. **Clone your fork:**
   ```bash
   git clone https://github.com/YOUR_USERNAME/kakei.git
   cd kakei
   ```
1. **Add upstream remote:**
   ```bash
   git remote add upstream https://github.com/haruki7049/kakei.git
   ```

### 2. Set Up Development Environment

See the [Development](./development.md) guide for detailed setup instructions.

Quick start:

```bash
# Option 1: Standard Rust
cargo build

# Option 2: Nix
nix develop
```

### 3. Create a Branch

```bash
git checkout -b feature/my-feature
# or
git checkout -b fix/my-bugfix
```

Branch naming conventions:

- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation changes
- `refactor/` - Code refactoring
- `test/` - Test additions or changes

## Development Workflow

### Making Changes

1. **Make your changes** in the appropriate files
1. **Add tests** for new functionality
1. **Run tests** to ensure nothing breaks:
   ```bash
   cargo test --workspace
   ```
1. **Run linter:**
   ```bash
   cargo clippy --workspace
   ```
1. **Format code:**
   ```bash
   cargo fmt
   ```

### Writing Good Commits

Use clear, descriptive commit messages:

```bash
# Good
git commit -m "Add support for EUR currency in Money type"
git commit -m "Fix database connection leak in transaction list"
git commit -m "docs: Add examples for group-by function"

# Not as good
git commit -m "fix stuff"
git commit -m "wip"
git commit -m "changes"
```

Commit message format:

```
<type>: <short description>

<optional detailed description>

<optional issue references>
```

Types:

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Test additions or changes
- `chore`: Maintenance tasks

### Testing Your Changes

Always test your changes thoroughly:

```bash
# Run all tests
cargo test --workspace

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run clippy
cargo clippy --workspace

# Check formatting
cargo fmt --check
```

For CLI changes, test manually:

```bash
# Build and test
cargo run -- init
cargo run -- add --date 2025-01-01 --amount -1000 --category Food --account Cash
cargo run -- list
```

## Pull Request Process

### 1. Push Your Changes

```bash
git push origin feature/my-feature
```

### 2. Create Pull Request

1. Go to the [kakei repository](https://github.com/haruki7049/kakei)
1. Click "New Pull Request"
1. Select your branch
1. Fill in the PR template (if provided)

### 3. PR Description

Write a clear PR description:

```markdown
## Description
Brief description of what this PR does.

## Changes
- Change 1
- Change 2
- Change 3

## Testing
How you tested the changes.

## Related Issues
Fixes #123
Related to #456
```

### 4. Respond to Reviews

- Be open to feedback
- Make requested changes promptly
- Ask questions if something is unclear
- Update your PR based on review comments

### 5. Merge

Once approved, a maintainer will merge your PR.

## Code Style

### Rust Style

Follow standard Rust conventions:

- Use `rustfmt` for formatting: `cargo fmt`
- Follow Clippy suggestions: `cargo clippy`
- Write doc comments for public APIs
- Use meaningful variable names
- Keep functions focused and small

Example:

````rust
/// Add a new transaction to the database.
///
/// # Arguments
///
/// * `date` - Transaction date in YYYY-MM-DD format
/// * `amount` - Amount in minor units (negative for expenses)
/// * `category` - Transaction category
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if the transaction cannot be added.
///
/// # Examples
///
/// ```
/// let result = add_transaction("2025-01-01", -1000, "Food").await?;
/// ```
pub async fn add_transaction(
    date: &str,
    amount: i64,
    category: &str,
) -> Result<(), Error> {
    // Implementation
}
````

### Documentation Style

- Use clear, concise language
- Provide examples for complex features
- Keep README.md concise - detailed docs go in the book
- Update relevant documentation when changing behavior

## Contributing to Documentation

### mdBook Documentation

The documentation is in `docs/book/src/`:

```bash
# Install mdbook
cargo install mdbook

# Make changes to .md files in docs/book/src/

# Preview locally
cd docs/book
mdbook serve

# Open browser to http://localhost:3000
```

### Documentation Structure

- `introduction.md` - Overview and introduction
- `installation.md` - Installation instructions
- `quick-start.md` - Getting started guide
- `commands.md` - Command reference
- `data-format.md` - Data structure documentation
- `lisp-functions.md` - Lisp function reference
- `configuration.md` - Configuration options
- `examples.md` - Real-world examples
- `architecture.md` - Technical architecture
- `development.md` - Development guide
- `contributing.md` - This file!

### Adding Examples

When adding examples:

1. Make sure they actually work
1. Use realistic scenarios
1. Explain what the example demonstrates
1. Include expected output when relevant

## Areas That Need Help

Current areas where contributions are especially welcome:

### Code

- [ ] Add more Lisp built-in functions (arithmetic, string operations)
- [ ] Implement CSV import/export
- [ ] Add filtering capabilities to `list` command
- [ ] Improve error messages
- [ ] Add more currency support
- [ ] Performance optimizations

### Documentation

- [ ] More examples in the book
- [ ] Video tutorials
- [ ] Translations to other languages
- [ ] FAQ section
- [ ] Troubleshooting guide

### Tests

- [ ] Integration tests for CLI commands
- [ ] More unit tests for Lisp interpreter
- [ ] Property-based testing
- [ ] Performance benchmarks

### Infrastructure

- [ ] Automated releases
- [ ] Pre-built binaries for more platforms
- [ ] Docker image
- [ ] Homebrew formula

## Reporting Bugs

When reporting bugs, include:

1. **Description** - What happened vs. what you expected
1. **Steps to reproduce** - Exact commands to reproduce the bug
1. **Environment:**
   - OS and version
   - Rust version (`rustc --version`)
   - kakei version
1. **Error messages** - Full error output
1. **Additional context** - Logs, screenshots, etc.

Example:

```markdown
## Bug Description
The `list` command crashes when there are more than 100 transactions.

## Steps to Reproduce
1. Add 101 transactions
2. Run `kakei list`
3. Application crashes

## Environment
- OS: Ubuntu 22.04
- Rust: 1.91.1
- kakei: 0.1.0

## Error Message
```

thread 'main' panicked at 'index out of bounds'

```

## Suggesting Features

When suggesting features, explain:

1. **Use case** - What problem does this solve?
2. **Proposed solution** - How should it work?
3. **Alternatives** - Other ways to solve the problem?
4. **Additional context** - Examples, mockups, etc.

## Code Review Guidelines

When reviewing others' PRs:

- Be kind and constructive
- Explain the "why" behind suggestions
- Acknowledge good work
- Ask questions if something is unclear
- Suggest improvements, don't demand them
- Focus on the code, not the person

## Questions?

If you have questions:

1. Check the [documentation](https://haruki7049.github.io/kakei/)
2. Search [existing issues](https://github.com/haruki7049/kakei/issues)
3. Open a new issue with your question
4. Join discussions in existing issues

## License

By contributing, you agree that your contributions will be licensed under the MIT License, the same license as the project.

## Code of Conduct

Be respectful and inclusive:
- Use welcoming and inclusive language
- Be respectful of differing viewpoints and experiences
- Gracefully accept constructive criticism
- Focus on what is best for the community
- Show empathy towards other community members

## Thank You!

Your contributions make kakei better for everyone. Thank you for taking the time to contribute!

## See Also

- [Development Guide](./development.md) - Development setup and workflow
- [Architecture](./architecture.md) - Understanding the codebase
- [Repository](https://github.com/haruki7049/kakei) - Source code
```
