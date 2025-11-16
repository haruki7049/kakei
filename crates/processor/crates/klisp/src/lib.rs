//! `kakei_lisp` (klisp) provides a parser (reader) and evaluator for a simple Lisp dialect.
//!
//! This crate is responsible for turning a string representation of Lisp code
//! (S-expressions) into a Rust-native abstract syntax tree (AST) defined by
//! the [Sexpr] and [Atom] enums, and then evaluating those expressions into
//! runtime values.
//!
//! The main entry points are:
//! - [parse] for parsing a complete file or input into AST
//! - [eval] for evaluating an AST expression
//! - [create_global_env] for creating an environment with built-in functions
//!
//! # Example
//!
//! ```
//! use kakei_lisp::{parse, eval, create_global_env};
//!
//! let input = "(define x 42)";
//! let (_, sexprs) = parse(input).unwrap();
//! let mut env = create_global_env();
//! for sexpr in sexprs {
//!     eval(&sexpr, &mut env).unwrap();
//! }
//! ```

mod parser;
mod evaluator;

// Re-export the main types and functions for public API
pub use parser::{Atom, Sexpr, parse};
pub use evaluator::{Environment, EvalError, Value, eval, create_global_env};
