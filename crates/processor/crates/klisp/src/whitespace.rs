//! Whitespace and comment handling for the parser.

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::multispace1,
    combinator::recognize,
    error::Error,
    multi::many0,
    sequence::pair,
};

/// A helper parser that consumes whitespace and comments.
///
/// This parser recognizes:
/// - One or more whitespace characters (spaces, tabs, newlines)
/// - Line comments starting with `;` and continuing to end of line
///
/// Returns the consumed whitespace/comment text.
pub fn ws(input: &str) -> IResult<&str, &str, Error<&str>> {
    recognize(many0(alt((
        recognize(pair(tag(";"), is_not("\n\r"))),
        multispace1,
    ))))
    .parse(input)
}
