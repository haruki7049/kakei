//! Integration tests for the kakei_lisp parser.
//!
//! These tests verify the complete parsing functionality using the public API.

use kakei_lisp::{Atom, Sexpr, parse};

/// Tests for basic list parsing functionality.
mod basic_lists {
    use super::*;

    /// Tests parsing a simple list with mixed atom types.
    ///
    /// Verifies that a list containing a symbol, number, and string
    /// is correctly parsed as a single proper list.
    #[test]
    fn simple_list() {
        let input = "(a 1 \"two\")";
        let expected = Sexpr::List(vec![
            Sexpr::Atom(Atom::Symbol("a".to_string())),
            Sexpr::Atom(Atom::Number(1)),
            Sexpr::Atom(Atom::String("two".to_string())),
        ]);

        let result = parse(input).unwrap().1;
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], expected);
    }

    /// Tests parsing empty lists.
    ///
    /// Verifies that () and lists with whitespace are parsed as Nil.
    #[test]
    fn empty_list() {
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
}

/// Tests for quoted expressions.
mod quoted_expressions {
    use super::*;

    /// Tests parsing a quoted list.
    ///
    /// Verifies that '(a b) is correctly expanded to (quote (a b)).
    #[test]
    fn quoted_list() {
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

    /// Tests parsing multiple quoted expressions.
    ///
    /// Verifies that multiple quote shorthands in sequence are each expanded.
    #[test]
    fn multiple_quoted_expressions() {
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

    /// Tests parsing nested quoted expressions.
    ///
    /// Verifies that (quote (quote (quote x))) structure is correctly built
    /// from nested quote syntax.
    #[test]
    fn nested_quoted_expressions() {
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

    /// Tests equivalence between quote shorthand and longform.
    ///
    /// Verifies that 'x produces the same result as (quote x).
    #[test]
    fn quote_shorthand_equivalence() {
        let shorthand = parse("'x").unwrap().1;
        let longform = parse("(quote x)").unwrap().1;
        
        assert_eq!(shorthand, longform);
    }
}

/// Tests for dotted list parsing.
mod dotted_lists {
    use super::*;

    /// Tests parsing a simple dotted pair.
    ///
    /// Verifies that (a . b) is correctly parsed as a dotted list.
    #[test]
    fn simple_dotted_list() {
        let input = "(a . b)";
        let expected = Sexpr::DottedList(
            vec![Sexpr::Atom(Atom::Symbol("a".to_string()))],
            Box::new(Sexpr::Atom(Atom::Symbol("b".to_string()))),
        );

        let result = parse(input).unwrap().1;
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], expected);
    }

    /// Tests parsing a dotted list with multiple elements before the dot.
    ///
    /// Verifies that (a b c . d) is correctly parsed as a dotted list.
    #[test]
    fn complex_dotted_list() {
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

    /// Tests parsing a dotted list with a nested list after the dot.
    ///
    /// Verifies that (a b . (c d)) is correctly parsed.
    #[test]
    fn dotted_list_with_nested_list() {
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

    /// Tests parsing mixed dotted and proper lists.
    ///
    /// Verifies that a list containing both dotted and proper sublists
    /// is correctly parsed.
    #[test]
    fn mixed_dotted_and_proper_lists() {
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
}

/// Tests for whitespace and comment handling.
mod whitespace_and_comments {
    use super::*;

    /// Tests parsing with comments interspersed.
    ///
    /// Verifies that comments are properly skipped and don't affect parsing.
    #[test]
    fn toplevel_with_comments() {
        let input = r#"
            ; This is a comment
            (a b) ; another comment
            ; (c d)
            "#;
        let expected = Sexpr::List(vec![
            Sexpr::Atom(Atom::Symbol("a".to_string())),
            Sexpr::Atom(Atom::Symbol("b".to_string())),
        ]);

        let result = parse(input).unwrap().1;
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], expected);
    }

    /// Tests parsing with inline comments.
    ///
    /// Verifies that comments within an S-expression are properly handled.
    #[test]
    fn s_expression_with_inline_comments() {
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

    /// Tests parsing with various whitespace styles.
    ///
    /// Verifies that different whitespace combinations produce the same result.
    #[test]
    fn whitespace_variations() {
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
}

/// Tests for complex nested structures.
mod complex_nesting {
    use super::*;

    /// Tests parsing deeply nested lists.
    ///
    /// Verifies that deeply nested structures like (a (b (c (d (e)))))
    /// are correctly parsed.
    #[test]
    fn deeply_nested_lists() {
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

    /// Tests parsing a full example with multiple top-level expressions.
    ///
    /// Verifies that a realistic Lisp program with define and function calls
    /// is correctly parsed.
    #[test]
    fn full_example_toplevel() {
        let input = r#"
            ; This is the employee table
            (define employee-table
              '( (ID-001 . ((name . "Alice") (dept . "Dev")))
                 (ID-002 . ((name . "Bob") (dept . "Sales"))) ))

            ; A proper list
            (a b c)
            "#;

        match parse(input) {
            Ok((remaining_input, sexprs)) => {
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
                panic!("--- Parser Error ---\n{:#?}", e);
            }
            Err(e) => panic!("Incomplete input: {:?}", e),
        }
    }
}

/// Tests for real-world Lisp constructs.
mod lisp_constructs {
    use super::*;

    /// Tests parsing an arithmetic expression.
    ///
    /// Verifies that nested arithmetic operations are correctly parsed.
    #[test]
    fn arithmetic_expression() {
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

    /// Tests parsing a lambda expression.
    ///
    /// Verifies that lambda function definitions are correctly parsed.
    #[test]
    fn lambda_expression() {
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

    /// Tests parsing a let binding expression.
    ///
    /// Verifies that let expressions with bindings are correctly parsed.
    #[test]
    fn let_binding() {
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

    /// Tests parsing a cond expression.
    ///
    /// Verifies that conditional expressions with multiple clauses are parsed.
    #[test]
    fn cond_expression() {
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

    /// Tests parsing an association list.
    ///
    /// Verifies that association lists (alists) are correctly parsed.
    #[test]
    fn association_list() {
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

    /// Tests parsing a complex real-world example.
    ///
    /// Verifies that a realistic factorial function definition is correctly parsed.
    #[test]
    fn complex_real_world_example() {
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
}

/// Tests for various atom types.
mod atom_types {
    use super::*;

    /// Tests parsing numbers of various magnitudes.
    ///
    /// Verifies that numbers from 0 to i64::MAX are correctly parsed.
    #[test]
    fn numbers_with_various_magnitudes() {
        let input = "0 1 42 1000 1000000 9223372036854775807";
        let result = parse(input).unwrap().1;
        assert_eq!(result.len(), 6);
        
        let expected_numbers = vec![0, 1, 42, 1000, 1000000, 9223372036854775807];
        for (i, num) in expected_numbers.iter().enumerate() {
            assert_eq!(result[i], Sexpr::Atom(Atom::Number(*num)));
        }
    }

    /// Tests parsing various symbol types.
    ///
    /// Verifies that different symbol styles (identifiers, operators) are parsed.
    #[test]
    fn various_symbols() {
        let input = "define lambda set! null? list->vector + - * / < > =";
        let result = parse(input).unwrap().1;
        assert_eq!(result.len(), 13);
        
        for sexpr in &result {
            assert!(matches!(sexpr, Sexpr::Atom(Atom::Symbol(_))));
        }
    }

    /// Tests parsing strings with special content.
    ///
    /// Verifies that strings with various characters are correctly parsed.
    #[test]
    fn strings_with_special_content() {
        let input = r#""hello" "world 123" "with-dashes" "with_underscores" "with spaces!""#;
        let result = parse(input).unwrap().1;
        assert_eq!(result.len(), 5);
        
        for sexpr in &result {
            assert!(matches!(sexpr, Sexpr::Atom(Atom::String(_))));
        }
    }

    /// Tests parsing single atoms of each type.
    ///
    /// Verifies that number, string, and symbol atoms are individually parsed.
    #[test]
    fn single_atom_types() {
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
}

/// Tests for edge cases and empty input.
mod edge_cases {
    use super::*;

    /// Tests parsing empty input.
    ///
    /// Verifies that an empty string produces an empty result.
    #[test]
    fn empty_string() {
        let result = parse("").unwrap().1;
        assert_eq!(result.len(), 0);
    }

    /// Tests parsing only whitespace.
    ///
    /// Verifies that whitespace-only input produces an empty result.
    #[test]
    fn only_whitespace() {
        let result = parse("   \n\t  ").unwrap().1;
        assert_eq!(result.len(), 0);
    }

    /// Tests parsing only comments.
    ///
    /// Verifies that comment-only input produces an empty result.
    #[test]
    fn only_comments() {
        let input = r#"
            ; Just a comment
            ; Another comment
            ; And one more
        "#;
        let result = parse(input).unwrap().1;
        assert_eq!(result.len(), 0);
    }

    /// Tests parsing multiple expressions with varying complexity.
    ///
    /// Verifies that a mix of different expression types can be parsed together.
    #[test]
    fn multiple_expressions_with_varying_complexity() {
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
}
