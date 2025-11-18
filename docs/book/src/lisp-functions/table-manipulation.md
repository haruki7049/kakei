# Table Manipulation

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
