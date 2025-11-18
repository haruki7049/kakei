# Lisp Functions

The `kakei_lisp` dialect provides a set of built-in functions for transforming and analyzing transaction data. This reference covers all available functions.

## Core Functions

### lambda

Create anonymous (unnamed) functions.

**Syntax:**

```lisp
(lambda (parameter ...) body)
```

**Examples:**

```lisp
; Function that adds 1 to its argument
(lambda (x) (+ x 1))

; Function that gets the category from a transaction
(lambda (pair) (cdr (assoc 'category (cdr pair))))
```

**Usage in transform:**

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
```

### define

Define variables or functions for reuse within a program.

**Syntax:**

```lisp
(define name value)
```

**Examples:**

```lisp
; Define a constant
(define pi 3.14159)

; Define a function
(define (double x) (+ x x))

; Use the defined function
(double 5)  ; => 10
```

### if

Conditional evaluation - execute different code based on a condition.

**Syntax:**

```lisp
(if condition then-expr else-expr)
```

**Examples:**

```lisp
; Check if a value is empty
(if (null? x) "empty" "not empty")

; Check equality
(if (equal? category "Food") "food-related" "other")
```

## List Operations

### cons

Construct a pair (cons cell) - the fundamental building block of Lisp lists.

**Syntax:**

```lisp
(cons first second)
```

**Examples:**

```lisp
(cons 1 2)           ; => (1 . 2)
(cons 1 ())          ; => (1)
(cons 1 (cons 2 ())) ; => (1 2)
```

**Usage:**

```lisp
; Create a list with the first transaction only
(cons (car table) ())
```

### car

Get the **first element** of a pair.

**Syntax:**

```lisp
(car pair)
```

**Examples:**

```lisp
(car (cons 1 2))     ; => 1
(car '(1 2 3))       ; => 1
(car table)          ; => first transaction
```

**Usage:**

```bash
# Get the first transaction
kakei transform --program "(car table)"
```

### cdr

Get the **second element** of a pair (everything after the first element).

**Syntax:**

```lisp
(cdr pair)
```

**Examples:**

```lisp
(cdr (cons 1 2))     ; => 2
(cdr '(1 2 3))       ; => (2 3)
(cdr table)          ; => all transactions except first
```

**Usage:**

```bash
# Skip the first transaction
kakei transform --program "(cdr table)"
```

## Comparison Functions

### equal?

Test if two values are equal.

**Syntax:**

```lisp
(equal? value1 value2)
```

**Examples:**

```lisp
(equal? "Food" "Food")      ; => #t (true)
(equal? "Food" "Transport") ; => #f (false)
(equal? 100 100)            ; => #t
(equal? 100 200)            ; => #f
```

**Usage:**

```lisp
; Filter transactions where category is "Food"
(if (equal? (cdr (assoc 'category (cdr pair))) "Food")
    pair
    ())
```

### null?

Test if a value is nil (empty).

**Syntax:**

```lisp
(null? value)
```

**Examples:**

```lisp
(null? ())           ; => #t (true)
(null? '(1 2 3))     ; => #f (false)
(null? "")           ; => #f (false - empty string is not nil)
```

**Usage:**

```lisp
; Check if we've reached the end of a list
(if (null? remaining-transactions)
    "No more transactions"
    "More transactions exist")
```

## Association List Functions

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

## Table Manipulation

### group-by

Group a table of transactions by a key function. This is one of the most powerful functions for analysis.

**Syntax:**

```lisp
(group-by table key-fn)
```

**Parameters:**

- `table`: The list of transactions
- `key-fn`: A lambda function that extracts the grouping key from each transaction

**Returns:**
A list of groups, where each group is:

```lisp
("GroupName" (transaction1) (transaction2) ...)
```

**Examples:**

#### Group by Category

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
```

Output structure:

```lisp
(("Food" 
  (ID-001 . ((date . "2025-01-01") (amount . -1000) ...)))
 ("Transport"
  (ID-002 . ((date . "2025-01-02") (amount . -2000) ...)))
 ("Salary"
  (ID-003 . ((date . "2025-01-15") (amount . 50000) ...))))
```

#### Group by Account

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'account (cdr pair)))))"
```

#### Group by Date

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'date (cdr pair)))))"
```

**How it Works:**

For each transaction in the table:

1. The lambda function extracts a grouping key (e.g., category name)
1. Transactions with the same key are grouped together
1. The result is a list of groups labeled by their key

## Complete Examples

### Example 1: Extract All Categories

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
```

This groups all transactions by their category, showing you a breakdown of spending/income by category.

### Example 2: Get First Two Transactions

```bash
kakei transform --program "(cons (car table) (cons (car (cdr table)) ()))"
```

Step by step:

1. `(car table)` - get first transaction
1. `(cdr table)` - get remaining transactions
1. `(car (cdr table))` - get second transaction
1. `(cons ... ())` - build a list

### Example 3: Custom Filter (Food Only)

```lisp
(define (is-food? pair)
  (equal? (cdr (assoc 'category (cdr pair))) "Food"))

(define (filter-food table)
  (if (null? table)
      ()
      (if (is-food? (car table))
          (cons (car table) (filter-food (cdr table)))
          (filter-food (cdr table)))))

(filter-food table)
```

## Function Composition

You can combine functions to create powerful transformations:

```lisp
; Get categories from grouped results
(car (car (group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))))

; Count transactions in first group
(length (cdr (car (group-by table (lambda (pair) (cdr (assoc 'category (cdr pair))))))))
```

## Tips and Best Practices

1. **Start Simple**: Begin with simple expressions like `table` or `(car table)` and build up complexity

1. **Use assoc + cdr**: This pattern appears frequently for extracting field values:

   ```lisp
   (cdr (assoc 'field-name (cdr transaction)))
   ```

1. **Test Incrementally**: Build your expression piece by piece, testing each part:

   ```bash
   # Step 1: View all transactions
   kakei transform --program "table"

   # Step 2: Get first transaction
   kakei transform --program "(car table)"

   # Step 3: Get fields of first transaction
   kakei transform --program "(cdr (car table))"

   # Step 4: Get category field
   kakei transform --program "(assoc 'category (cdr (car table)))"
   ```

1. **Group for Analysis**: Use `group-by` to understand spending patterns by category, account, or date

1. **Save Complex Programs**: For complex transformations, save your Lisp program to a file and load it:

   ```bash
   cat > analysis.lisp << 'EOF'
   (group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))
   EOF

   kakei transform --program "$(cat analysis.lisp)"
   ```

## See Also

- [Data Format](./data-format.md) - Understanding transaction structure
- [Commands](./commands.md#transform) - Using the transform command
- [Examples](./examples.md) - Real-world usage examples
