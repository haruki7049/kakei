# transform

Transform and analyze transaction data using Lisp programs.

### Usage

```bash
kakei transform --program <LISP_PROGRAM>
```

### Required Arguments

- `--program <LISP_PROGRAM>`
  - A Lisp expression that transforms the transaction table
  - The variable `table` contains all transactions
  - See [Lisp Functions](./lisp-functions.md) for available functions

### Description

The `transform` command is kakei's most powerful feature. It allows you to:

- Filter transactions based on criteria
- Group transactions by category, account, or custom logic
- Perform calculations and aggregations
- Extract specific transaction fields
- Create custom reports

The transaction data is provided as a Lisp data structure in the `table` variable.

### Examples

#### View All Transactions

```bash
kakei transform --program "table"
```

#### Group by Category

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
```

#### Group by Account

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'account (cdr pair)))))"
```

#### Get First Transaction Only

```bash
kakei transform --program "(cons (car table) ())"
```

#### Skip First Transaction

```bash
kakei transform --program "(cdr table)"
```

#### Get First Two Transactions

```bash
kakei transform --program "(cons (car table) (cons (car (cdr table)) ()))"
```

### Shell Quoting

Be careful with shell quoting when passing Lisp programs:

**POSIX shells:**

```bash
kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
```

**PowerShell:**

```powershell
kakei transform --program '(group-by table (lambda (pair) (cdr (assoc ''category'' (cdr pair)))))'
```

### See Also

- [Data Format](./data-format.md) - Understanding transaction data structure
- [Lisp Functions](./lisp-functions.md) - Complete reference of available Lisp functions
- [Examples](./examples.md) - Real-world transformation examples
