# Examples

Real-world examples of using kakei for various financial tracking scenarios.

## Monthly Budget Tracking

Track your monthly income and expenses to understand your budget.

### Setup

First, add your transactions for the month:

```bash
# Add various expenses
kakei add --date 2025-01-05 --amount -1200 --category Food --account Cash --memo "Lunch"
kakei add --date 2025-01-10 --amount -3000 --category Transport --account Card --memo "Monthly pass"
kakei add --date 2025-01-12 --amount -500 --category Hobby --account Cash --memo "Book"
kakei add --date 2025-01-20 --amount -800 --category Food --account Cash --memo "Dinner"

# Add income
kakei add --date 2025-01-15 --amount 50000 --category Salary --account Bank --memo "January salary"
```

### View All Transactions

```bash
kakei list
```

Output:

```
╭────────────┬────────┬───────────┬─────────┬──────────────╮
│ Date       │ Amount │ Category  │ Account │ Memo         │
├────────────┼────────┼───────────┼─────────┼──────────────┤
│ 2025-01-20 │ ¥-800  │ Food      │ Cash    │ Dinner       │
│ 2025-01-15 │ ¥50000 │ Salary    │ Bank    │ January sal… │
│ 2025-01-12 │ ¥-500  │ Hobby     │ Cash    │ Book         │
│ 2025-01-10 │ ¥-3000 │ Transport │ Card    │ Monthly pass │
│ 2025-01-05 │ ¥-1200 │ Food      │ Cash    │ Lunch        │
╰────────────┴────────┴───────────┴─────────┴──────────────╯
```

### Group by Category

See spending breakdown by category:

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
```

This shows all transactions grouped by their category, making it easy to see where your money is going.

### Calculate Category Totals

While kakei's Lisp dialect doesn't have arithmetic functions yet, you can group by category and manually sum the amounts:

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
```

Look at the amounts in each group to understand your spending patterns.

## Analyzing Spending Patterns

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

## Daily Expense Tracking

Track your daily spending habits.

### Morning Routine

```bash
# Coffee at the station
kakei add --date 2025-01-25 --amount -300 --category Food --account Cash --memo "Morning coffee"

# Train to work
kakei add --date 2025-01-25 --amount -200 --category Transport --account Card --memo "Train fare"
```

### Lunch Break

```bash
kakei add --date 2025-01-25 --amount -800 --category Food --account Cash --memo "Lunch bento"
```

### Evening

```bash
# Grocery shopping
kakei add --date 2025-01-25 --amount -2500 --category Food --account Card --memo "Groceries"

# Quick review of today's spending
kakei list
```

### Group by Date

See all transactions for a specific date:

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'date (cdr pair)))))"
```

## Freelance Income Tracking

Track multiple income sources.

### Record Different Income Types

```bash
# Regular salary
kakei add --date 2025-01-15 --amount 50000 --category Salary --account Bank --memo "January salary"

# Freelance project payment
kakei add --date 2025-01-20 --amount 30000 --category Freelance --account Bank --memo "Website project"

# Consulting fee
kakei add --date 2025-01-25 --amount 20000 --category Consulting --account Bank --memo "Tech consultation"
```

**Note:** You'll need to add "Freelance" and "Consulting" to your config.toml categories first.

### View Income Only

Using transform to filter (conceptual - filtering requires custom Lisp):

```bash
# View all income grouped by category
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
```

Look at the categories with positive amounts to see your income sources.

## Multi-Currency Tracking

Track transactions in different currencies.

### Mixed Currency Transactions

```bash
# Salary in JPY
kakei add --date 2025-01-15 --amount 50000 --category Salary --account Bank --currency JPY

# Business trip expenses in USD (amounts in cents)
kakei add --date 2025-01-20 --amount -5000 --category Travel --account Card --currency USD --memo "Hotel"
kakei add --date 2025-01-20 --amount -3500 --category Food --account Card --currency USD --memo "Meals"

