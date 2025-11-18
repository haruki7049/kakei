# Transform with Lisp

One of kakei's most powerful features is the ability to transform and analyze your transaction data using Lisp expressions.

### View All Transactions

```bash
kakei transform --program "table"
```

The `table` variable contains all your transactions as a Lisp data structure.

### Group by Category

See spending broken down by category:

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
```

### Group by Account

See transactions grouped by account:

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'account (cdr pair)))))"
```

### Get First Transaction

```bash
kakei transform --program "(cons (car table) ())"
```

### Skip First Transaction

```bash
kakei transform --program "(cdr table)"
```

### Get First Two Transactions

```bash
kakei transform --program "(cons (car table) (cons (car (cdr table)) ()))"
```
