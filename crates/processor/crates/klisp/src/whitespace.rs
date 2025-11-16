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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_empty() {
        assert_eq!(ws(""), Ok(("", "")));
    }

    #[test]
    fn test_ws_no_whitespace() {
        assert_eq!(ws("abc"), Ok(("abc", "")));
    }

    #[test]
    fn test_ws_single_space() {
        assert_eq!(ws(" abc"), Ok(("abc", " ")));
    }

    #[test]
    fn test_ws_multiple_spaces() {
        assert_eq!(ws("   abc"), Ok(("abc", "   ")));
    }

    #[test]
    fn test_ws_tab() {
        assert_eq!(ws("\tabc"), Ok(("abc", "\t")));
    }

    #[test]
    fn test_ws_newline() {
        assert_eq!(ws("\nabc"), Ok(("abc", "\n")));
    }

    #[test]
    fn test_ws_mixed_whitespace() {
        assert_eq!(ws(" \t\n abc"), Ok(("abc", " \t\n ")));
    }

    #[test]
    fn test_ws_comment_simple() {
        assert_eq!(ws("; comment\nabc"), Ok(("abc", "; comment\n")));
    }

    #[test]
    fn test_ws_comment_without_newline() {
        assert_eq!(ws("; comment"), Ok(("", "; comment")));
    }

    #[test]
    fn test_ws_comment_with_content() {
        assert_eq!(
            ws("; This is a comment\nabc"),
            Ok(("abc", "; This is a comment\n"))
        );
    }

    #[test]
    fn test_ws_multiple_comments() {
        assert_eq!(
            ws("; comment 1\n; comment 2\nabc"),
            Ok(("abc", "; comment 1\n; comment 2\n"))
        );
    }

    #[test]
    fn test_ws_comment_and_spaces() {
        assert_eq!(
            ws("  ; comment\n  abc"),
            Ok(("abc", "  ; comment\n  "))
        );
    }

    #[test]
    fn test_ws_spaces_before_comment() {
        assert_eq!(
            ws(" \t; comment\nabc"),
            Ok(("abc", " \t; comment\n"))
        );
    }

    #[test]
    fn test_ws_complex_mix() {
        let input = " \n ; first comment\n \t ; second comment\n\nabc";
        let (remaining, consumed) = ws(input).unwrap();
        assert_eq!(remaining, "abc");
        assert!(consumed.contains("; first comment"));
        assert!(consumed.contains("; second comment"));
    }

    #[test]
    fn test_ws_only_whitespace() {
        assert_eq!(ws("   "), Ok(("", "   ")));
    }

    #[test]
    fn test_ws_only_comment() {
        assert_eq!(ws("; just a comment"), Ok(("", "; just a comment")));
    }

    #[test]
    fn test_ws_carriage_return() {
        assert_eq!(ws("\r\nabc"), Ok(("abc", "\r\n")));
    }
}
