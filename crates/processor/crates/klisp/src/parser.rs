//! Parser implementations for Lisp S-expressions.
//!
//! This module contains all the nom-based parser combinators that transform
//! text input into the AST types defined in the [ast] module.

use crate::ast::{Atom, Sexpr};
use crate::whitespace::ws;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::is_not,
    character::complete::{alpha1, char, digit1},
    combinator::{map, map_res, recognize},
    error::Error,
    multi::many0,
    sequence::{delimited, pair, preceded},
};

/// Type alias for parser results to reduce repetition.
type ParseResult<'a, O> = IResult<&'a str, O, Error<&'a str>>;

/// The symbol name used for quoted expressions.
const QUOTE_SYMBOL: &str = "quote";

/// Parses a String literal, e.g., `"Alice"`.
///
/// # Examples
/// - Input: `"hello"` → Output: `Atom::String("hello")`
fn parse_string(input: &str) -> ParseResult<'_, Atom> {
    map(delimited(char('"'), is_not("\""), char('"')), |s: &str| {
        Atom::String(s.to_string())
    })
    .parse(input)
}

/// Parses a Number, e.g., `60000`.
///
/// # Examples
/// - Input: `123` → Output: `Atom::Number(123)`
fn parse_number(input: &str) -> ParseResult<'_, Atom> {
    map(map_res(digit1, str::parse::<i64>), Atom::Number).parse(input)
}

/// Parses a Symbol, e.g., `define`, `ID-001`, or `+`.
///
/// Symbols can start with:
/// - Alphabetic characters
/// - Special operator characters: `-`, `+`, `*`, `/`, `>`, `<`, `=`, `?`
///
/// And can continue with:
/// - Alphabetic characters
/// - Digits
/// - Characters: `-`, `?`, `!`
///
/// # Examples
/// - Input: `define` → Output: `Atom::Symbol("define")`
/// - Input: `ID-001` → Output: `Atom::Symbol("ID-001")`
/// - Input: `+` → Output: `Atom::Symbol("+")`
fn parse_symbol(input: &str) -> ParseResult<'_, Atom> {
    let first_char = alt((
        alpha1,
        recognize(char('-')),
        recognize(char('+')),
        recognize(char('*')),
        recognize(char('/')),
        recognize(char('>')),
        recognize(char('<')),
        recognize(char('=')),
        recognize(char('?')),
    ));

    let rest_chars = many0(alt((
        alpha1,
        digit1,
        recognize(char('-')),
        recognize(char('?')),
        recognize(char('!')),
    )));

    map(recognize(pair(first_char, rest_chars)), |s: &str| {
        Atom::Symbol(s.to_string())
    })
    .parse(input)
}

/// Parses any [Atom] (Number, String, or Symbol).
///
/// Order matters: numbers must be tried before symbols since digits
/// could be part of a symbol.
fn parse_atom(input: &str) -> ParseResult<'_, Atom> {
    alt((parse_number, parse_string, parse_symbol)).parse(input)
}

/// Parses a quoted S-expression, e.g., `'A` or `'(A B)`.
///
/// Quoted expressions are syntactic sugar that transforms:
/// - `'A` → `(quote A)`
/// - `'(A B)` → `(quote (A B))`
fn parse_quoted(input: &str) -> ParseResult<'_, Sexpr> {
    map(preceded(char('\''), parse_sexpr), |sexpr| {
        Sexpr::List(vec![
            Sexpr::Atom(Atom::Symbol(QUOTE_SYMBOL.to_string())),
            sexpr,
        ])
    })
    .parse(input)
}

/// Parses a list or dotted list: `()`, `(A B C)`, `(A . B)`, or `(A B . C)`.
///
/// This handles three cases:
/// 1. Empty list: `()` → `Sexpr::Atom(Atom::Nil)`
/// 2. Proper list: `(A B C)` → `Sexpr::List([A, B, C])`
/// 3. Dotted list: `(A . B)` → `Sexpr::DottedList([A], B)`
fn parse_list(input: &str) -> ParseResult<'_, Sexpr> {
    // Must start with '(' (with optional leading whitespace)
    let (input, _) = preceded(ws, char('(')).parse(input)?;

    // Handle the empty list '()' case first
    if let Ok((input, _)) = preceded(ws, char(')')).parse(input) {
        return Ok((input, Sexpr::Atom(Atom::Nil)));
    }

    // Parse zero or more S-expressions (the elements before dot or closing paren)
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
        let (next_input, sexpr) = parse_sexpr(current_input)?;
        elements.push(sexpr);
        current_input = next_input;
    }

    // We are now at the dot or the closing parenthesis
    let (input, _) = ws(current_input)?;

    // Check for the dot (dotted list notation)
    if let Ok((input, _)) = char::<&str, Error<&str>>('.').parse(input) {
        // Dot found: This is a DottedList
        let (input, final_expr) = preceded(ws, parse_sexpr).parse(input)?;
        let (input, _) = preceded(ws, char(')')).parse(input)?;
        Ok((input, Sexpr::DottedList(elements, Box::new(final_expr))))
    } else {
        // No dot found: This is a proper List
        let (input, _) = char(')').parse(input)?;
        Ok((input, Sexpr::List(elements)))
    }
}