# European vacation in EUR (amounts in cents)
kakei add --date 2025-02-01 --amount -10000 --category Travel --account Card --currency EUR --memo "Flight"
```

### View All Currencies

```bash
kakei list
```

The list will show each transaction with its appropriate currency symbol (¥, $, €, etc.).

## Savings Goal Tracking

Track progress toward savings goals.

### Regular Savings

```bash
# Monthly transfer to savings
kakei add --date 2025-01-15 --amount -10000 --category Savings --account Bank --memo "To savings account"

# Record in savings account (if tracking separately)
kakei add --date 2025-01-15 --amount 10000 --category Savings --account "Savings Account" --memo "From checking"
```

### View Savings Transactions

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
```

Look for the "Savings" group to see all savings-related transactions.

## Category Analysis

Deep dive into specific categories.

### Food Spending Analysis

1. **Record food transactions over time**:

   ```bash
   kakei add --date 2025-01-05 --amount -1200 --category Food --account Cash --memo "Lunch"
   kakei add --date 2025-01-10 --amount -800 --category Food --account Cash --memo "Dinner"
   kakei add --date 2025-01-15 --amount -2500 --category Food --account Card --memo "Groceries"
   kakei add --date 2025-01-20 --amount -1500 --category Food --account Cash --memo "Restaurant"
   ```

1. **Group by category to see all food transactions**:

   ```bash
   kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
   ```

1. **Manually sum the amounts** to get total food spending

### Transport Cost Analysis

Similar to food analysis, but for transport:

```bash
# Daily train
kakei add --date 2025-01-05 --amount -200 --category Transport --account Card --memo "Train"
kakei add --date 2025-01-10 --amount -3000 --category Transport --account Card --memo "Monthly pass"
kakei add --date 2025-01-15 --amount -1500 --category Transport --account Cash --memo "Taxi"

# Group to analyze
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
```

## Account Balance Tracking

Track cash flow between accounts.

### Bank Account Activity

```bash
# Income
kakei add --date 2025-01-15 --amount 50000 --category Salary --account Bank

# Expenses from bank
kakei add --date 2025-01-20 --amount -30000 --category Rent --account Bank
kakei add --date 2025-01-25 --amount -5000 --category Utilities --account Bank

# Withdrawal to cash
kakei add --date 2025-01-10 --amount -10000 --category "Transfer" --account Bank --memo "To cash"
```

### Cash Account Activity

```bash
# Cash deposit from bank
kakei add --date 2025-01-10 --amount 10000 --category "Transfer" --account Cash --memo "From bank"

# Cash expenses
kakei add --date 2025-01-12 --amount -800 --category Food --account Cash
kakei add --date 2025-01-15 --amount -500 --category Transport --account Cash
```

### Group by Account

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'account (cdr pair)))))"
```

This shows all transactions for each account, helping you understand your account balances.

## Tips for Effective Usage

### 1. Consistent Entry

Enter transactions regularly (daily or weekly) to maintain accurate records:

```bash
# End of day routine
kakei add --date $(date +%Y-%m-%d) --amount -XXXX --category YYY --account ZZZ --memo "..."
```

### 2. Descriptive Memos

Use memos to add context that will be helpful later:

```bash
# Good memos
kakei add ... --memo "Monthly train pass"
kakei add ... --memo "Grocery shopping at SuperMart"
kakei add ... --memo "Birthday gift for friend"

# Less helpful memos
kakei add ... --memo "stuff"
kakei add ... --memo "food"
```

### 3. Regular Reviews

Review your spending weekly or monthly:

```bash
# Weekly review
kakei list

# Monthly category analysis
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
```

### 4. Backup Regularly

Your financial data is valuable - back it up:

```bash
# Simple backup
cp ~/.local/share/kakei/kakei.db ~/backups/kakei-$(date +%Y%m%d).db

# Or create a backup script (see Configuration chapter)
```

## See Also

- [Commands](./commands.md) - Complete command reference
- [Lisp Functions](./lisp-functions.md) - All available Lisp functions
- [Configuration](./configuration.md) - Customizing categories and accounts
