//! `kakei_lisp` (klisp) provides a parser (reader) for a simple Lisp dialect.
//!
//! This crate is responsible for turning a string representation of Lisp code
//! (S-expressions) into a Rust-native abstract syntax tree (AST) defined by
//! the [Sexpr] and [Atom] enums.
//!
//! The main entry point is [parse] for parsing a complete file or input.

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alpha1, char, digit1, multispace1},
    combinator::{map, map_res, recognize},
    error::Error,
    multi::many0,
    sequence::{delimited, pair, preceded},
};

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

// A type alias for our parser's result type
type ParseResult<'a, O> = IResult<&'a str, O, Error<&'a str>>;

/// A helper parser that consumes whitespace (1+) or comments.
fn ws<'a>(input: &'a str) -> ParseResult<'a, &'a str> {
    recognize(many0(alt((
        recognize(pair(tag(";"), is_not("\n\r"))),
        multispace1,
    ))))
    .parse(input)
}

/// Parses a String literal, e.g., `"Alice"`.
fn parse_string<'a>(input: &'a str) -> ParseResult<'a, Atom> {
    map(delimited(char('"'), is_not("\""), char('"')), |s: &str| {
        Atom::String(s.to_string())
    })
    .parse(input)
}

/// Parses a Number, e.g., `60000`.
fn parse_number<'a>(input: &'a str) -> ParseResult<'a, Atom> {
    map(map_res(digit1, |s: &str| s.parse::<i64>()), Atom::Number).parse(input)
}

/// Parses a Symbol, e.g., `define`, `ID-001`, or `+`.
fn parse_symbol<'a>(input: &'a str) -> ParseResult<'a, Atom> {
    map(
        recognize(pair(
            alt((
                alpha1,
                tag("-"),
                tag("+"),
                tag("*"),
                tag("/"),
                tag(">"),
                tag("<"),
                tag("="),
                tag("?"),
            )),
            many0(alt((alpha1, digit1, tag("-"), tag("?"), tag("!")))),
        )),
        |s: &str| Atom::Symbol(s.to_string()),
    )
    .parse(input)
}

/// Parses any [Atom] (Number, String, or Symbol).
fn parse_atom<'a>(input: &'a str) -> ParseResult<'a, Atom> {
    alt((parse_number, parse_string, parse_symbol)).parse(input)
}

/// Parses a quoted S-expression, e.g., `'A` or `'(A B)`.
fn parse_quoted<'a>(input: &'a str) -> ParseResult<'a, Sexpr> {
    map(
        preceded(
            char('\''),
            parse_sexpr, // Recursively calls the internal parser
        ),
        |sexpr| {
            // Converts 'A into (quote A)
            Sexpr::List(vec![Sexpr::Atom(Atom::Symbol("quote".to_string())), sexpr])
        },
    )
    .parse(input)
}

/// Parses a list or dotted list: `()`, `(A B C)`, `(A . B)`, or `(A B . C)`.
fn parse_list<'a>(input: &'a str) -> ParseResult<'a, Sexpr> {
    // Must start with '('
    let (input, _) = preceded(ws, char('(')).parse(input)?;

    // Handle the empty list '()' case first
    if let Ok((input, _)) = preceded(ws, char(')')).parse(input) {
        return Ok((input, Sexpr::Atom(Atom::Nil)));
    }

    // Parse zero or more S-expressions (the elements)
    let mut elements: Vec<Sexpr> = Vec::new();
    let mut current_input = input;

    loop {
        // Peek at the next non-whitespace char
        let (peek_input, _) = ws(current_input)?;
        if peek_input.starts_with(')') || peek_input.starts_with('.') {
            current_input = peek_input;
            break;
        }

        // If not ')' or '.', parse one S-expression
        let (next_input, sexpr) = parse_sexpr.parse(current_input)?;
        elements.push(sexpr);
        current_input = next_input;
    }

    // We are now at the dot or the closing parenthesis
    let (input, _) = ws(current_input)?; // Consume whitespace

    // Check for the dot
    if let Ok((input, _)) = char::<&'a str, Error<&'a str>>('.').parse(input) {
        // Dot found: This is a DottedList
        let (input, final_expr) = preceded(ws, parse_sexpr).parse(input)?;
        let (input, _) = preceded(ws, char(')')).parse(input)?;
        Ok((input, Sexpr::DottedList(elements, Box::new(final_expr))))
    } else {
        // No dot found: This must be a Proper List
        let (input, _) = char(')').parse(input)?;
        Ok((input, Sexpr::List(elements)))
    }
}

/// Parses a single, complete S-expression.
fn parse_sexpr<'a>(input: &'a str) -> ParseResult<'a, Sexpr> {
    // No longer pub
    preceded(
        ws,
        alt((
            parse_quoted,                 // Try 'A first
            parse_list,                   // Try (A B) or (A . B)
            map(parse_atom, Sexpr::Atom), // Try an Atom
        )),
    )
    .parse(input)
}

/// Parses a sequence of S-expressions from an input string.
pub fn parse<'a>(input: &'a str) -> ParseResult<'a, Vec<Sexpr>> {
    many0(parse_sexpr).parse(input)
}

// --- Test Module ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_atoms() {
        assert_eq!(parse_atom("123"), Ok(("", Atom::Number(123))));
        assert_eq!(
            parse_atom("\"hello\""),
            Ok(("", Atom::String("hello".to_string())))
        );
        assert_eq!(
            parse_atom("ID-001"),
            Ok(("", Atom::Symbol("ID-001".to_string())))
        );
    }

    // Unit test for the internal parse_sexpr
    #[test]
    fn test_internal_parse_sexpr() {
        let input = "(a 1 \"two\")";
        let expected = Sexpr::List(vec![
            Sexpr::Atom(Atom::Symbol("a".to_string())),
            Sexpr::Atom(Atom::Number(1)),
            Sexpr::Atom(Atom::String("two".to_string())),
        ]);
        assert_eq!(parse_sexpr(input), Ok(("", expected)));
    }
}
