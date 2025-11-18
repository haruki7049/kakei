# Transaction Structure

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
