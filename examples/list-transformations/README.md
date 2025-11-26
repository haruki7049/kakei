# List Transformation Examples

This directory contains example Lisp transformation files for the `kakei list` command.

## Usage

To use any of these examples:

1. Copy the desired `.kakei` file to your kakei configuration directory as `list.kakei`:

   **Linux/macOS:**

   ```bash
   cp examples/list-transformations/group-by-category.kakei ~/.config/kakei/list.kakei
   ```

   **Windows:**

   ```powershell
   copy examples\list-transformations\group-by-category.kakei %APPDATA%\kakei\list.kakei
   ```

1. Run `kakei list` to see the transformed output

1. To restore the default view, simply delete the `list.kakei` file

## Available Examples

### group-by-category.kakei

Groups transactions by category. This is useful for quickly seeing how much you spend in each category.

### group-by-account.kakei

Groups transactions by account. This helps you see which accounts have the most activity.

### default.kakei

The default ungrouped view. This is equivalent to not having a `list.kakei` file at all.

## Creating Your Own Transformations

The `list.kakei` file uses the same Lisp syntax as the `kakei transform` command. The Lisp program receives a `table` variable containing all recent transactions.

See the [Lisp Functions documentation](https://haruki7049.github.io/kakei/lisp-functions.html) for available functions.

### Example: Filter by Amount

To show only transactions above a certain amount:

```lisp
(filter table (lambda (pair) 
  (> (cdr (assoc 'amount (cdr pair))) 1000)))
```

### Example: Sort by Date

Transactions are already sorted by date (newest first), but you can reverse the order:

```lisp
(reverse table)
```
