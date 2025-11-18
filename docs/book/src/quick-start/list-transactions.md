# List Transactions

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
