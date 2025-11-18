# kakei

A kakeibo (household financial ledger) CLI application with powerful Lisp-based table transformations.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 📚 Documentation

**Full documentation is available at: [https://haruki7049.github.io/kakei/](https://haruki7049.github.io/kakei/)**

- [Installation Guide](https://haruki7049.github.io/kakei/installation.html)
- [Quick Start](https://haruki7049.github.io/kakei/quick-start.html)
- [Command Reference](https://haruki7049.github.io/kakei/commands.html)
- [Lisp Functions](https://haruki7049.github.io/kakei/lisp-functions.html)
- [Examples](https://haruki7049.github.io/kakei/examples.html)
- [Contributing Guide](https://haruki7049.github.io/kakei/contributing.html)

## Overview

`kakei` is a command-line application for managing personal finances using the Japanese kakeibo (家計簿) method. It provides transaction tracking, categorization, and powerful Lisp-based table transformations for flexible analysis and reporting.

### Key Features

- 📊 **Transaction Management** - Add, list, and manage financial transactions
- 🏷️ **Category & Account Organization** - Organize by customizable categories and accounts
- 🔄 **Lisp-Based Transformations** - Transform and analyze data using a Lisp dialect
- 📋 **Beautiful Tables** - Formatted output using the `tabled` crate
- 💾 **SQLite Database** - Persistent storage with automatic migrations
- ⚙️ **Configurable** - Customize categories and accounts via TOML config

## Quick Installation

### Using Cargo

```bash
cargo install --path .
```

### Using Nix

```bash
nix profile install github:haruki7049/kakei
```

### From Source

```bash
git clone https://github.com/haruki7049/kakei.git
cd kakei
cargo build --release
```

**📖 For detailed installation instructions, see the [Installation Guide](https://haruki7049.github.io/kakei/installation.html).**

## Quick Start

```bash
# Initialize database
kakei init

# Add transactions
kakei add --date 2025-01-01 --amount -1000 --category Food --account Cash
kakei add --date 2025-01-15 --amount 50000 --category Salary --account Bank

# List transactions
kakei list

# Transform with Lisp
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
```

**📖 For a detailed walkthrough, see the [Quick Start Guide](https://haruki7049.github.io/kakei/quick-start.html).**

## Documentation

All detailed documentation has been moved to the **[kakei Documentation Book](https://haruki7049.github.io/kakei/)**:

- **[Commands Reference](https://haruki7049.github.io/kakei/commands.html)** - Complete command documentation
- **[Data Format](https://haruki7049.github.io/kakei/data-format.html)** - Understanding transaction data structure
- **[Lisp Functions](https://haruki7049.github.io/kakei/lisp-functions.html)** - All available Lisp functions
- **[Configuration](https://haruki7049.github.io/kakei/configuration.html)** - Customizing kakei
- **[Architecture](https://haruki7049.github.io/kakei/architecture.html)** - Technical architecture
- **[Examples](https://haruki7049.github.io/kakei/examples.html)** - Real-world usage examples
- **[Development](https://haruki7049.github.io/kakei/development.html)** - Building and testing
- **[Contributing](https://haruki7049.github.io/kakei/contributing.html)** - Contribution guidelines

## Building the Documentation

The documentation is built with [mdBook](https://rust-lang.github.io/mdBook/):

```bash
# Install mdBook
cargo install mdbook

# Build and serve locally
cd book
mdbook serve

# Open http://localhost:3000 in your browser
```

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! See the [Contributing Guide](https://haruki7049.github.io/kakei/contributing.html) for details.

## Author

haruki7049 <tontonkirikiri@gmail.com>

## Repository

https://github.com/haruki7049/kakei
