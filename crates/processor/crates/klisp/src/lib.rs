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

// Represents an S-expression (Sexpr)
// (Sexpr と Atom の enum 定義は変更なし)
#[derive(Debug, PartialEq, Clone)]
pub enum Sexpr {
    Atom(Atom),
    List(Vec<Sexpr>),
    DottedList(Vec<Sexpr>, Box<Sexpr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Nil,
    Symbol(String),
    Number(i64),
    String(String),
}

// A type alias for our parser's result type
// VerboseError を標準の Error に変更
type ParseResult<'a, O> = IResult<&'a str, O, Error<&'a str>>;

/// A helper parser that consumes whitespace or comments
/// Comments start with ';' and go to the end of the line
fn ws<'a>(input: &'a str) -> ParseResult<'a, &'a str> {
    // 修正: (input) を .parse(input) に変更
    recognize(many0(alt((
        recognize(pair(tag(";"), is_not("\n\r"))),
        multispace1,
    ))))
    .parse(input)
}

/// Parses a String (e.g., "Alice")
/// TODO: Does not yet handle escaped quotes \"
fn parse_string<'a>(input: &'a str) -> ParseResult<'a, Atom> {
    // 修正: (input) を .parse(input) に変更
    map(delimited(char('"'), is_not("\""), char('"')), |s: &str| {
        Atom::String(s.to_string())
    })
    .parse(input)
}

/// Parses a Number (e.g., 60000)
fn parse_number<'a>(input: &'a str) -> ParseResult<'a, Atom> {
    // 修正: (input) を .parse(input) に変更
    map(map_res(digit1, |s: &str| s.parse::<i64>()), Atom::Number).parse(input)
}

/// Parses a Symbol (e.g., define, ID-001, +)
fn parse_symbol<'a>(input: &'a str) -> ParseResult<'a, Atom> {
    // 修正: (input) を .parse(input) に変更
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

/// Parses any Atom
fn parse_atom<'a>(input: &'a str) -> ParseResult<'a, Atom> {
    // 修正: (input) を .parse(input) に変更
    alt((parse_number, parse_string, parse_symbol)).parse(input)
}

/// Parses a quoted S-expression (e.g., 'A or '(A B))
fn parse_quoted<'a>(input: &'a str) -> ParseResult<'a, Sexpr> {
    // 修正: (input) を .parse(input) に変更
    map(
        preceded(
            char('\''),
            parse_sexpr, // Recursively calls the main parser
        ),
        |sexpr| {
            // Converts 'A into (quote A)
            Sexpr::List(vec![Sexpr::Atom(Atom::Symbol("quote".to_string())), sexpr])
        },
    )
    .parse(input)
}

/// Parses a list '()', '(A B C)', or a dotted list '(A . B)', '(A B . C)'
fn parse_list<'a>(input: &'a str) -> ParseResult<'a, Sexpr> {
    // 内部のロジックは .parse() を使わない
    // .parse() は IResult を返す関数全体を呼び出すときに使う

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
            // Stop parsing elements
            current_input = peek_input;
            break;
        }

        // If not ')' or '.', parse one S-expression
        // 修正: parse_sexpr(current_input)? を parse_sexpr.parse(current_input)? に変更
        let (next_input, sexpr) = parse_sexpr.parse(current_input)?;
        elements.push(sexpr);
        current_input = next_input;
    }

    // We are now at the dot or the closing parenthesis
    let (input, _) = ws(current_input)?; // Consume whitespace

    // Check for the dot
    if let Ok((input, _)) = char::<&'a str, Error<&'a str>>('.').parse(input) {
        // Dot found: This is a DottedList

        // Must be followed by exactly one S-expression
        let (input, final_expr) = preceded(ws, parse_sexpr).parse(input)?;

        // Must be followed by ')'
        let (input, _) = preceded(ws, char(')')).parse(input)?;

        Ok((input, Sexpr::DottedList(elements, Box::new(final_expr))))
    } else {
        // No dot found: This must be a Proper List

        // Must be followed by ')'
        let (input, _) = char(')').parse(input)?;

        Ok((input, Sexpr::List(elements)))
    }
}

/// This is the main parser function
/// It tries to parse any valid S-expression
pub fn parse_sexpr<'a>(input: &'a str) -> ParseResult<'a, Sexpr> {
    // 修正: (input) を .parse(input) に変更
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
/// This is the main entry point for parsing a file or a buffer.
pub fn parse_toplevel<'a>(input: &'a str) -> ParseResult<'a, Vec<Sexpr>> {
    // 修正: (input) を .parse(input) に変更
    many0(parse_sexpr).parse(input)
}

