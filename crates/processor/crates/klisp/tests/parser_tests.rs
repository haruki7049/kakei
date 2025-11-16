use kakei_lisp::{Atom, Sexpr, parse};

#[test]
fn test_parse_simple_list() {
    let input = "(a 1 \"two\")";
    let expected = Sexpr::List(vec![
        Sexpr::Atom(Atom::Symbol("a".to_string())),
        Sexpr::Atom(Atom::Number(1)),
        Sexpr::Atom(Atom::String("two".to_string())),
    ]);

    // Test that `parse` correctly returns a Vec with one element
    let result = parse(input).unwrap().1;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], expected);
}

#[test]
fn test_parse_empty_list() {
    let expected = Sexpr::Atom(Atom::Nil);

    // Test `()`
    let result_1 = parse("()").unwrap().1;
    assert_eq!(result_1.len(), 1);
    assert_eq!(result_1[0], expected);

    // Test with whitespace `( )`
    let result_2 = parse("( )").unwrap().1;
    assert_eq!(result_2.len(), 1);
    assert_eq!(result_2[0], expected);
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

    let result = parse(input).unwrap().1;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], expected);
}

#[test]
fn test_parse_dotted_list() {
    let input = "(a . b)";
    let expected = Sexpr::DottedList(
        vec![Sexpr::Atom(Atom::Symbol("a".to_string()))],
        Box::new(Sexpr::Atom(Atom::Symbol("b".to_string()))),
    );

    let result = parse(input).unwrap().1;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], expected);
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

    let result = parse(input).unwrap().1;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], expected);
}

#[test]
fn test_parse_toplevel_with_comments() {
    let input = r#"
        ; This is a comment
        (a b) ; another comment
        ; (c d)
        "#;
    let expected = Sexpr::List(vec![
        Sexpr::Atom(Atom::Symbol("a".to_string())),
        Sexpr::Atom(Atom::Symbol("b".to_string())),
    ]);

    // parse should parse the one valid expression and skip comments
    let result = parse(input).unwrap().1;
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], expected);
}

#[test]
fn test_full_example_toplevel() {
    let input = r#"
        ; This is the employee table
        (define employee-table
          '( (ID-001 . ((name . "Alice") (dept . "Dev")))
             (ID-002 . ((name . "Bob") (dept . "Sales"))) ))

        ; A proper list
        (a b c)
        "#;

    // Use the main entry point `parse`
    match parse(input) {
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

            // Check the second expression
            if let Sexpr::List(list_expr) = &sexprs[1] {
                assert_eq!(list_expr.len(), 3);
                assert_eq!(list_expr[0], Sexpr::Atom(Atom::Symbol("a".to_string())));
            } else {
                panic!("Second expression was not a list");
            }

            // Check that remaining input is empty or just whitespace
            assert!(remaining_input.trim().is_empty());
        }
        Err(nom::Err::Error(e) | nom::Err::Failure(e)) => {
            // This will fail the test if the parser errors
            panic!("--- Parser Error ---\n{:#?}", e);
        }
        Err(e) => panic!("Incomplete input: {:?}", e),
    }
}
