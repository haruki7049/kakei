# kakei

A kakeibo (household financial ledger) CLI application with powerful Lisp-based table transformations.

## Features

- **Transaction Management**: Add, list, and manage financial transactions
- **Category & Account Organization**: Organize transactions by categories and accounts
- **Lisp-Based Transformations**: Use Lisp programs to transform and analyze your transaction data

## Quick Start

### Initialize the database

```bash
kakei init
```

### Add transactions

```bash
kakei add --date 2025-01-01 --amount -1000 --category Food --account Cash
kakei add --date 2025-01-02 --amount -2000 --category Transport --account Cash
```

### List transactions

```bash
kakei list
```

### Transform transactions with Lisp

View raw table data:

```bash
kakei transform --program "table"
```

Group transactions by category:

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
```

## Table Data Format

Transactions are represented as association lists in Lisp format:

```lisp
(ID-001 . ((date . "2025-01-01")
           (amount . -1000)
           (category . "Food")
           (account . "Cash")
           (memo . "")))
```

## Transformation Examples

### Group by Category

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
```

This groups all transactions by their category, producing output like:

```lisp
(("Food" (ID-001 ...) (ID-002 ...))
 ("Transport" (ID-003 ...) (ID-004 ...)))
```

### Filter Transactions

You can use Lisp's built-in functions to filter and manipulate the table:

```bash
# Define a filter function and apply it
kakei transform --program "(define filtered (lambda (t) (if (null? t) () (if (equal? (cdr (assoc 'category (cdr (car t)))) \"Food\") (cons (car t) (filtered (cdr t))) (filtered (cdr t)))))) (filtered table)"
```

## Available Lisp Functions

The `kakei_lisp` dialect supports:

- **Core functions**: `lambda`, `define`, `if`, `cons`, `car`, `cdr`
- **Comparison**: `equal?`, `null?`
- **Association lists**: `assoc` (for accessing fields by name)
- **Table manipulation**: `group-by` (for splitting tables by key)

See the [kakei_lisp documentation](crates/processor/crates/klisp/README.md) for more details.

## Configuration

Configuration file location: `~/.config/kakei/config.toml`

Default categories and accounts can be customized in the configuration file.