/// Parses a single, complete S-expression.
///
/// This is the core recursive parser that handles:
/// - Quoted expressions: `'A`
/// - Lists and dotted lists: `()`, `(A B)`, `(A . B)`
/// - Atoms: numbers, strings, symbols
fn parse_sexpr(input: &str) -> ParseResult<'_, Sexpr> {
    preceded(
        ws,
        alt((parse_quoted, parse_list, map(parse_atom, Sexpr::Atom))),
    )
    .parse(input)
}

/// Parses a sequence of S-expressions from an input string.
///
/// This is the main entry point for parsing complete Lisp code.
/// It returns a vector of all top-level S-expressions found in the input.
///
/// # Examples
/// ```
/// use kakei_lisp::parse;
///
/// let input = "(a 1) (b 2)";
/// let result = parse(input);
/// assert!(result.is_ok());
/// let (remaining, sexprs) = result.unwrap();
/// assert_eq!(sexprs.len(), 2);
/// ```
pub fn parse(input: &str) -> ParseResult<'_, Vec<Sexpr>> {
    many0(parse_sexpr).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for parse_string function
    #[test]
    fn test_parse_string_simple() {
        assert_eq!(
            parse_string("\"hello\""),
            Ok(("", Atom::String("hello".to_string())))
        );
    }

    #[test]
    fn test_parse_string_empty() {
        // Empty strings fail because is_not("\"") requires at least one character
        // This is a known limitation of the current parser implementation
        assert!(parse_string("\"\"").is_err());
    }

    #[test]
    fn test_parse_string_with_spaces() {
        assert_eq!(
            parse_string("\"hello world\""),
            Ok(("", Atom::String("hello world".to_string())))
        );
    }

    #[test]
    fn test_parse_string_with_special_chars() {
        assert_eq!(
            parse_string("\"hello-world_123!\""),
            Ok(("", Atom::String("hello-world_123!".to_string())))
        );
    }

    #[test]
    fn test_parse_string_with_remaining() {
        assert_eq!(
            parse_string("\"test\" extra"),
            Ok((" extra", Atom::String("test".to_string())))
        );
    }

    // Tests for parse_number function
    #[test]
    fn test_parse_number_single_digit() {
        assert_eq!(parse_number("5"), Ok(("", Atom::Number(5))));
    }

    #[test]
    fn test_parse_number_multiple_digits() {
        assert_eq!(parse_number("60000"), Ok(("", Atom::Number(60000))));
    }

    #[test]
    fn test_parse_number_zero() {
        assert_eq!(parse_number("0"), Ok(("", Atom::Number(0))));
    }

    #[test]
    fn test_parse_number_with_remaining() {
        assert_eq!(parse_number("123abc"), Ok(("abc", Atom::Number(123))));
    }

    #[test]
    fn test_parse_number_large() {
        assert_eq!(
            parse_number("9223372036854775807"),
            Ok(("", Atom::Number(9223372036854775807)))
        );
    }

    // Tests for parse_symbol function
    #[test]
    fn test_parse_symbol_simple() {
        assert_eq!(
            parse_symbol("define"),
            Ok(("", Atom::Symbol("define".to_string())))
        );
    }

    #[test]
    fn test_parse_symbol_with_dash() {
        assert_eq!(
            parse_symbol("ID-001"),
            Ok(("", Atom::Symbol("ID-001".to_string())))
        );
    }

    #[test]
    fn test_parse_symbol_operators() {
        assert_eq!(
            parse_symbol("+"),
            Ok(("", Atom::Symbol("+".to_string())))
        );
        assert_eq!(
            parse_symbol("-"),
            Ok(("", Atom::Symbol("-".to_string())))
        );
        assert_eq!(
            parse_symbol("*"),
            Ok(("", Atom::Symbol("*".to_string())))
        );
        assert_eq!(
            parse_symbol("/"),
            Ok(("", Atom::Symbol("/".to_string())))
        );
        assert_eq!(
            parse_symbol(">"),
            Ok(("", Atom::Symbol(">".to_string())))
        );
        assert_eq!(
            parse_symbol("<"),
            Ok(("", Atom::Symbol("<".to_string())))
        );
        assert_eq!(
            parse_symbol("="),
            Ok(("", Atom::Symbol("=".to_string())))
        );
        assert_eq!(
            parse_symbol("?"),
            Ok(("", Atom::Symbol("?".to_string())))
        );
    }

    #[test]
    fn test_parse_symbol_with_question_mark() {
        assert_eq!(
            parse_symbol("null?"),
            Ok(("", Atom::Symbol("null?".to_string())))
        );
    }

    #[test]
    fn test_parse_symbol_with_exclamation() {
        assert_eq!(
            parse_symbol("set!"),
            Ok(("", Atom::Symbol("set!".to_string())))
        );
    }

    #[test]
    fn test_parse_symbol_complex() {
        assert_eq!(
            parse_symbol("my-var-123?"),
            Ok(("", Atom::Symbol("my-var-123?".to_string())))
        );
    }

    #[test]
    fn test_parse_symbol_with_remaining() {
        assert_eq!(
            parse_symbol("abc("),
            Ok(("(", Atom::Symbol("abc".to_string())))
        );
    }

    // Tests for parse_atom function
    #[test]
    fn test_parse_atom_number() {
        assert_eq!(parse_atom("123"), Ok(("", Atom::Number(123))));
    }

    #[test]
    fn test_parse_atom_string() {
        assert_eq!(
            parse_atom("\"hello\""),
            Ok(("", Atom::String("hello".to_string())))
        );
    }

    #[test]
    fn test_parse_atom_symbol() {
        assert_eq!(
            parse_atom("ID-001"),
            Ok(("", Atom::Symbol("ID-001".to_string())))
        );
    }

    #[test]
    fn test_parse_atom_operator() {
        assert_eq!(
            parse_atom("+"),
            Ok(("", Atom::Symbol("+".to_string())))
        );
    }

    // Tests for parse_quoted function
    #[test]
    fn test_parse_quoted_symbol() {
        let expected = Sexpr::List(vec![
            Sexpr::Atom(Atom::Symbol("quote".to_string())),
            Sexpr::Atom(Atom::Symbol("A".to_string())),
        ]);
        assert_eq!(parse_quoted("'A"), Ok(("", expected)));
    }

    #[test]
    fn test_parse_quoted_number() {
        let expected = Sexpr::List(vec![
            Sexpr::Atom(Atom::Symbol("quote".to_string())),
            Sexpr::Atom(Atom::Number(42)),
        ]);
        assert_eq!(parse_quoted("'42"), Ok(("", expected)));
    }

    #[test]
    fn test_parse_quoted_list() {
        let expected = Sexpr::List(vec![
            Sexpr::Atom(Atom::Symbol("quote".to_string())),
            Sexpr::List(vec![
                Sexpr::Atom(Atom::Symbol("a".to_string())),
                Sexpr::Atom(Atom::Symbol("b".to_string())),
            ]),
        ]);
        assert_eq!(parse_quoted("'(a b)"), Ok(("", expected)));
    }

    #[test]
    fn test_parse_quoted_nested() {
        let expected = Sexpr::List(vec![
            Sexpr::Atom(Atom::Symbol("quote".to_string())),
            Sexpr::List(vec![
                Sexpr::Atom(Atom::Symbol("quote".to_string())),
                Sexpr::Atom(Atom::Symbol("x".to_string())),
            ]),
        ]);
        assert_eq!(parse_quoted("''x"), Ok(("", expected)));
    }

    // Tests for parse_list function
    #[test]
    fn test_parse_list_empty() {
        assert_eq!(parse_list("()"), Ok(("", Sexpr::Atom(Atom::Nil))));
    }

    #[test]
    fn test_parse_list_empty_with_spaces() {
        assert_eq!(parse_list("( )"), Ok(("", Sexpr::Atom(Atom::Nil))));
        assert_eq!(parse_list("(  \n  )"), Ok(("", Sexpr::Atom(Atom::Nil))));
    }

    #[test]
    fn test_parse_list_single_element() {
        let expected = Sexpr::List(vec![Sexpr::Atom(Atom::Number(1))]);
        assert_eq!(parse_list("(1)"), Ok(("", expected)));
    }

    #[test]
    fn test_parse_list_multiple_elements() {
        let expected = Sexpr::List(vec![
            Sexpr::Atom(Atom::Symbol("a".to_string())),
            Sexpr::Atom(Atom::Number(1)),
            Sexpr::Atom(Atom::String("two".to_string())),
        ]);
        assert_eq!(parse_list("(a 1 \"two\")"), Ok(("", expected)));
    }

    #[test]
    fn test_parse_list_nested() {
        let expected = Sexpr::List(vec![
            Sexpr::Atom(Atom::Symbol("a".to_string())),
            Sexpr::List(vec![
                Sexpr::Atom(Atom::Symbol("b".to_string())),
                Sexpr::Atom(Atom::Symbol("c".to_string())),
            ]),
        ]);
        assert_eq!(parse_list("(a (b c))"), Ok(("", expected)));
    }

    #[test]
    fn test_parse_list_dotted_simple() {
        let expected = Sexpr::DottedList(
            vec![Sexpr::Atom(Atom::Symbol("a".to_string()))],
            Box::new(Sexpr::Atom(Atom::Symbol("b".to_string()))),
        );
        assert_eq!(parse_list("(a . b)"), Ok(("", expected)));
    }

    #[test]
    fn test_parse_list_dotted_multiple() {
        let expected = Sexpr::DottedList(
            vec![
                Sexpr::Atom(Atom::Symbol("a".to_string())),
                Sexpr::Atom(Atom::Symbol("b".to_string())),
                Sexpr::Atom(Atom::Symbol("c".to_string())),
            ],
            Box::new(Sexpr::Atom(Atom::Symbol("d".to_string()))),
        );
        assert_eq!(parse_list("(a b c . d)"), Ok(("", expected)));
    }

    #[test]
    fn test_parse_list_dotted_with_list() {
        let expected = Sexpr::DottedList(
            vec![Sexpr::Atom(Atom::Symbol("a".to_string()))],
            Box::new(Sexpr::List(vec![
                Sexpr::Atom(Atom::Symbol("b".to_string())),
                Sexpr::Atom(Atom::Symbol("c".to_string())),
            ])),
        );
        assert_eq!(parse_list("(a . (b c))"), Ok(("", expected)));
    }

    // Tests for parse_sexpr function
    #[test]
    fn test_parse_sexpr_atom() {
        assert_eq!(
            parse_sexpr("42"),
            Ok(("", Sexpr::Atom(Atom::Number(42))))
        );
    }

    #[test]
    fn test_parse_sexpr_list() {
        let expected = Sexpr::List(vec![
            Sexpr::Atom(Atom::Symbol("a".to_string())),
            Sexpr::Atom(Atom::Number(1)),
            Sexpr::Atom(Atom::String("two".to_string())),
        ]);
        assert_eq!(parse_sexpr("(a 1 \"two\")"), Ok(("", expected)));
    }

    #[test]
    fn test_parse_sexpr_quoted() {
        let expected = Sexpr::List(vec![
            Sexpr::Atom(Atom::Symbol("quote".to_string())),
            Sexpr::Atom(Atom::Symbol("x".to_string())),
        ]);
        assert_eq!(parse_sexpr("'x"), Ok(("", expected)));
    }

    #[test]
    fn test_parse_sexpr_with_leading_whitespace() {
        assert_eq!(
            parse_sexpr("  42"),
            Ok(("", Sexpr::Atom(Atom::Number(42))))
        );
    }

    #[test]
    fn test_parse_sexpr_with_remaining() {
        assert_eq!(
            parse_sexpr("42 extra"),
            Ok((" extra", Sexpr::Atom(Atom::Number(42))))
        );
    }

    // Tests for parse function
    #[test]
    fn test_parse_empty() {
        assert_eq!(parse(""), Ok(("", vec![])));
    }

    #[test]
    fn test_parse_single_expression() {
        let expected = vec![Sexpr::Atom(Atom::Number(42))];
        assert_eq!(parse("42"), Ok(("", expected)));
    }

    #[test]
    fn test_parse_multiple_expressions() {
        let expected = vec![
            Sexpr::Atom(Atom::Number(1)),
            Sexpr::Atom(Atom::Number(2)),
            Sexpr::Atom(Atom::Number(3)),
        ];
        assert_eq!(parse("1 2 3"), Ok(("", expected)));
    }

    #[test]
    fn test_parse_mixed_expressions() {
        let expected = vec![
            Sexpr::List(vec![
                Sexpr::Atom(Atom::Symbol("a".to_string())),
                Sexpr::Atom(Atom::Number(1)),
            ]),
            Sexpr::List(vec![
                Sexpr::Atom(Atom::Symbol("b".to_string())),
                Sexpr::Atom(Atom::Number(2)),
            ]),
        ];
        assert_eq!(parse("(a 1) (b 2)"), Ok(("", expected)));
    }

    #[test]
    fn test_parse_with_whitespace() {
        let expected = vec![Sexpr::Atom(Atom::Number(42))];
        // Trailing whitespace is not consumed by parse
        assert_eq!(parse("  42  "), Ok(("  ", expected)));
    }

    #[test]
    fn test_parse_with_newlines() {
        let expected = vec![
            Sexpr::Atom(Atom::Number(1)),
            Sexpr::Atom(Atom::Number(2)),
        ];
        assert_eq!(parse("1\n2"), Ok(("", expected)));
    }
}
