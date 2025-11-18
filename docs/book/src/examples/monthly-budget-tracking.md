# Monthly Budget Tracking

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
