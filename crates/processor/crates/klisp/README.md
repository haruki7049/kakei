# kakei_lisp

`kakei_lisp` is a simple S-expression (Lisp) parser (or "reader") built with the [nom](https://crates.io/crates/nom) parser combinator library.

This crate serves as the parsing foundation for the scripting and configuration language used by the main [`kakei`](https://github.com/haruki7049/kakei) financial ledger application. Its sole responsibility is to take a string input and transform it into a Rust-native Abstract Syntax Tree (AST), represented by the `Sexpr` and `Atom` enums.

## Features

This parser supports standard S-expression syntax:

* **Atoms**: Symbols (`define`), Numbers (`60000`), Strings (`"Alice"`), and Nil (`()`).
* **Lists**: Proper lists, e.g., `(a b c)`.
* **Dotted Pairs**: Improper lists, e.g., `(a . b)` or `(a b . c)`.
* **Quoting**: The `'` (single quote) syntax, e.g., `'(a b)`, which is parsed into `(quote (a b))`.
* **Comments**: Line comments starting with `;` which are ignored by the parser.

## API & Usage

The primary public API consists of the `parse` function and the `Sexpr` and `Atom` enums.

### Add to `Cargo.toml`

This crate is part of the `kakei` workspace.

```toml
[dependencies]
kakei_lisp = { path = "crates/processor/crates/klisp" }
````

### Data Structures

The parser converts text into these Rust enums:

```rust
/// Represents a complete S-expression (Sexpr).
#[derive(Debug, PartialEq, Clone)]
pub enum Sexpr {
    /// An atomic value, such as a symbol, number, or string.
    Atom(Atom),
    /// A proper list of S-expressions, e.g., `(a b c)`.
    List(Vec<Sexpr>),
    /// An improper list or "dotted pair", e.g., `(a . b)` or `(a b . c)`.
    DottedList(Vec<Sexpr>, Box<Sexpr>),
}

/// Represents the smallest indivisible unit of the Lisp syntax (an "atom").
#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    /// The empty list, `()`, also known as Nil.
    Nil,
    /// A symbolic identifier, e.g., `define`, `ID-001`, or `+`.
    Symbol(String),
    /// An integer number, e.g., `60000`.
    Number(i64),
    /// A string literal, e.g., `"Alice"`.
    String(String),
}
```

### Example

The `parse` function takes a string slice (`&str`) and returns a `Result` containing a `Vec<Sexpr>` on success.

```rust
use kakei_lisp::{parse, Atom, Sexpr};

fn main() {
    let input = r#"
        ; This is the employee table
        (define employee-table
          '( (ID-001 . ((name . "Alice") (dept . "Dev")))
             (ID-002 . ((name . "Bob") (dept . "Sales"))) ))
        
        ; A proper list
        (a b c)
        "#;

    match parse(input) {
        Ok((remaining_input, sexprs)) => {
            // Success! `sexprs` is a Vec<Sexpr>
            assert_eq!(sexprs.len(), 2);

            // Check the first expression (define ...)
            if let Sexpr::List(define_expr) = &sexprs[0] {
                assert_eq!(define_expr[0], Sexpr::Atom(Atom::Symbol("define".to_string())));
            }

            // Check the second expression (a b c)
            if let Sexpr::List(list_expr) = &sexprs[1] {
                 assert_eq!(list_expr[0], Sexpr::Atom(Atom::Symbol("a".to_string())));
            }

            assert!(remaining_input.trim().is_empty());
        }
        Err(e) => {
            // Handle parser error
            panic!("Parser failed: {:?}", e);
        }
    }
}
```

## License

This crate is part of the `kakei` project and is licensed under the MIT License.
