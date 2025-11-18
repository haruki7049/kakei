# Table Structure

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
