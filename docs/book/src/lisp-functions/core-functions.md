# Core Functions

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
