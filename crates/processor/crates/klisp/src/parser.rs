//! Parser implementations for Lisp S-expressions.
//!
//! This module contains all the nom-based parser combinators that transform
//! text input into the AST types defined in the [ast] module.

use crate::ast::{Atom, Sexpr};
use crate::whitespace::ws;
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{is_not, take_while},
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

/// Parses a String literal, e.g., `"Alice"` or `""`.
///
/// # Examples
/// - Input: `"hello"` → Output: `Atom::String("hello")`
/// - Input: `""` → Output: `Atom::String("")`
fn parse_string(input: &str) -> ParseResult<'_, Atom> {
    map(
        delimited(char('"'), take_while(|c| c != '"'), char('"')),
        |s: &str| Atom::String(s.to_string()),
    )
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

    mod parse_string_tests {
        use super::*;

        /// Tests parsing a simple string literal.
        ///
        /// Verifies that a basic string "hello" is correctly parsed into an Atom::String.
        #[test]
        fn simple() {
            assert_eq!(
                parse_string("\"hello\""),
                Ok(("", Atom::String("hello".to_string())))
            );
        }

        /// Tests parsing an empty string literal.
        ///
        /// Verifies that empty strings "" are correctly parsed into Atom::String("").
        #[test]
        fn empty() {
            assert_eq!(
                parse_string("\"\""),
                Ok(("", Atom::String("".to_string())))
            );
        }

        /// Tests parsing a string with spaces.
        ///
        /// Verifies that strings containing spaces are correctly preserved.
        #[test]
        fn with_spaces() {
            assert_eq!(
                parse_string("\"hello world\""),
                Ok(("", Atom::String("hello world".to_string())))
            );
        }

        /// Tests parsing a string with special characters.
        ///
        /// Verifies that dashes, underscores, digits, and exclamation marks
        /// are correctly included in the parsed string.
        #[test]
        fn with_special_chars() {
            assert_eq!(
                parse_string("\"hello-world_123!\""),
                Ok(("", Atom::String("hello-world_123!".to_string())))
            );
        }

        /// Tests parsing a string with remaining input.
        ///
        /// Verifies that the parser returns the parsed string and leaves
        /// any remaining input unparsed.
        #[test]
        fn with_remaining() {
            assert_eq!(
                parse_string("\"test\" extra"),
                Ok((" extra", Atom::String("test".to_string())))
            );
        }
    }

    mod parse_number_tests {
        use super::*;

        /// Tests parsing a single digit number.
        ///
        /// Verifies that single digit numbers like "5" are correctly parsed.
        #[test]
        fn single_digit() {
            assert_eq!(parse_number("5"), Ok(("", Atom::Number(5))));
        }

        /// Tests parsing a number with multiple digits.
        ///
        /// Verifies that larger numbers like "60000" are correctly parsed.
        #[test]
        fn multiple_digits() {
            assert_eq!(parse_number("60000"), Ok(("", Atom::Number(60000))));
        }

        /// Tests parsing zero.
        ///
        /// Verifies that the number "0" is correctly parsed.
        #[test]
        fn zero() {
            assert_eq!(parse_number("0"), Ok(("", Atom::Number(0))));
        }

        /// Tests parsing a number with remaining non-numeric input.
        ///
        /// Verifies that the parser stops at the first non-digit character
        /// and leaves the rest unparsed.
        #[test]
        fn with_remaining() {
            assert_eq!(parse_number("123abc"), Ok(("abc", Atom::Number(123))));
        }

        /// Tests parsing a very large number (i64::MAX).
        ///
        /// Verifies that the parser can handle the maximum i64 value.
        #[test]
        fn large() {
            assert_eq!(
                parse_number("9223372036854775807"),
                Ok(("", Atom::Number(9223372036854775807)))
            );
        }
    }

    mod parse_symbol_tests {
        use super::*;

        /// Tests parsing a simple symbol.
        ///
        /// Verifies that alphabetic symbols like "define" are correctly parsed.
        #[test]
        fn simple() {
            assert_eq!(
                parse_symbol("define"),
                Ok(("", Atom::Symbol("define".to_string())))
            );
        }

        /// Tests parsing a symbol with dashes.
        ///
        /// Verifies that symbols containing dashes and numbers like "ID-001"
        /// are correctly parsed.
        #[test]
        fn with_dash() {
            assert_eq!(
                parse_symbol("ID-001"),
                Ok(("", Atom::Symbol("ID-001".to_string())))
            );
        }

        /// Tests parsing operator symbols.
        ///
        /// Verifies that single-character operator symbols (+, -, *, /, >, <, =, ?)
        /// are correctly parsed as symbols.
        #[test]
        fn operators() {
            assert_eq!(parse_symbol("+"), Ok(("", Atom::Symbol("+".to_string()))));
            assert_eq!(parse_symbol("-"), Ok(("", Atom::Symbol("-".to_string()))));
            assert_eq!(parse_symbol("*"), Ok(("", Atom::Symbol("*".to_string()))));
            assert_eq!(parse_symbol("/"), Ok(("", Atom::Symbol("/".to_string()))));
            assert_eq!(parse_symbol(">"), Ok(("", Atom::Symbol(">".to_string()))));
            assert_eq!(parse_symbol("<"), Ok(("", Atom::Symbol("<".to_string()))));
            assert_eq!(parse_symbol("="), Ok(("", Atom::Symbol("=".to_string()))));
            assert_eq!(parse_symbol("?"), Ok(("", Atom::Symbol("?".to_string()))));
        }

        /// Tests parsing a symbol ending with a question mark.
        ///
        /// Verifies that predicate-style symbols like "null?" are correctly parsed.
        #[test]
        fn with_question_mark() {
            assert_eq!(
                parse_symbol("null?"),
                Ok(("", Atom::Symbol("null?".to_string())))
            );
        }

        /// Tests parsing a symbol ending with an exclamation mark.
        ///
        /// Verifies that mutation-style symbols like "set!" are correctly parsed.
        #[test]
        fn with_exclamation() {
            assert_eq!(
                parse_symbol("set!"),
                Ok(("", Atom::Symbol("set!".to_string())))
            );
        }

        /// Tests parsing a complex symbol with multiple special characters.
        ///
        /// Verifies that symbols combining letters, dashes, numbers, and
        /// question marks like "my-var-123?" are correctly parsed.
        #[test]
        fn complex() {
            assert_eq!(
                parse_symbol("my-var-123?"),
                Ok(("", Atom::Symbol("my-var-123?".to_string())))
            );
        }

        /// Tests parsing a symbol with remaining input.
        ///
        /// Verifies that the parser stops at invalid symbol characters
        /// and leaves them unparsed.
        #[test]
        fn with_remaining() {
            assert_eq!(
                parse_symbol("abc("),
                Ok(("(", Atom::Symbol("abc".to_string())))
            );
        }
    }

    mod parse_atom_tests {
        use super::*;

        /// Tests parsing a number atom.
        ///
        /// Verifies that parse_atom correctly identifies and parses numeric values.
        #[test]
        fn number() {
            assert_eq!(parse_atom("123"), Ok(("", Atom::Number(123))));
        }

        /// Tests parsing a string atom.
        ///
        /// Verifies that parse_atom correctly identifies and parses string literals.
        #[test]
        fn string() {
            assert_eq!(
                parse_atom("\"hello\""),
                Ok(("", Atom::String("hello".to_string())))
            );
        }

        /// Tests parsing a symbol atom.
        ///
        /// Verifies that parse_atom correctly identifies and parses symbolic identifiers.
        #[test]
        fn symbol() {
            assert_eq!(
                parse_atom("ID-001"),
                Ok(("", Atom::Symbol("ID-001".to_string())))
            );
        }

        /// Tests parsing an operator symbol atom.
        ///
        /// Verifies that parse_atom correctly identifies operator characters as symbols.
        #[test]
        fn operator() {
            assert_eq!(parse_atom("+"), Ok(("", Atom::Symbol("+".to_string()))));
        }
    }

    mod parse_quoted_tests {
        use super::*;

        /// Tests parsing a quoted symbol.
        ///
        /// Verifies that 'A is transformed into (quote A).
        #[test]
        fn symbol() {
            let expected = Sexpr::List(vec![
                Sexpr::Atom(Atom::Symbol("quote".to_string())),
                Sexpr::Atom(Atom::Symbol("A".to_string())),
            ]);
            assert_eq!(parse_quoted("'A"), Ok(("", expected)));
        }

        /// Tests parsing a quoted number.
        ///
        /// Verifies that '42 is transformed into (quote 42).
        #[test]
        fn number() {
            let expected = Sexpr::List(vec![
                Sexpr::Atom(Atom::Symbol("quote".to_string())),
                Sexpr::Atom(Atom::Number(42)),
            ]);
            assert_eq!(parse_quoted("'42"), Ok(("", expected)));
        }

        /// Tests parsing a quoted list.
        ///
        /// Verifies that '(a b) is transformed into (quote (a b)).
        #[test]
        fn list() {
            let expected = Sexpr::List(vec![
                Sexpr::Atom(Atom::Symbol("quote".to_string())),
                Sexpr::List(vec![
                    Sexpr::Atom(Atom::Symbol("a".to_string())),
                    Sexpr::Atom(Atom::Symbol("b".to_string())),
                ]),
            ]);
            assert_eq!(parse_quoted("'(a b)"), Ok(("", expected)));
        }

        /// Tests parsing nested quoted expressions.
        ///
        /// Verifies that ''x is transformed into (quote (quote x)).
        #[test]
        fn nested() {
            let expected = Sexpr::List(vec![
                Sexpr::Atom(Atom::Symbol("quote".to_string())),
                Sexpr::List(vec![
                    Sexpr::Atom(Atom::Symbol("quote".to_string())),
                    Sexpr::Atom(Atom::Symbol("x".to_string())),
                ]),
            ]);
            assert_eq!(parse_quoted("''x"), Ok(("", expected)));
        }
    }

    mod parse_list_tests {
        use super::*;

        /// Tests parsing an empty list.
        ///
        /// Verifies that () is parsed as Nil.
        #[test]
        fn empty() {
            assert_eq!(parse_list("()"), Ok(("", Sexpr::Atom(Atom::Nil))));
        }

        /// Tests parsing an empty list with whitespace.
        ///
        /// Verifies that ( ) and lists with various whitespace are parsed as Nil.
        #[test]
        fn empty_with_spaces() {
            assert_eq!(parse_list("( )"), Ok(("", Sexpr::Atom(Atom::Nil))));
            assert_eq!(parse_list("(  \n  )"), Ok(("", Sexpr::Atom(Atom::Nil))));
        }

        /// Tests parsing a list with a single element.
        ///
        /// Verifies that (1) is parsed as a proper list with one element.
        #[test]
        fn single_element() {
            let expected = Sexpr::List(vec![Sexpr::Atom(Atom::Number(1))]);
            assert_eq!(parse_list("(1)"), Ok(("", expected)));
        }

        /// Tests parsing a list with multiple elements.
        ///
        /// Verifies that (a 1 "two") is parsed as a proper list with mixed types.
        #[test]
        fn multiple_elements() {
            let expected = Sexpr::List(vec![
                Sexpr::Atom(Atom::Symbol("a".to_string())),
                Sexpr::Atom(Atom::Number(1)),
                Sexpr::Atom(Atom::String("two".to_string())),
            ]);
            assert_eq!(parse_list("(a 1 \"two\")"), Ok(("", expected)));
        }

        /// Tests parsing a nested list.
        ///
        /// Verifies that (a (b c)) is parsed with proper nesting.
        #[test]
        fn nested() {
            let expected = Sexpr::List(vec![
                Sexpr::Atom(Atom::Symbol("a".to_string())),
                Sexpr::List(vec![
                    Sexpr::Atom(Atom::Symbol("b".to_string())),
                    Sexpr::Atom(Atom::Symbol("c".to_string())),
                ]),
            ]);
            assert_eq!(parse_list("(a (b c))"), Ok(("", expected)));
        }

        /// Tests parsing a simple dotted list.
        ///
        /// Verifies that (a . b) is parsed as a dotted pair.
        #[test]
        fn dotted_simple() {
            let expected = Sexpr::DottedList(
                vec![Sexpr::Atom(Atom::Symbol("a".to_string()))],
                Box::new(Sexpr::Atom(Atom::Symbol("b".to_string()))),
            );
            assert_eq!(parse_list("(a . b)"), Ok(("", expected)));
        }

        /// Tests parsing a dotted list with multiple elements before the dot.
        ///
        /// Verifies that (a b c . d) is parsed as a dotted list.
        #[test]
        fn dotted_multiple() {
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

        /// Tests parsing a dotted list with a list after the dot.
        ///
        /// Verifies that (a . (b c)) is parsed correctly.
        #[test]
        fn dotted_with_list() {
            let expected = Sexpr::DottedList(
                vec![Sexpr::Atom(Atom::Symbol("a".to_string()))],
                Box::new(Sexpr::List(vec![
                    Sexpr::Atom(Atom::Symbol("b".to_string())),
                    Sexpr::Atom(Atom::Symbol("c".to_string())),
                ])),
            );
            assert_eq!(parse_list("(a . (b c))"), Ok(("", expected)));
        }
    }

    mod parse_sexpr_tests {
        use super::*;

        /// Tests parsing an S-expression that is an atom.
        ///
        /// Verifies that standalone atoms are correctly parsed as S-expressions.
        #[test]
        fn atom() {
            assert_eq!(parse_sexpr("42"), Ok(("", Sexpr::Atom(Atom::Number(42)))));
        }

        /// Tests parsing an S-expression that is a list.
        ///
        /// Verifies that lists are correctly parsed as S-expressions.
        #[test]
        fn list() {
            let expected = Sexpr::List(vec![
                Sexpr::Atom(Atom::Symbol("a".to_string())),
                Sexpr::Atom(Atom::Number(1)),
                Sexpr::Atom(Atom::String("two".to_string())),
            ]);
            assert_eq!(parse_sexpr("(a 1 \"two\")"), Ok(("", expected)));
        }

        /// Tests parsing a quoted S-expression.
        ///
        /// Verifies that the quote shorthand is correctly expanded.
        #[test]
        fn quoted() {
            let expected = Sexpr::List(vec![
                Sexpr::Atom(Atom::Symbol("quote".to_string())),
                Sexpr::Atom(Atom::Symbol("x".to_string())),
            ]);
            assert_eq!(parse_sexpr("'x"), Ok(("", expected)));
        }

        /// Tests parsing an S-expression with leading whitespace.
        ///
        /// Verifies that leading whitespace is correctly consumed.
        #[test]
        fn with_leading_whitespace() {
            assert_eq!(parse_sexpr("  42"), Ok(("", Sexpr::Atom(Atom::Number(42)))));
        }

        /// Tests parsing an S-expression with remaining input.
        ///
        /// Verifies that parsing stops after the first S-expression
        /// and leaves the rest unparsed.
        #[test]
        fn with_remaining() {
            assert_eq!(
                parse_sexpr("42 extra"),
                Ok((" extra", Sexpr::Atom(Atom::Number(42))))
            );
        }
    }

    mod parse_tests {
        use super::*;

        /// Tests parsing empty input.
        ///
        /// Verifies that empty input returns an empty vector of S-expressions.
        #[test]
        fn empty() {
            assert_eq!(parse(""), Ok(("", vec![])));
        }

        /// Tests parsing a single S-expression.
        ///
        /// Verifies that a single expression is returned in a vector.
        #[test]
        fn single_expression() {
            let expected = vec![Sexpr::Atom(Atom::Number(42))];
            assert_eq!(parse("42"), Ok(("", expected)));
        }

        /// Tests parsing multiple S-expressions.
        ///
        /// Verifies that all top-level expressions are parsed and returned.
        #[test]
        fn multiple_expressions() {
            let expected = vec![
                Sexpr::Atom(Atom::Number(1)),
                Sexpr::Atom(Atom::Number(2)),
                Sexpr::Atom(Atom::Number(3)),
            ];
            assert_eq!(parse("1 2 3"), Ok(("", expected)));
        }

        /// Tests parsing mixed types of expressions.
        ///
        /// Verifies that lists and atoms can be parsed together.
        #[test]
        fn mixed_expressions() {
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

        /// Tests parsing with leading and trailing whitespace.
        ///
        /// Verifies that leading whitespace is consumed but trailing is not.
        /// This is intentional behavior: the parser consumes only what's needed
        /// for valid S-expressions, leaving any remaining input (including trailing
        /// whitespace) for potential further parsing or validation.
        #[test]
        fn with_whitespace() {
            let expected = vec![Sexpr::Atom(Atom::Number(42))];
            // Note: Trailing whitespace is intentionally not consumed
            assert_eq!(parse("  42  "), Ok(("  ", expected)));
        }

        /// Tests parsing with newlines between expressions.
        ///
        /// Verifies that newlines are treated as whitespace separators.
        #[test]
        fn with_newlines() {
            let expected = vec![Sexpr::Atom(Atom::Number(1)), Sexpr::Atom(Atom::Number(2))];
            assert_eq!(parse("1\n2"), Ok(("", expected)));
        }
    }
}
