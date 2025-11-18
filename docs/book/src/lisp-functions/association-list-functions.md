# Association List Functions

### assoc

Find a key in an association list and return the key-value pair.

**Syntax:**

```lisp
(assoc key alist)
```

**Examples:**

```lisp
(assoc 'category '((date . "2025-01-01") (category . "Food")))
; => (category . "Food")

(assoc 'amount '((date . "2025-01-01") (amount . -1000)))
; => (amount . -1000)
```

**Usage:**

```bash
# Get the category field from first transaction
kakei transform --program "(assoc 'category (cdr (car table)))"
```

To get just the value (not the pair), combine with `cdr`:

```bash
# Get the category value
kakei transform --program "(cdr (assoc 'category (cdr (car table))))"
```

### Common Pattern: Extracting Field Values

```lisp
; Get category of a transaction
(cdr (assoc 'category (cdr transaction)))

; Get amount of a transaction
(cdr (assoc 'amount (cdr transaction)))

; Get date of a transaction
(cdr (assoc 'date (cdr transaction)))

; Get account of a transaction
(cdr (assoc 'account (cdr transaction)))

; Get memo of a transaction
(cdr (assoc 'memo (cdr transaction)))
```
