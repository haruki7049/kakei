# Comparison Functions

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
