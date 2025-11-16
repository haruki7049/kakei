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

    mod basic_whitespace {
        use super::*;

        /// Tests parsing empty input.
        /// 
        /// Verifies that empty strings return empty consumed whitespace.
        #[test]
        fn empty() {
            assert_eq!(ws(""), Ok(("", "")));
        }

        /// Tests parsing input with no whitespace.
        /// 
        /// Verifies that non-whitespace input is left unparsed.
        #[test]
        fn no_whitespace() {
            assert_eq!(ws("abc"), Ok(("abc", "")));
        }

        /// Tests parsing a single space.
        /// 
        /// Verifies that a single space is correctly consumed.
        #[test]
        fn single_space() {
            assert_eq!(ws(" abc"), Ok(("abc", " ")));
        }

        /// Tests parsing multiple spaces.
        /// 
        /// Verifies that consecutive spaces are correctly consumed.
        #[test]
        fn multiple_spaces() {
            assert_eq!(ws("   abc"), Ok(("abc", "   ")));
        }

        /// Tests parsing a tab character.
        /// 
        /// Verifies that tab characters are correctly consumed as whitespace.
        #[test]
        fn tab() {
            assert_eq!(ws("\tabc"), Ok(("abc", "\t")));
        }

        /// Tests parsing a newline character.
        /// 
        /// Verifies that newline characters are correctly consumed as whitespace.
        #[test]
        fn newline() {
            assert_eq!(ws("\nabc"), Ok(("abc", "\n")));
        }

        /// Tests parsing mixed whitespace characters.
        /// 
        /// Verifies that combinations of spaces, tabs, and newlines are consumed together.
        #[test]
        fn mixed_whitespace() {
            assert_eq!(ws(" \t\n abc"), Ok(("abc", " \t\n ")));
        }

        /// Tests parsing only whitespace.
        /// 
        /// Verifies that input containing only whitespace is fully consumed.
        #[test]
        fn only_whitespace() {
            assert_eq!(ws("   "), Ok(("", "   ")));
        }

        /// Tests parsing carriage return and newline.
        /// 
        /// Verifies that CRLF line endings are correctly consumed.
        #[test]
        fn carriage_return() {
            assert_eq!(ws("\r\nabc"), Ok(("abc", "\r\n")));
        }
    }

    mod comments {
        use super::*;

        /// Tests parsing a simple comment.
        /// 
        /// Verifies that a comment line ending with newline is correctly consumed.
        #[test]
        fn simple() {
            assert_eq!(ws("; comment\nabc"), Ok(("abc", "; comment\n")));
        }

        /// Tests parsing a comment without a newline.
        /// 
        /// Verifies that comments at end of input (no trailing newline) are consumed.
        #[test]
        fn without_newline() {
            assert_eq!(ws("; comment"), Ok(("", "; comment")));
        }

        /// Tests parsing a comment with content.
        /// 
        /// Verifies that comment text with spaces and content is correctly consumed.
        #[test]
        fn with_content() {
            assert_eq!(
                ws("; This is a comment\nabc"),
                Ok(("abc", "; This is a comment\n"))
            );
        }

        /// Tests parsing multiple consecutive comments.
        /// 
        /// Verifies that multiple comment lines are all consumed together.
        #[test]
        fn multiple() {
            assert_eq!(
                ws("; comment 1\n; comment 2\nabc"),
                Ok(("abc", "; comment 1\n; comment 2\n"))
            );
        }

        /// Tests parsing only a comment.
        /// 
        /// Verifies that input containing only a comment is fully consumed.
        #[test]
        fn only_comment() {
            assert_eq!(ws("; just a comment"), Ok(("", "; just a comment")));
        }
    }

    mod mixed_whitespace_and_comments {
        use super::*;

        /// Tests parsing spaces followed by a comment.
        /// 
        /// Verifies that whitespace before comments is consumed together.
        #[test]
        fn comment_and_spaces() {
            assert_eq!(
                ws("  ; comment\n  abc"),
                Ok(("abc", "  ; comment\n  "))
            );
        }

        /// Tests parsing tabs and spaces before a comment.
        /// 
        /// Verifies that mixed whitespace before comments is correctly handled.
        #[test]
        fn spaces_before_comment() {
            assert_eq!(
                ws(" \t; comment\nabc"),
                Ok(("abc", " \t; comment\n"))
            );
        }

        /// Tests parsing a complex mix of whitespace and comments.
        /// 
        /// Verifies that multiple whitespace types mixed with multiple comments
        /// are all consumed together correctly.
        #[test]
        fn complex_mix() {
            let input = " \n ; first comment\n \t ; second comment\n\nabc";
            let (remaining, consumed) = ws(input).unwrap();
            assert_eq!(remaining, "abc");
            assert!(consumed.contains("; first comment"));
            assert!(consumed.contains("; second comment"));
        }
    }
}
