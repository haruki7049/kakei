# List Operations

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
