# kakei_lisp

`kakei_lisp` is a simple Lisp interpreter built with Rust, featuring both a parser (reader) and an evaluator. The parser is built with the [nom](https://crates.io/crates/nom) parser combinator library.

This crate serves as the scripting and configuration language for the main [`kakei`](https://github.com/haruki7049/kakei) financial ledger application. It can parse Lisp code into a Rust-native Abstract Syntax Tree (AST) and evaluate it to perform data transformations, particularly for table manipulation.

## Features

### Parser (Reader)

The parser supports standard S-expression syntax:

- **Atoms**: Symbols (`define`), Numbers (`60000`), Strings (`"Alice"`), and Nil (`()`).
- **Lists**: Proper lists, e.g., `(a b c)`.
- **Dotted Pairs**: Improper lists, e.g., `(a . b)` or `(a b . c)`.
- **Quoting**: The `'` (single quote) syntax, e.g., `'(a b)`, which is parsed into `(quote (a b))`.
- **Comments**: Line comments starting with `;` which are ignored by the parser.

### Evaluator

The evaluator provides a functional Lisp interpreter with the following features:

**Special Forms:**

- `quote` - Prevent evaluation, treat as data
- `define` - Define variables in the environment
- `lambda` - Create anonymous functions (closures)
- `if` - Conditional evaluation

**Built-in Functions:**

- `cons` - Create cons cells (pairs)
- `car` - Get the first element of a pair
- `cdr` - Get the second element of a pair
- `null?` - Check if a value is nil
- `equal?` - Check equality of two values
- `assoc` - Search association lists by key
- `group-by` - Group table rows by a key function (for data transformation)

## REPL (Interactive Shell)

`kakei_lisp` includes an interactive REPL (Read-Eval-Print Loop) for experimenting with Lisp expressions:

```bash
# Run the REPL from the klisp directory
cd crates/processor/crates/klisp
cargo run --bin repl

# Or run from the workspace root
cargo run --manifest-path crates/processor/crates/klisp/Cargo.toml --bin repl
```

Example REPL session:

```
kakei_lisp REPL v0.1.0
Type expressions to evaluate. Press Ctrl+C or Ctrl+D to exit.

klisp> (define x 42)
42
klisp> x
42
klisp> (car (cons 1 2))
1
klisp> '(a b c)
(a b c)
```

The REPL features:
- Line editing and history (arrow keys work!)
- Persistent environment across evaluations
- Error reporting for parse and evaluation errors

## API & Usage

The primary public API consists of:

- `parse` - Parse Lisp code into AST
- `eval` - Evaluate an AST expression
- `create_global_env` - Create an environment with built-in functions
- `Sexpr` and `Atom` - AST types
- `Value` - Runtime value types

### Add to `Cargo.toml`

This crate is part of the `kakei` workspace.

```toml
[dependencies]
kakei_lisp = { path = "crates/processor/crates/klisp" }
```

### Example: Basic Evaluation

```rust
use kakei_lisp::{parse, eval, create_global_env, Value};

fn main() {
    let input = r#"
        (define x 42)
        (define y 10)
        (cons x y)
    "#;

    // Parse the input
    let (_, sexprs) = parse(input).expect("Parse error");

    // Create environment with built-in functions
    let mut env = create_global_env();

    // Evaluate each expression
    let mut result = Value::Nil;
    for sexpr in sexprs {
        result = eval(&sexpr, &mut env).expect("Eval error");
    }

    // Result is (42 . 10)
    println!("Result: {}", result);
}
```

### Example: Table Manipulation with `group-by`

The primary use case for this Lisp dialect is manipulating tabular data. Here's a complete example using the `group-by` function to organize employee data by department:

```rust
use kakei_lisp::{parse, eval, create_global_env};

fn main() {
    let program = r#"
        ; Define employee table with row IDs and data
        ; Each row is (ID . ((col . val) ...))
        (define employee-table '(
            (ID-001 . ((name . "Alice") (dept . "Dev") (salary . 60000)))
            (ID-002 . ((name . "Bob") (dept . "Sales") (salary . 70000)))
            (ID-003 . ((name . "Carol") (dept . "Dev") (salary . 65000)))
            (ID-004 . ((name . "Dave") (dept . "HR") (salary . 55000)))
        ))
        
        ; Define lambda to extract department from a row
        ; The row format is (row-id . row-data)
        ; We use assoc to get the dept column from row-data
        ; (cdr row) extracts the row-data part from the (row-id . row-data) structure
        (define get-dept (lambda (row)
            (cdr (assoc 'dept (cdr row)))))
        
        ; Group by department
        (group-by employee-table get-dept)
    "#;

    let (_, sexprs) = parse(program).expect("Parse error");
    let mut env = create_global_env();
    
    let mut result = kakei_lisp::Value::Nil;
    for sexpr in sexprs {
        result = eval(&sexpr, &mut env).expect("Eval error");
    }
    
    // Result is an association list:
    // (("Dev" . ((ID-001 . ...) (ID-003 . ...)))
    //  ("Sales" . ((ID-002 . ...)))
    //  ("HR" . ((ID-004 . ...))))
    println!("Grouped by department: {}", result);
}
```

### Data Structures

#### AST Types (from Parser)

```rust
/// Represents a complete S-expression (Sexpr).
pub enum Sexpr {
    Atom(Atom),
    List(Vec<Sexpr>),
    DottedList(Vec<Sexpr>, Box<Sexpr>),
}

/// Represents an atom in the AST.
pub enum Atom {
    Nil,
    Symbol(String),
    Number(i64),
    String(String),
}
```

#### Runtime Value Types (from Evaluator)

```rust
/// Runtime values that result from evaluation.
pub enum Value {
    Nil,
    Bool(bool),
    Number(i64),
    String(String),
    Symbol(String),
    Cons(Rc<Value>, Rc<Value>),
    Primitive(PrimitiveFn),
    Lambda { params, body, closure },
}
```

## License

This crate is part of the `kakei` project and is licensed under the MIT License.
