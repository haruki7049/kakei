# list

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
