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

### Custom List Transformation

You can customize how transactions are displayed by creating a Lisp transformation file at `~/.config/kakei/list.kakei`.

#### Location

The transformation file should be placed at:

- **Linux (XDG)**: `~/.config/kakei/list.kakei` (or `$XDG_CONFIG_HOME/kakei/list.kakei`)
- **macOS**: `~/Library/Application Support/kakei/list.kakei`
- **Windows**: `%APPDATA%\kakei\list.kakei`

#### How It Works

When you run `kakei list`:

1. If `list.kakei` exists, the Lisp program in the file is used to transform the transaction table
1. If the file doesn't exist, the default table format is used (as shown above)

The Lisp program receives a `table` variable containing all recent transactions, just like the `transform` command.

#### Example: Group by Category

Create `~/.config/kakei/list.kakei` with:

```lisp
(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))
```

Now when you run `kakei list`, transactions will be grouped by category:

```bash
$ kakei list

=== Food ===
╭────────────┬────────┬──────────┬─────────┬──────╮
│ Date       │ Amount │ Category │ Account │ Memo │
├────────────┼────────┼──────────┼─────────┼──────┤
│ 2025-01-01 │ ¥-1000 │ Food     │ Cash    │      │
╰────────────┴────────┴──────────┴─────────┴──────╯

=== Transport ===
╭────────────┬────────┬───────────┬─────────┬──────╮
│ Date       │ Amount │ Category  │ Account │ Memo │
├────────────┼────────┼───────────┼─────────┼──────┤
│ 2025-01-02 │ ¥-2000 │ Transport │ Cash    │      │
╰────────────┴────────┴───────────┴─────────┴──────╯

=== Salary ===
╭────────────┬────────┬──────────┬─────────┬──────╮
│ Date       │ Amount │ Category │ Account │ Memo │
├────────────┼────────┼──────────┼─────────┼──────┤
│ 2025-01-15 │ ¥50000 │ Salary   │ Bank    │      │
╰────────────┴────────┴──────────┴─────────┴──────╯
```

#### Example: Group by Account

Create `~/.config/kakei/list.kakei` with:

```lisp
(group-by table (lambda (pair) (cdr (assoc 'account (cdr pair)))))
```

This will group transactions by account instead of category.

#### Example: Default View

To restore the default ungrouped view, simply delete the `list.kakei` file or create it with:

```lisp
table
```

#### Tips

- The `list.kakei` file uses the same Lisp syntax as the `transform` command
- You can use any Lisp transformation supported by `kakei_lisp`
- See the [Lisp Functions](../lisp-functions.html) documentation for available functions
- If the transformation has an error, you'll see an error message explaining what went wrong
