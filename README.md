# kakei

A kakeibo (household financial ledger) CLI application with powerful Lisp-based table transformations.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
  - [Prerequisites](#prerequisites)
  - [Option A — Build from source](#option-a--build-from-source)
  - [Option B — Install via cargo (local)](#option-b--install-via-cargo-local)
  - [Option C — Releases](#option-c--releases)
  - [Option D — Install via Nix flakes](#option-d--install-via-nix-flakes)
  - [Platform notes](#platform-notes)
- [Quick Start](#quick-start)
  - [1. Initialize the database](#1-initialize-the-database)
  - [2. Add transactions](#2-add-transactions)
  - [3. List transactions](#3-list-transactions)
  - [4. Transform transactions with Lisp](#4-transform-transactions-with-lisp)
- [Commands](#commands)
  - [`kakei init`](#kakei-init)
  - [`kakei add`](#kakei-add)
  - [`kakei list`](#kakei-list)
  - [`kakei transform`](#kakei-transform)
- [Data Format](#data-format)
  - [Transaction Structure](#transaction-structure)
  - [Table Structure](#table-structure)
- [Lisp Functions](#lisp-functions)
- [Configuration](#configuration)
  - [Configuration File](#configuration-file)
  - [Database Location](#database-location)
- [Internationalization (i18n)](#internationalization-i18n)
  - [Supported Languages](#supported-languages)
  - [Changing the Language](#changing-the-language)
  - [Adding New Translations](#adding-new-translations)
- [Architecture](#architecture)
- [Development](#development)
  - [Running Tests](#running-tests)
  - [Linting](#linting)
  - [Building](#building)
- [Examples](#examples)
  - [Monthly Budget Tracking](#monthly-budget-tracking)
  - [Analyzing Spending Patterns](#analyzing-spending-patterns)
- [Contributing](#contributing)
- [License](#license)
- [Author](#author)
- [Repository](#repository)

## Overview

`kakei` is a command-line application for managing personal finances using the Japanese kakeibo (家計簿) method. It provides transaction tracking, categorization, and powerful Lisp-based table transformations for flexible analysis and reporting.

## Features

- 📊 Transaction Management: Add, list, and manage financial transactions
- 🏷️ Category & Account Organization: Organize transactions by customizable categories and accounts
- 🔄 Lisp-Based Transformations: Transform and analyze transaction data using a small Lisp dialect
- 📋 Table Display: Beautiful table formatting using the `tabled` crate
- 💾 SQLite Database: Persistent storage with automatic migrations
- ⚙️ Configuration: Customizable categories and accounts
- 🌍 Internationalization: Built-in support for multiple languages with compile-time embedded translations (English, Japanese)

## Installation

### Prerequisites

- Rust 1.91.1 or later (for building from source)
- SQLite 3

### Option A — Build from source

```bash
git clone https://github.com/haruki7049/kakei.git
cd kakei
cargo build --release
```

The binary will be available at `target/release/kakei`. You can copy it to a directory on your PATH (e.g., `/usr/local/bin`) if desired.

### Option B — Install via cargo (local)

From the repository root:

```bash
cargo install --path .
```

This installs `kakei` to your cargo bin directory (usually `~/.cargo/bin`).

### Option C — Releases

If release binaries are published on GitHub Releases, you can download the appropriate archive for your platform and unpack the `kakei` binary.

### Option D — Install via Nix flakes

If you have Nix with flakes enabled, you can install `kakei` directly from the repository:

```bash
# Install from the flake
nix profile install github:haruki7049/kakei

# Or run directly without installing
nix run github:haruki7049/kakei -- --help

# For local development
nix develop
```

The flake provides:

- `packages.default`: The kakei binary
- `devShells.default`: Development environment with Rust toolchain and dependencies

### Platform notes

- On Linux `kakei` follows the XDG spec for config and data directories (see Configuration section).
- On macOS/Windows, the app will fall back to reasonable defaults if XDG variables are not set (see Configuration section).

## Quick Start

### 1. Initialize the database

```bash
kakei init
```

This creates the database at `~/.local/share/kakei/kakei.db` (on Linux when XDG is used) and initializes default categories and accounts.

### 2. Add transactions

Example usages:

```bash
# Add an expense
kakei add --date 2025-01-01 --amount -1000 --category Food --account Cash

# Add an expense with memo
kakei add --date 2025-01-02 --amount -2000 --category Transport --account Cash --memo "Train pass"

# Add income
kakei add --date 2025-01-15 --amount 50000 --category Salary --account Bank --memo "Monthly salary"
```

Note on amounts:

- `--amount` expects the value in the currency's minor units as an integer.
- For JPY (no subunits), use integer yen (e.g., `-1000` represents ¥-1000).
- For other currencies, follow that currency's minor unit convention (e.g., cents for USD).

### 3. List transactions

```bash
kakei list
```

Example output:

```
╭────────────┬────────┬───────────┬─────────┬──────────────╮
│ Date       │ Amount │ Category  │ Account │ Memo         │
├────────────┼────────┼───────────┼─────────┼──────────────┤
│ 2025-01-15 │ ¥50000 │ Salary    │ Bank    │ Monthly sal… │
│ 2025-01-02 │ ¥-2000 │ Transport │ Cash    │ Train pass   │
│ 2025-01-01 │ ¥-1000 │ Food      │ Cash    │              │
╰────────────┴────────┴───────────┴─────────┴──────────────╯
```

### 4. Transform transactions with Lisp

View all transactions as a table:

```bash
kakei transform --program "table"
```

Group transactions by category:

Note about quoting:

- When passing Lisp programs on a shell command line, ensure correct quoting/escaping.
- Example for POSIX shells where the program contains parentheses and single quotes:

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
```

If you run into shell quoting issues, put the program in a file and read from it or use alternate quoting.

## Commands

### `kakei init`

Initialize the database and configuration.

Usage:

```bash
kakei init
```

Description:

- Creates the database file (default: `~/.local/share/kakei/kakei.db` on Linux when XDG is available)
- Runs database migrations
- Initializes default categories: Food, Transport, Daily Goods, Hobby, Salary
- Initializes default accounts: Cash, Bank

### `kakei add`

Add a new transaction.

Usage:

```bash
kakei add --date <DATE> --amount <AMOUNT> --category <CATEGORY> --account <ACCOUNT> [--currency <CURRENCY>] [--memo <MEMO>]
```

Arguments:

- `--date <DATE>`: Transaction date in YYYY-MM-DD format (required)
- `--amount <AMOUNT>`: Transaction amount in minor units (negative for expenses, positive for income) (required)
- `--category <CATEGORY>`: Category name (required)
- `--account <ACCOUNT>`: Account name (required)
- `--currency <CURRENCY>`: Currency code (default: JPY)
- `--memo <MEMO>`: Optional memo/note

Examples:

```bash
# Simple expense
kakei add --date 2025-01-01 --amount -1000 --category Food --account Cash

# Expense with memo
kakei add --date 2025-01-02 --amount -2000 --category Transport --account Cash --memo "Monthly train pass"

# Income
kakei add --date 2025-01-15 --amount 50000 --category Salary --account Bank

# USD transaction
kakei add --date 2025-01-20 --amount -50 --category Food --account Cash --currency USD
```

### `kakei list`

List recent transactions in a formatted table.

Usage:

```bash
kakei list
```

Description:

- Displays the 20 most recent transactions
- Shows: Date, Amount, Category, Account, Memo
- Formatted as a rounded table with proper currency symbols

### `kakei transform`

Transform and analyze transactions using Lisp programs.

Usage:

```bash
kakei transform --program <LISP_PROGRAM>
```

Arguments:

- `--program <LISP_PROGRAM>`: Lisp expression to transform the transaction table (required)

Examples:

View all transactions:

```bash
kakei transform --program "table"
```

Group by category:

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
```

Group by account:

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'account (cdr pair)))))"
```

Get first transaction only:

```bash
kakei transform --program "(cons (car table) ())"
```

Skip first transaction:

```bash
kakei transform --program "(cdr table)"
```

Get first two transactions:

```bash
kakei transform --program "(cons (car table) (cons (car (cdr table)) ()))"
```

## Data Format

### Transaction Structure

Transactions are represented as association lists in Lisp format:

```lisp
(ID-001 . ((date . "2025-01-01")
           (amount . -1000)
           (category . "Food")
           (account . "Cash")
           (memo . "")))
```

Fields:

- `date`: Transaction date (string, YYYY-MM-DD format)
- `amount`: Transaction amount in minor units (integer, e.g., -1000 for ¥-1000)
- `category`: Category name (string)
- `account`: Account name (string)
- `memo`: Optional memo (string, empty if not provided)

### Table Structure

The `table` variable contains a list of transactions:

```lisp
((ID-001 . ((date . "2025-01-01") (amount . -1000) ...))
 (ID-002 . ((date . "2025-01-02") (amount . -2000) ...))
 ...)
```

## Lisp Functions

The `kakei_lisp` dialect provides the following built-in functions:

### Core Functions

- **`lambda`**: Create anonymous functions

  ```lisp
  (lambda (x) (+ x 1))
  ```

- **`define`**: Define variables or functions

  ```lisp
  (define x 42)
  ```

- **`if`**: Conditional evaluation

  ```lisp
  (if (null? x) "empty" "not empty")
  ```

### List Operations

- **`cons`**: Construct a pair (cons cell)

  ```lisp
  (cons 1 2)  ; => (1 . 2)
  ```

- **`car`**: Get the first element of a pair

  ```lisp
  (car (cons 1 2))  ; => 1
  ```

- **`cdr`**: Get the second element of a pair

  ```lisp
  (cdr (cons 1 2))  ; => 2
  ```

### Comparison Functions

- **`equal?`**: Test equality

  ```lisp
  (equal? "Food" "Food")  ; => #t
  ```

- **`null?`**: Test if value is nil

  ```lisp
  (null? ())  ; => #t
  ```

### Association List Functions

- **`assoc`**: Find a key in an association list
  ```lisp
  (assoc 'category '((date . "2025-01-01") (category . "Food")))
  ; => (category . "Food")
  ```

### Table Manipulation

- **`group-by`**: Group a table by a key function

  ```lisp
  (group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))
  ```

  Returns a list of groups: `(("GroupName" (row1) (row2) ...) ...)`

## Configuration

### Configuration File

Location: `~/.config/kakei/config.toml` (on Linux when XDG is used)

Example:

```toml
default_categories = ["Food", "Transport", "Daily Goods", "Hobby", "Salary"]
default_accounts = ["Cash", "Bank"]
```

### Database Location

The SQLite database is stored at: `~/.local/share/kakei/kakei.db` (on Linux when XDG is used)

This follows the XDG Base Directory specification on Linux. On macOS/Windows, the app will use platform-appropriate directories if XDG environment variables are not set.

## Internationalization (i18n)

`kakei` supports multiple languages through embedded locale files. The translations are compiled into the binary at build time, making the application self-contained and usable offline.

### Supported Languages

- **English (en)** - Default language
- **Japanese (ja)** - 日本語サポート

### Changing the Language

You can change the language using one of these methods:

#### Method 1: Using RUST_I18N_LOCALE environment variable

```bash
# English (default)
kakei list

# Japanese
RUST_I18N_LOCALE=ja kakei list
```

#### Method 2: Using LANG environment variable

The application will automatically detect your system locale from the `LANG` environment variable:

```bash
# Set system locale to Japanese
LANG=ja_JP.UTF-8 kakei list

# Set system locale to English
LANG=en_US.UTF-8 kakei list
```

### Adding New Translations

To add support for a new language:

1. Create a new YAML file in the `locales/` directory named `[language_code].yml` (e.g., `fr.yml` for French)
1. Copy the structure from `locales/en.yml` and translate the strings
1. Rebuild the application with `cargo build`

The translations are embedded at compile time, so there's no need to distribute separate locale files.

Example locale file structure:

```yaml
# locales/fr.yml
transaction_added: "✅ Transaction ajoutée avec succès ! (ID: %{id})"
init_complete: "✅ Initialisation terminée. Base de données prête à : %{path}"
# ... other translations
```

## Architecture

The project is organized into multiple crates:

- **`kakei`**: Main CLI application
- **`kakei_processor`**: Business logic and table transformations
- **`kakei_database`**: Database layer with SQLite
- **`kakei_money`**: Money type with currency support
- **`kakei_lisp`**: Embedded Lisp dialect (parser and evaluator)

## Development

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for a specific crate
cargo test --package kakei_processor

# Run with output
cargo test -- --nocapture
```

### Linting

```bash
cargo clippy --workspace
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

## Examples

### Monthly Budget Tracking

```bash
# Add expenses for the month
kakei add --date 2025-01-05 --amount -1200 --category Food --account Cash --memo "Lunch"
kakei add --date 2025-01-10 --amount -3000 --category Transport --account Card --memo "Monthly pass"
kakei add --date 2025-01-15 --amount 50000 --category Salary --account Bank --memo "Salary"
kakei add --date 2025-01-20 --amount -500 --category Hobby --account Cash --memo "Book"

# View all transactions
kakei list

# Group by category to see spending breakdown
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
```

### Analyzing Spending Patterns

```bash
# Group by account to see where money goes
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'account (cdr pair)))))"

# View only the most recent transaction
kakei transform --program "(cons (car table) ())"
```

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Author

haruki7049 <tontonkirikiri@gmail.com>

## Repository

https://github.com/haruki7049/kakei
