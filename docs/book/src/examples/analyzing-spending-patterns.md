# Analyzing Spending Patterns

### Group by Account

See where your money is being spent:

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'account (cdr pair)))))"
```

This helps you understand:

- How much you're spending from each account
- Whether you're using cash vs. card appropriately
- If your bank balance matches your expectations

### View Recent Transactions Only

Get just the most recent transaction:

```bash
kakei transform --program "(cons (car table) ())"
```

Get the two most recent transactions:

```bash
kakei transform --program "(cons (car table) (cons (car (cdr table)) ()))"
```

### Skip Old Transactions

Skip the first (oldest) transaction:

```bash
kakei transform --program "(cdr table)"
```

This is useful when you want to exclude historical data from analysis.
