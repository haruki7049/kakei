# Add Transactions

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
