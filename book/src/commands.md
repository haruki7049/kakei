# Commands

Complete reference for all kakei commands.

## init

Initialize the database and configuration.

### Usage

```bash
kakei init
```

### Description

The `init` command sets up kakei for first-time use:

1. **Creates the database file** at the platform-appropriate location:

   - Linux (XDG): `~/.local/share/kakei/kakei.db`
   - macOS: `~/Library/Application Support/kakei/kakei.db`
   - Windows: `%APPDATA%\kakei\kakei.db`

1. **Runs database migrations** to create the necessary tables and schema

1. **Initializes default categories**:

   - Food
   - Transport
   - Daily Goods
   - Hobby
   - Salary

1. **Initializes default accounts**:

   - Cash
   - Bank

### When to Use

- Run this command **once** when you first install kakei
- If you delete your database and want to start fresh
- After moving kakei to a new system

### Example

```bash
$ kakei init
Database initialized successfully!
```

## add

Add a new financial transaction to the database.

### Usage

```bash
kakei add --date <DATE> --amount <AMOUNT> --category <CATEGORY> --account <ACCOUNT> [OPTIONS]
```

### Required Arguments

- `--date <DATE>`

  - Transaction date in **YYYY-MM-DD** format
  - Example: `2025-01-15`

- `--amount <AMOUNT>`

  - Transaction amount in **minor units** (integer)
  - **Negative** for expenses (money out)
  - **Positive** for income (money in)
  - Example: `-1000` (¥-1,000 expense) or `50000` (¥50,000 income)

- `--category <CATEGORY>`

  - Category name for the transaction
  - Must match one of your configured categories
  - Example: `Food`, `Transport`, `Salary`

- `--account <ACCOUNT>`

  - Account name for the transaction
  - Must match one of your configured accounts
  - Example: `Cash`, `Bank`, `Card`

### Optional Arguments

- `--currency <CURRENCY>`

  - Currency code (default: `JPY`)
  - Example: `USD`, `EUR`, `GBP`

- `--memo <MEMO>`

  - Optional note or description
  - Example: `"Monthly train pass"`

### Examples

#### Simple Expense

```bash
kakei add --date 2025-01-01 --amount -1000 --category Food --account Cash
```

#### Expense with Memo

```bash
kakei add --date 2025-01-02 --amount -2000 --category Transport --account Cash --memo "Monthly train pass"
```

#### Recording Income

```bash
kakei add --date 2025-01-15 --amount 50000 --category Salary --account Bank
```

#### USD Transaction

```bash
kakei add --date 2025-01-20 --amount -5000 --category Food --account Cash --currency USD --memo "Dinner in NYC"
```

### Amount Format

The `--amount` parameter uses **minor units** (the smallest subdivision of a currency):

| Currency | Minor Unit | Example Input | Actual Value |
|----------|-----------|---------------|--------------|
| JPY | Yen (no subunit) | `-1000` | ¥-1,000 |
| USD | Cents | `-1050` | $-10.50 |
| EUR | Cents | `-2099` | €-20.99 |
| GBP | Pence | `-1500` | £-15.00 |

## list

Display recent transactions in a formatted table.

### Usage

```bash
kakei list
```

### Description

The `list` command displays your **20 most recent transactions** in a beautifully formatted table with rounded borders.

### Output Format

```
╭────────────┬────────┬───────────┬─────────┬──────────────╮
│ Date       │ Amount │ Category  │ Account │ Memo         │
├────────────┼────────┼───────────┼─────────┼──────────────┤
│ 2025-01-15 │ ¥50000 │ Salary    │ Bank    │ Monthly sal… │
│ 2025-01-02 │ ¥-2000 │ Transport │ Cash    │ Train pass   │
│ 2025-01-01 │ ¥-1000 │ Food      │ Cash    │              │
╰────────────┴────────┴───────────┴─────────┴──────────────╯
```

### Columns

- **Date**: Transaction date (YYYY-MM-DD)
- **Amount**: Formatted amount with currency symbol
- **Category**: Transaction category
- **Account**: Account used
- **Memo**: Optional note (truncated if long)

### Example

```bash
$ kakei list
╭────────────┬────────┬───────────┬─────────┬──────────────╮
│ Date       │ Amount │ Category  │ Account │ Memo         │
├────────────┼────────┼───────────┼─────────┼──────────────┤
│ 2025-01-15 │ ¥50000 │ Salary    │ Bank    │ Monthly sal… │
│ 2025-01-02 │ ¥-2000 │ Transport │ Cash    │ Train pass   │
│ 2025-01-01 │ ¥-1000 │ Food      │ Cash    │              │
╰────────────┴────────┴───────────┴─────────┴──────────────╯
```

## transform

Transform and analyze transaction data using Lisp programs.

### Usage

```bash
kakei transform --program <LISP_PROGRAM>
```

### Required Arguments

- `--program <LISP_PROGRAM>`
  - A Lisp expression that transforms the transaction table
  - The variable `table` contains all transactions
  - See [Lisp Functions](./lisp-functions.md) for available functions

### Description

The `transform` command is kakei's most powerful feature. It allows you to:

- Filter transactions based on criteria
- Group transactions by category, account, or custom logic
- Perform calculations and aggregations
- Extract specific transaction fields
- Create custom reports

The transaction data is provided as a Lisp data structure in the `table` variable.

### Examples

#### View All Transactions

```bash
kakei transform --program "table"
```

#### Group by Category

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
```

#### Group by Account

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'account (cdr pair)))))"
```

#### Get First Transaction Only

```bash
kakei transform --program "(cons (car table) ())"
```

#### Skip First Transaction

```bash
kakei transform --program "(cdr table)"
```

#### Get First Two Transactions

```bash
kakei transform --program "(cons (car table) (cons (car (cdr table)) ()))"
```

### Shell Quoting

Be careful with shell quoting when passing Lisp programs:

**POSIX shells:**

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
```

**PowerShell:**

```powershell
kakei transform --program '(group-by table (lambda (pair) (cdr (assoc ''category'' (cdr pair)))))'
```

### See Also

- [Data Format](./data-format.md) - Understanding transaction data structure
- [Lisp Functions](./lisp-functions.md) - Complete reference of available Lisp functions
- [Examples](./examples.md) - Real-world transformation examples
