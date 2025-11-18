# add

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
