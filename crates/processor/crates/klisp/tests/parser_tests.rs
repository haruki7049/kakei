use kakei_lisp::{Atom, Sexpr, parse};

// Basic list parsing tests
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

// Additional integration tests for complex nested expressions
#[test]
fn test_parse_deeply_nested_lists() {
    let input = "(a (b (c (d (e)))))";
    let result = parse(input).unwrap().1;
    assert_eq!(result.len(), 1);
    
    // Verify deep nesting structure
    if let Sexpr::List(outer) = &result[0] {
        assert_eq!(outer.len(), 2);
        assert_eq!(outer[0], Sexpr::Atom(Atom::Symbol("a".to_string())));
        
        if let Sexpr::List(level1) = &outer[1] {
            assert_eq!(level1[0], Sexpr::Atom(Atom::Symbol("b".to_string())));
        } else {
            panic!("Expected nested list");
        }
    } else {
        panic!("Expected outer list");
    }
}

#[test]
fn test_parse_multiple_quoted_expressions() {
    let input = "'a 'b '(c d)";
    let result = parse(input).unwrap().1;
    assert_eq!(result.len(), 3);
    
    // Each should be wrapped in quote
    for sexpr in &result {
        if let Sexpr::List(list) = sexpr {
            assert_eq!(list[0], Sexpr::Atom(Atom::Symbol("quote".to_string())));
        } else {
            panic!("Expected quoted expression");
        }
    }
}

#[test]
fn test_parse_mixed_dotted_and_proper_lists() {
    let input = "((a . b) (c d) (e f . g))";
    let result = parse(input).unwrap().1;
    assert_eq!(result.len(), 1);
    
    if let Sexpr::List(items) = &result[0] {
        assert_eq!(items.len(), 3);
        
        // First is dotted list
        assert!(matches!(items[0], Sexpr::DottedList(_, _)));
        
        // Second is proper list
        assert!(matches!(items[1], Sexpr::List(_)));
        
        // Third is dotted list
        assert!(matches!(items[2], Sexpr::DottedList(_, _)));
    } else {
        panic!("Expected outer list");
    }
}

#[test]
fn test_parse_arithmetic_expression() {
    let input = "(+ (* 2 3) (- 10 5))";
    let result = parse(input).unwrap().1;
    assert_eq!(result.len(), 1);
    
    if let Sexpr::List(expr) = &result[0] {
        assert_eq!(expr[0], Sexpr::Atom(Atom::Symbol("+".to_string())));
        assert_eq!(expr.len(), 3);
    } else {
        panic!("Expected list");
    }
}

#[test]
fn test_parse_lambda_expression() {
    let input = "(lambda (x y) (+ x y))";
    let result = parse(input).unwrap().1;
    assert_eq!(result.len(), 1);
    
    if let Sexpr::List(lambda_expr) = &result[0] {
        assert_eq!(lambda_expr[0], Sexpr::Atom(Atom::Symbol("lambda".to_string())));
        assert_eq!(lambda_expr.len(), 3);
    } else {
        panic!("Expected lambda expression");
    }
}

#[test]
fn test_parse_let_binding() {
    let input = "(let ((x 10) (y 20)) (+ x y))";
    let result = parse(input).unwrap().1;
    assert_eq!(result.len(), 1);
    
    if let Sexpr::List(let_expr) = &result[0] {
        assert_eq!(let_expr[0], Sexpr::Atom(Atom::Symbol("let".to_string())));
        assert_eq!(let_expr.len(), 3);
    } else {
        panic!("Expected let expression");
    }
}

#[test]
fn test_parse_cond_expression() {
    let input = "(cond ((> x 0) 'positive) ((< x 0) 'negative) (else 'zero))";
    let result = parse(input).unwrap().1;
    assert_eq!(result.len(), 1);
    
    if let Sexpr::List(cond_expr) = &result[0] {
        assert_eq!(cond_expr[0], Sexpr::Atom(Atom::Symbol("cond".to_string())));
        assert_eq!(cond_expr.len(), 4); // cond + 3 clauses
    } else {
        panic!("Expected cond expression");
    }
}

#[test]
fn test_parse_association_list() {
    let input = "((name . \"Alice\") (age . 30) (dept . \"Engineering\"))";
    let result = parse(input).unwrap().1;
    assert_eq!(result.len(), 1);
    
    if let Sexpr::List(alist) = &result[0] {
        assert_eq!(alist.len(), 3);
        for item in alist {
            assert!(matches!(item, Sexpr::DottedList(_, _)));
        }
    } else {
        panic!("Expected association list");
    }
}

#[test]
fn test_parse_numbers_with_various_magnitudes() {
    let input = "0 1 42 1000 1000000 9223372036854775807";
    let result = parse(input).unwrap().1;
    assert_eq!(result.len(), 6);
    
    let expected_numbers = vec![0, 1, 42, 1000, 1000000, 9223372036854775807];
    for (i, num) in expected_numbers.iter().enumerate() {
        assert_eq!(result[i], Sexpr::Atom(Atom::Number(*num)));
    }
}

#[test]
fn test_parse_various_symbols() {
    let input = "define lambda set! null? list->vector + - * / < > =";
    let result = parse(input).unwrap().1;
    assert_eq!(result.len(), 13);
    
    for sexpr in &result {
        assert!(matches!(sexpr, Sexpr::Atom(Atom::Symbol(_))));
    }
}

