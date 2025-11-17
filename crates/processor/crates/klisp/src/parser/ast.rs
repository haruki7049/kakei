//! Abstract Syntax Tree (AST) types for the Lisp dialect.
//!
//! This module defines the core data structures that represent parsed Lisp code.

/// Represents a complete S-expression (Sexpr).
/// This is the primary AST node for the Lisp dialect.
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
