//! `kakei_lisp` (klisp) provides a parser (reader) for a simple Lisp dialect.
//!
//! This crate is responsible for turning a string representation of Lisp code
//! (S-expressions) into a Rust-native abstract syntax tree (AST) defined by
//! the [Sexpr] and [Atom] enums.
//!
//! The main entry point is [parse] for parsing a complete file or input.
//!
//! # Example
//!
//! ```
//! use kakei_lisp::{parse, Sexpr, Atom};
//!
//! let input = "(define x 42)";
//! let result = parse(input);
//! assert!(result.is_ok());
//! ```

mod ast;
mod parser;
mod whitespace;

// Re-export the main types and functions for public API
pub use ast::{Atom, Sexpr};
pub use parser::parse;
