# Data Format

Understanding how kakei represents transaction data internally is essential for effective use of the `transform` command.

## Transaction Structure

Each transaction in kakei is represented as an **association list** (alist) in Lisp format. An association list is a list of key-value pairs.

### Single Transaction Example

```lisp
(ID-001 . ((date . "2025-01-01")
           (amount . -1000)
           (category . "Food")
           (account . "Cash")
           (memo . "")))
```

### Transaction Fields

Each transaction contains the following fields:

| Field | Type | Description | Example |
|-------|------|-------------|---------|
| **ID** | String | Unique transaction identifier | `"ID-001"` |
| **date** | String | Transaction date (YYYY-MM-DD) | `"2025-01-01"` |
| **amount** | Integer | Amount in minor units | `-1000` (¥-1,000) |
| **category** | String | Category name | `"Food"` |
| **account** | String | Account name | `"Cash"` |
| **memo** | String | Optional note (empty string if none) | `"Train pass"` |

### Understanding the Structure

The transaction is a **pair** (cons cell):
- **Car** (first element): Transaction ID
- **Cdr** (second element): Association list of fields

```lisp
(ID-001 . field-list)
```

The field list is itself a list of pairs:

```lisp
((date . "2025-01-01")
 (amount . -1000)
 (category . "Food")
 (account . "Cash")
 (memo . ""))
```

## Table Structure

The `table` variable provided to `transform` commands contains a **list of transactions**:

```lisp
((ID-001 . ((date . "2025-01-01") (amount . -1000) ...))
 (ID-002 . ((date . "2025-01-02") (amount . -2000) ...))
 (ID-003 . ((date . "2025-01-15") (amount . 50000) ...))
 ...)
```

### Table Operations

#### Access First Transaction

```lisp
(car table)
```

Returns:
```lisp
(ID-001 . ((date . "2025-01-01") (amount . -1000) ...))
```

#### Access Remaining Transactions

```lisp
(cdr table)
```

Returns a table without the first transaction.

#### Get Transaction ID

```lisp
(car (car table))
```

or

```lisp
(car (car table))
```

#### Get Transaction Fields

```lisp
(cdr (car table))
```

Returns the association list of fields for the first transaction.

## Working with Association Lists

### Extract a Field Value

To get the value of a specific field, use `assoc` followed by `cdr`:

```lisp
(cdr (assoc 'category (cdr (car table))))
```

This expression:
1. Gets the first transaction: `(car table)`
2. Gets its fields: `(cdr (car table))`
3. Finds the category pair: `(assoc 'category ...)`
4. Extracts the value: `(cdr ...)`

### Example: Get Category of First Transaction

```bash
kakei transform --program "(cdr (assoc 'category (cdr (car table))))"
```

### Example: Get Amount of First Transaction

```bash
kakei transform --program "(cdr (assoc 'amount (cdr (car table))))"
```

## Complete Example

Let's say you have these transactions:

```bash
kakei add --date 2025-01-01 --amount -1000 --category Food --account Cash
kakei add --date 2025-01-02 --amount -2000 --category Transport --account Cash --memo "Train pass"
kakei add --date 2025-01-15 --amount 50000 --category Salary --account Bank --memo "Monthly salary"
```

The `table` variable would contain:

```lisp
((ID-001 . ((date . "2025-01-01")
            (amount . -1000)
            (category . "Food")
            (account . "Cash")
            (memo . "")))
 (ID-002 . ((date . "2025-01-02")
            (amount . -2000)
            (category . "Transport")
            (account . "Cash")
            (memo . "Train pass")))
 (ID-003 . ((date . "2025-01-15")
            (amount . 50000)
            (category . "Salary")
            (account . "Bank")
            (memo . "Monthly salary"))))
```

## Data Type Reference

### Integers

Used for amounts in minor units:
- `-1000` = ¥-1,000 expense
- `50000` = ¥50,000 income

### Strings

Used for:
- Transaction IDs
- Dates (YYYY-MM-DD format)
- Categories
- Accounts
- Memos

Strings in Lisp are enclosed in double quotes: `"Food"`

### Symbols

Used as keys in association lists:
- `'date`
- `'amount`
- `'category`
- `'account`
- `'memo`

Symbols are prefixed with a single quote in Lisp.

## See Also

- [Lisp Functions](./lisp-functions.md) - Functions for manipulating this data
- [Commands](./commands.md#transform) - Using the transform command
- [Examples](./examples.md) - Practical examples of data transformation
