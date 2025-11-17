# kakei

A kakeibo (household financial ledger) CLI application with powerful Lisp-based table transformations.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

`kakei` is a command-line interface application for managing personal finances using the Japanese kakeibo (家計簿) method. It provides transaction tracking, categorization, and powerful data transformation capabilities through an embedded Lisp dialect.

## Features

- 📊 **Transaction Management**: Add, list, and manage financial transactions
- 🏷️ **Category & Account Organization**: Organize transactions by customizable categories and accounts
- 🔄 **Lisp-Based Transformations**: Transform and analyze transaction data using a Lisp dialect
- 📋 **Table Display**: Beautiful table formatting using the `tabled` crate
- 💾 **SQLite Database**: Persistent storage with automatic migrations
- ⚙️ **Configuration**: Customizable categories and accounts

## Installation

### Prerequisites

- Rust 1.91.1 or later
- SQLite 3

### Build from source

```bash
git clone https://github.com/haruki7049/kakei.git
cd kakei
cargo build --release
```

The binary will be available at `target/release/kakei`.

## Quick Start

### 1. Initialize the database

```bash
kakei init
```

This creates the database at `~/.local/share/kakei/kakei.db` and initializes default categories and accounts.

### 2. Add transactions

```bash
# Add an expense
kakei add --date 2025-01-01 --amount -1000 --category Food --account Cash

# Add an expense with memo
kakei add --date 2025-01-02 --amount -2000 --category Transport --account Cash --memo "Train pass"

# Add income
kakei add --date 2025-01-15 --amount 50000 --category Salary --account Bank --memo "Monthly salary"
```

### 3. List transactions

```bash
kakei list
```

Output:

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

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
```

## Commands

### `kakei init`

Initialize the database and configuration.

**Usage:**

```bash
kakei init
```

**Description:**

- Creates the database file at `~/.local/share/kakei/kakei.db`
- Runs database migrations
- Initializes default categories: Food, Transport, Daily Goods, Hobby, Salary
- Initializes default accounts: Cash, Bank

### `kakei add`

Add a new transaction.

**Usage:**

```bash
kakei add --date <DATE> --amount <AMOUNT> --category <CATEGORY> --account <ACCOUNT> [--currency <CURRENCY>] [--memo <MEMO>]
```

**Arguments:**

- `--date <DATE>`: Transaction date in YYYY-MM-DD format (required)
- `--amount <AMOUNT>`: Transaction amount (negative for expenses, positive for income) (required)
- `--category <CATEGORY>`: Category name (required)
- `--account <ACCOUNT>`: Account name (required)
- `--currency <CURRENCY>`: Currency code (default: JPY)
- `--memo <MEMO>`: Optional memo/note

**Examples:**

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

**Usage:**

```bash
kakei list
```

**Description:**

- Displays the 20 most recent transactions
- Shows: Date, Amount, Category, Account, Memo
- Formatted as a rounded table with proper currency symbols

### `kakei transform`

Transform and analyze transactions using Lisp programs.

**Usage:**

```bash
kakei transform --program <LISP_PROGRAM>
```

**Arguments:**

- `--program <LISP_PROGRAM>`: Lisp expression to transform the transaction table (required)

**Examples:**

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

**Fields:**

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

Location: `~/.config/kakei/config.toml`

**Example:**

```toml
default_categories = ["Food", "Transport", "Daily Goods", "Hobby", "Salary"]
default_accounts = ["Cash", "Bank"]
```

### Database Location

The SQLite database is stored at: `~/.local/share/kakei/kakei.db`

This follows the XDG Base Directory specification on Linux.

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
