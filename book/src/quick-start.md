# Quick Start

This guide will walk you through the basic workflow of using kakei to manage your finances.

## Initialize Database

Before you can start tracking transactions, you need to initialize the database:

```bash
kakei init
```

This command:

- Creates the database file at the appropriate location for your platform
  - Linux (XDG): `~/.local/share/kakei/kakei.db`
  - macOS: `~/Library/Application Support/kakei/kakei.db`
  - Windows: `%APPDATA%\kakei\kakei.db`
- Runs database migrations to set up the schema
- Initializes default categories: **Food**, **Transport**, **Daily Goods**, **Hobby**, **Salary**
- Initializes default accounts: **Cash**, **Bank**

You only need to run this command once when you first start using kakei.

## Add Transactions

Now you can start adding financial transactions. The `kakei add` command requires several pieces of information:

### Basic Expense

Record a simple expense:

```bash
kakei add --date 2025-01-01 --amount -1000 --category Food --account Cash
```

### Expense with Memo

Add a note to help remember what the transaction was for:

```bash
kakei add --date 2025-01-02 --amount -2000 --category Transport --account Cash --memo "Train pass"
```

### Recording Income

Positive amounts represent income:

```bash
kakei add --date 2025-01-15 --amount 50000 --category Salary --account Bank --memo "Monthly salary"
```

### Different Currency

By default, kakei uses JPY (Japanese Yen), but you can specify other currencies:

```bash
kakei add --date 2025-01-20 --amount -50 --category Food --account Cash --currency USD
```

### Understanding Amounts

The `--amount` parameter expects values in the currency's **minor units** as an integer:

- **For JPY** (no subunits): Use integer yen

  - `-1000` represents ¥-1,000 (expense)
  - `50000` represents ¥50,000 (income)

- **For USD, EUR, etc.**: Use cents

  - `-1050` represents $-10.50 (expense)
  - `50000` represents $500.00 (income)

**Negative amounts** = expenses (money going out)\
**Positive amounts** = income (money coming in)

## List Transactions

View your recent transactions in a formatted table:

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

The `list` command displays the **20 most recent transactions** by default, showing:

- Date
- Amount (with currency symbol)
- Category
- Account
- Memo

## Transform with Lisp

One of kakei's most powerful features is the ability to transform and analyze your transaction data using Lisp expressions.

### View All Transactions

```bash
kakei transform --program "table"
```

The `table` variable contains all your transactions as a Lisp data structure.

### Group by Category

See spending broken down by category:

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
```

### Group by Account

See transactions grouped by account:

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'account (cdr pair)))))"
```

### Get First Transaction

```bash
kakei transform --program "(cons (car table) ())"
```

### Skip First Transaction

```bash
kakei transform --program "(cdr table)"
```

### Get First Two Transactions

```bash
kakei transform --program "(cons (car table) (cons (car (cdr table)) ()))"
```

## Shell Quoting Notes

When passing Lisp programs on the command line, you may need to properly quote or escape the program string depending on your shell:

**POSIX shells (bash, zsh, etc.):**

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
```

**PowerShell:**

```powershell
kakei transform --program '(group-by table (lambda (pair) (cdr (assoc ''category'' (cdr pair)))))'
```

If you encounter quoting issues, consider putting your Lisp program in a file and reading from it.

## Next Steps

Now that you understand the basics, explore:

- [Commands](./commands.md) - Complete reference for all commands and options
- [Lisp Functions](./lisp-functions.md) - Learn all available Lisp functions for data transformation
- [Examples](./examples.md) - Real-world usage examples for common scenarios
- [Configuration](./configuration.md) - Customize categories, accounts, and more