// --- Test Module ---
// (テストモジュールは変更なし)

#[cfg(test)]
mod tests {
    // Import all functions and structs from the parent module (this file)
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

    #[test]
    fn test_parse_simple_list() {
        let input = "(a 1 \"two\")";
        let expected = Sexpr::List(vec![
            Sexpr::Atom(Atom::Symbol("a".to_string())),
            Sexpr::Atom(Atom::Number(1)),
            Sexpr::Atom(Atom::String("two".to_string())),
        ]);
        assert_eq!(parse_sexpr(input), Ok(("", expected)));
    }

    #[test]
    fn test_parse_empty_list() {
        assert_eq!(parse_sexpr("()"), Ok(("", Sexpr::Atom(Atom::Nil))));
        // Test with whitespace
        assert_eq!(parse_sexpr("( )"), Ok(("", Sexpr::Atom(Atom::Nil))));
    }

    #[test]
    fn test_parse_quoted() {
        let input = "'(a b)";
        let expected = Sexpr::List(vec![
            Sexpr::Atom(Atom::Symbol("quote".to_string())),
            Sexpr::List(vec![
                Sexpr::Atom(Atom::Symbol("a".to_string())),
                Sexpr::Atom(Atom::Symbol("b".to_string())),
            ]),
        ]);
        assert_eq!(parse_sexpr(input), Ok(("", expected)));
    }

    #[test]
    fn test_parse_dotted_list() {
        let input = "(a . b)";
        let expected = Sexpr::DottedList(
            vec![Sexpr::Atom(Atom::Symbol("a".to_string()))],
            Box::new(Sexpr::Atom(Atom::Symbol("b".to_string()))),
        );
        assert_eq!(parse_sexpr(input), Ok(("", expected)));
    }

    #[test]
    fn test_parse_complex_dotted_list() {
        let input = "(a b c . d)";
        let expected = Sexpr::DottedList(
            vec![
                Sexpr::Atom(Atom::Symbol("a".to_string())),
                Sexpr::Atom(Atom::Symbol("b".to_string())),
                Sexpr::Atom(Atom::Symbol("c".to_string())),
            ],
            Box::new(Sexpr::Atom(Atom::Symbol("d".to_string()))),
        );
        assert_eq!(parse_sexpr(input), Ok(("", expected)));
    }

    #[test]
    fn test_parse_comments() {
        let input = r#"
        ; This is a comment
        (a b) ; another comment
        ; (c d)
        "#;
        let expected = Sexpr::List(vec![
            Sexpr::Atom(Atom::Symbol("a".to_string())),
            Sexpr::Atom(Atom::Symbol("b".to_string())),
        ]);
        // parse_toplevel should parse the one valid expression and skip comments
        let result = parse_toplevel(input).unwrap().1;
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], expected);
    }

    #[test]
    fn test_full_example() {
        let input = r#"
        ; This is the employee table
        (define employee-table
          '( (ID-001 . ((name . "Alice") (dept . "Dev")))
             (ID-002 . ((name . "Bob") (dept . "Sales"))) ))
        
        ; A proper list
        (a b c)
        "#;

        // Use the main entry point `parse_toplevel`
        match parse_toplevel(input) {
            Ok((remaining_input, sexprs)) => {
                println!("--- Parser Success ---");
                println!("Parsed S-expressions:\n{:#?}", sexprs);
                println!("\nRemaining input:\n'{}'", remaining_input);

                // Check that we parsed 2 top-level expressions
                assert_eq!(sexprs.len(), 2);

                // Check the first expression structure (define ...)
                if let Sexpr::List(define_expr) = &sexprs[0] {
                    assert_eq!(define_expr.len(), 3);
                    assert_eq!(
                        define_expr[0],
                        Sexpr::Atom(Atom::Symbol("define".to_string()))
                    );
                } else {
                    panic!("First expression was not a list");
                }

                // Check that remaining input is empty or just whitespace
                assert!(remaining_input.trim().is_empty());
            }
            // 修正: convert_error を削除し、標準の Debug フォーマットでエラーを表示
            Err(nom::Err::Error(e) | nom::Err::Failure(e)) => {
                // This will fail the test if the parser errors
                panic!("--- Parser Error ---\n{:#?}", e);
            }
            Err(e) => panic!("Incomplete input: {:?}", e),
        }
    }
}