#[test]
fn test_parse_strings_with_special_content() {
    let input = r#""hello" "world 123" "with-dashes" "with_underscores" "with spaces!""#;
    let result = parse(input).unwrap().1;
    assert_eq!(result.len(), 5);
    
    for sexpr in &result {
        assert!(matches!(sexpr, Sexpr::Atom(Atom::String(_))));
    }
}

#[test]
fn test_parse_nested_quoted_expressions() {
    let input = "(quote (quote (quote x)))";
    let result = parse(input).unwrap().1;
    assert_eq!(result.len(), 1);
    
    // Verify nested quotes
    let mut current = &result[0];
    let mut quote_count = 0;
    
    loop {
        if let Sexpr::List(list) = current {
            if list[0] == Sexpr::Atom(Atom::Symbol("quote".to_string())) {
                quote_count += 1;
                current = &list[1];
            } else {
                break;
            }
        } else {
            break;
        }
    }
    
    assert_eq!(quote_count, 3);
}

#[test]
fn test_parse_complex_real_world_example() {
    let input = r#"
        ; Define a function to calculate factorial
        (define (factorial n)
          (if (<= n 1)
              1
              (* n (factorial (- n 1)))))
        
        ; Use the function
        (factorial 5)
    "#;
    
    let result = parse(input).unwrap().1;
    assert_eq!(result.len(), 2); // Two top-level expressions
    
    // First should be define
    if let Sexpr::List(define_expr) = &result[0] {
        assert_eq!(define_expr[0], Sexpr::Atom(Atom::Symbol("define".to_string())));
    } else {
        panic!("Expected define expression");
    }
    
    // Second should be function call
    if let Sexpr::List(call_expr) = &result[1] {
        assert_eq!(call_expr[0], Sexpr::Atom(Atom::Symbol("factorial".to_string())));
        assert_eq!(call_expr[1], Sexpr::Atom(Atom::Number(5)));
    } else {
        panic!("Expected function call");
    }
}

// Tests for edge cases and error scenarios
#[test]
fn test_parse_empty_string() {
    let result = parse("").unwrap().1;
    assert_eq!(result.len(), 0);
}

#[test]
fn test_parse_only_whitespace() {
    let result = parse("   \n\t  ").unwrap().1;
    assert_eq!(result.len(), 0);
}

#[test]
fn test_parse_only_comments() {
    let input = r#"
        ; Just a comment
        ; Another comment
        ; And one more
    "#;
    let result = parse(input).unwrap().1;
    assert_eq!(result.len(), 0);
}

#[test]
fn test_parse_single_atom_types() {
    // Number
    let result = parse("42").unwrap().1;
    assert_eq!(result, vec![Sexpr::Atom(Atom::Number(42))]);
    
    // String
    let result = parse(r#""hello""#).unwrap().1;
    assert_eq!(result, vec![Sexpr::Atom(Atom::String("hello".to_string()))]);
    
    // Symbol
    let result = parse("symbol").unwrap().1;
    assert_eq!(result, vec![Sexpr::Atom(Atom::Symbol("symbol".to_string()))]);
}

#[test]
fn test_parse_dotted_list_with_nested_list() {
    let input = "(a b . (c d))";
    let result = parse(input).unwrap().1;
    assert_eq!(result.len(), 1);
    
    if let Sexpr::DottedList(vec, tail) = &result[0] {
        assert_eq!(vec.len(), 2);
        assert!(matches!(**tail, Sexpr::List(_)));
    } else {
        panic!("Expected dotted list");
    }
}

#[test]
fn test_parse_whitespace_variations() {
    // Test different whitespace styles produce same result
    let inputs = vec![
        "(a b c)",
        "( a b c )",
        "(  a  b  c  )",
        "(\na\nb\nc\n)",
        "(\ta\tb\tc\t)",
    ];
    
    let expected = Sexpr::List(vec![
        Sexpr::Atom(Atom::Symbol("a".to_string())),
        Sexpr::Atom(Atom::Symbol("b".to_string())),
        Sexpr::Atom(Atom::Symbol("c".to_string())),
    ]);
    
    for input in inputs {
        let result = parse(input).unwrap().1;
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], expected);
    }
}

#[test]
fn test_parse_quote_shorthand_equivalence() {
    // 'x should be equivalent to (quote x)
    let shorthand = parse("'x").unwrap().1;
    let longform = parse("(quote x)").unwrap().1;
    
    assert_eq!(shorthand, longform);
}

#[test]
fn test_parse_multiple_expressions_with_varying_complexity() {
    let input = r#"
        42
        "string"
        symbol
        (a b)
        '(c d)
        (e . f)
        ((nested (list)))
    "#;
    
    let result = parse(input).unwrap().1;
    assert_eq!(result.len(), 7);
}

#[test]
fn test_parse_s_expression_with_inline_comments() {
    let input = r#"
        (define x ; variable name
                10) ; value
    "#;
    
    let result = parse(input).unwrap().1;
    assert_eq!(result.len(), 1);
    
    if let Sexpr::List(define_expr) = &result[0] {
        assert_eq!(define_expr[0], Sexpr::Atom(Atom::Symbol("define".to_string())));
        assert_eq!(define_expr[1], Sexpr::Atom(Atom::Symbol("x".to_string())));
        assert_eq!(define_expr[2], Sexpr::Atom(Atom::Number(10)));
    } else {
        panic!("Expected list");
    }
}
