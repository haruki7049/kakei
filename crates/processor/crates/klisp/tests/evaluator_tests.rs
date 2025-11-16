//! Integration tests for the kakei_lisp evaluator.
//!
//! These tests verify the complete evaluation functionality using the public API.

use kakei_lisp::{create_global_env, eval, parse, Value};

/// Helper function to parse and evaluate a single expression.
fn eval_str(input: &str) -> Result<Value, String> {
    let (_, sexprs) = parse(input).map_err(|e| format!("Parse error: {:?}", e))?;
    if sexprs.len() != 1 {
        return Err("Expected exactly one expression".to_string());
    }
    let mut env = create_global_env();
    eval(&sexprs[0], &mut env).map_err(|e| format!("Eval error: {}", e))
}

/// Helper function to parse and evaluate multiple expressions, returning the last value.
fn eval_program(input: &str) -> Result<Value, String> {
    let (_, sexprs) = parse(input).map_err(|e| format!("Parse error: {:?}", e))?;
    let mut env = create_global_env();
    let mut result = Value::Nil;
    for sexpr in sexprs {
        result = eval(&sexpr, &mut env).map_err(|e| format!("Eval error: {}", e))?;
    }
    Ok(result)
}

/// Tests for basic value evaluation.
mod basic_values {
    use super::*;

    #[test]
    fn test_eval_number() {
        let result = eval_str("42").unwrap();
        assert_eq!(result, Value::Number(42));
    }

    #[test]
    fn test_eval_string() {
        let result = eval_str(r#""hello""#).unwrap();
        assert_eq!(result, Value::String("hello".to_string()));
    }

    #[test]
    fn test_eval_quote_symbol() {
        let result = eval_str("'x").unwrap();
        assert_eq!(result, Value::Symbol("x".to_string()));
    }

    #[test]
    fn test_eval_quote_list() {
        let result = eval_str("'(a b c)").unwrap();
        // Should be a cons list
        if let Value::Cons(car, _) = result {
            assert_eq!(*car, Value::Symbol("a".to_string()));
        } else {
            panic!("Expected cons list");
        }
    }
}

/// Tests for define special form.
mod define_tests {
    use super::*;

    #[test]
    fn test_define_simple() {
        let program = r#"
            (define x 42)
            x
        "#;
        let result = eval_program(program).unwrap();
        assert_eq!(result, Value::Number(42));
    }

    #[test]
    fn test_define_string() {
        let program = r#"
            (define name "Alice")
            name
        "#;
        let result = eval_program(program).unwrap();
        assert_eq!(result, Value::String("Alice".to_string()));
    }

    #[test]
    fn test_define_expression() {
        let program = r#"
            (define x (cons 1 2))
            x
        "#;
        let result = eval_program(program).unwrap();
        if let Value::Cons(car, cdr) = result {
            assert_eq!(*car, Value::Number(1));
            assert_eq!(*cdr, Value::Number(2));
        } else {
            panic!("Expected cons cell");
        }
    }
}

/// Tests for lambda and function application.
mod lambda_tests {
    use super::*;

    #[test]
    fn test_lambda_identity() {
        let program = r#"
            (define id (lambda (x) x))
            (id 42)
        "#;
        let result = eval_program(program).unwrap();
        assert_eq!(result, Value::Number(42));
    }

    #[test]
    fn test_lambda_two_params() {
        let program = r#"
            (define first (lambda (x y) x))
            (first 10 20)
        "#;
        let result = eval_program(program).unwrap();
        assert_eq!(result, Value::Number(10));
    }

    #[test]
    fn test_lambda_closure() {
        let program = r#"
            (define make-adder (lambda (n) (lambda (x) (cons n x))))
            (define add5 (make-adder 5))
            (add5 10)
        "#;
        let result = eval_program(program).unwrap();
        if let Value::Cons(car, cdr) = result {
            assert_eq!(*car, Value::Number(5));
            assert_eq!(*cdr, Value::Number(10));
        } else {
            panic!("Expected cons cell");
        }
    }

    #[test]
    fn test_lambda_multiple_body_expressions() {
        let program = r#"
            (define f (lambda (x)
                (define y (cons x 1))
                y))
            (f 42)
        "#;
        let result = eval_program(program).unwrap();
        if let Value::Cons(car, cdr) = result {
            assert_eq!(*car, Value::Number(42));
            assert_eq!(*cdr, Value::Number(1));
        } else {
            panic!("Expected cons cell");
        }
    }
}

/// Tests for if special form.
mod if_tests {
    use super::*;

    #[test]
    fn test_if_true_number() {
        let result = eval_str("(if 1 10 20)").unwrap();
        assert_eq!(result, Value::Number(10));
    }

    #[test]
    fn test_if_false_nil() {
        let result = eval_str("(if () 10 20)").unwrap();
        assert_eq!(result, Value::Number(20));
    }

    #[test]
    fn test_if_with_equal() {
        let program = r#"
            (if (equal? 5 5) "yes" "no")
        "#;
        let result = eval_program(program).unwrap();
        assert_eq!(result, Value::String("yes".to_string()));
    }

    #[test]
    fn test_if_nested() {
        let program = r#"
            (if (equal? 1 2)
                "first"
                (if (equal? 2 2) "second" "third"))
        "#;
        let result = eval_program(program).unwrap();
        assert_eq!(result, Value::String("second".to_string()));
    }
}

/// Tests for cons, car, cdr operations.
mod cons_car_cdr_tests {
    use super::*;

    #[test]
    fn test_cons() {
        let result = eval_str("(cons 1 2)").unwrap();
        if let Value::Cons(car, cdr) = result {
            assert_eq!(*car, Value::Number(1));
            assert_eq!(*cdr, Value::Number(2));
        } else {
            panic!("Expected cons cell");
        }
    }

    #[test]
    fn test_car() {
        let result = eval_str("(car (cons 1 2))").unwrap();
        assert_eq!(result, Value::Number(1));
    }

    #[test]
    fn test_cdr() {
        let result = eval_str("(cdr (cons 1 2))").unwrap();
        assert_eq!(result, Value::Number(2));
    }

    #[test]
    fn test_cons_list() {
        let program = r#"
            (cons 1 (cons 2 (cons 3 ())))
        "#;
        let result = eval_program(program).unwrap();
        // Should be (1 2 3)
        if let Value::Cons(car1, cdr1) = result {
            assert_eq!(*car1, Value::Number(1));
            if let Value::Cons(car2, cdr2) = cdr1.as_ref() {
                assert_eq!(car2.as_ref(), &Value::Number(2));
                if let Value::Cons(car3, cdr3) = cdr2.as_ref() {
                    assert_eq!(car3.as_ref(), &Value::Number(3));
                    assert_eq!(cdr3.as_ref(), &Value::Nil);
                } else {
                    panic!("Expected third cons");
                }
            } else {
                panic!("Expected second cons");
            }
        } else {
            panic!("Expected first cons");
        }
    }
}

/// Tests for null? predicate.
mod null_tests {
    use super::*;

    #[test]
    fn test_null_true() {
        let result = eval_str("(null? ())").unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_null_false_number() {
        let result = eval_str("(null? 42)").unwrap();
        assert_eq!(result, Value::Bool(false));
    }

    #[test]
    fn test_null_false_cons() {
        let result = eval_str("(null? (cons 1 2))").unwrap();
        assert_eq!(result, Value::Bool(false));
    }
}

/// Tests for equal? predicate.
mod equal_tests {
    use super::*;

    #[test]
    fn test_equal_numbers_true() {
        let result = eval_str("(equal? 42 42)").unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_equal_numbers_false() {
        let result = eval_str("(equal? 42 43)").unwrap();
        assert_eq!(result, Value::Bool(false));
    }

    #[test]
    fn test_equal_strings_true() {
        let result = eval_str(r#"(equal? "hello" "hello")"#).unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_equal_strings_false() {
        let result = eval_str(r#"(equal? "hello" "world")"#).unwrap();
        assert_eq!(result, Value::Bool(false));
    }

    #[test]
    fn test_equal_symbols_true() {
        let result = eval_str("(equal? 'foo 'foo)").unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_equal_symbols_false() {
        let result = eval_str("(equal? 'foo 'bar)").unwrap();
        assert_eq!(result, Value::Bool(false));
    }

    #[test]
    fn test_equal_cons_true() {
        let result = eval_str("(equal? (cons 1 2) (cons 1 2))").unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_equal_cons_false() {
        let result = eval_str("(equal? (cons 1 2) (cons 1 3))").unwrap();
        assert_eq!(result, Value::Bool(false));
    }
}

/// Tests for assoc function.
mod assoc_tests {
    use super::*;

    #[test]
    fn test_assoc_found() {
        let program = r#"
            (define alist '((a . 1) (b . 2) (c . 3)))
            (assoc 'b alist)
        "#;
        let result = eval_program(program).unwrap();
        // Should return (b . 2)
        if let Value::Cons(car, cdr) = result {
            assert_eq!(*car, Value::Symbol("b".to_string()));
            assert_eq!(*cdr, Value::Number(2));
        } else {
            panic!("Expected cons cell");
        }
    }

    #[test]
    fn test_assoc_not_found() {
        let program = r#"
            (define alist '((a . 1) (b . 2)))
            (assoc 'c alist)
        "#;
        let result = eval_program(program).unwrap();
        assert_eq!(result, Value::Nil);
    }

    #[test]
    fn test_assoc_with_strings() {
        let program = r#"
            (define alist '(("name" . "Alice") ("age" . 30)))
            (assoc "name" alist)
        "#;
        let result = eval_program(program).unwrap();
        // Should return ("name" . "Alice")
        if let Value::Cons(car, cdr) = result {
            assert_eq!(*car, Value::String("name".to_string()));
            assert_eq!(*cdr, Value::String("Alice".to_string()));
        } else {
            panic!("Expected cons cell");
        }
    }

    #[test]
    fn test_assoc_nested() {
        let program = r#"
            (define alist '((name . "Alice") (dept . "Dev")))
            (cdr (assoc 'dept alist))
        "#;
        let result = eval_program(program).unwrap();
        assert_eq!(result, Value::String("Dev".to_string()));
    }
}

/// Tests for group-by function.
mod group_by_tests {
    use super::*;

    #[test]
    fn test_group_by_simple() {
        let program = r#"
            (define table '(
                (ID-001 . ((dept . "Dev")))
                (ID-002 . ((dept . "Sales")))
                (ID-003 . ((dept . "Dev")))
            ))
            (define get-dept (lambda (row) (cdr (assoc 'dept (cdr row)))))
            (group-by table get-dept)
        "#;
        let result = eval_program(program).unwrap();

        // Result should be an association list of groups
        // Each group is (dept-name . list-of-rows)
        assert!(matches!(result, Value::Cons(_, _)));
    }

    #[test]
    fn test_group_by_full_example() {
        let program = r#"
            ; Define employee table with row IDs and data
            (define employee-table '(
                (ID-001 . ((name . "Alice") (dept . "Dev") (salary . 60000)))
                (ID-002 . ((name . "Bob") (dept . "Sales") (salary . 70000)))
                (ID-003 . ((name . "Carol") (dept . "Dev") (salary . 65000)))
                (ID-004 . ((name . "Dave") (dept . "HR") (salary . 55000)))
            ))
            
            ; Define lambda to extract department from a row
            (define get-dept (lambda (row) (cdr (assoc 'dept (cdr row)))))
            
            ; Group by department
            (define grouped (group-by employee-table get-dept))
            
            ; Return the grouped table
            grouped
        "#;
        let result = eval_program(program).unwrap();

        // The result should be a list of (dept . rows) pairs
        assert!(matches!(result, Value::Cons(_, _)));
        
        // Count the number of groups (should be 3: Dev, Sales, HR)
        let mut count = 0;
        let mut current = &result;
        loop {
            match current {
                Value::Nil => break,
                Value::Cons(_, cdr) => {
                    count += 1;
                    current = cdr.as_ref();
                }
                _ => panic!("Expected proper list"),
            }
        }
        assert_eq!(count, 3);
    }
}

/// Tests for complex real-world examples.
mod complex_examples {
    use super::*;

    #[test]
    fn test_list_operations() {
        let program = r#"
            (define list1 (cons 1 (cons 2 (cons 3 ()))))
            (define list2 (cons 4 (cons 5 ())))
            (define first-of-list1 (car list1))
            (define rest-of-list1 (cdr list1))
            first-of-list1
        "#;
        let result = eval_program(program).unwrap();
        assert_eq!(result, Value::Number(1));
    }

    #[test]
    fn test_higher_order_function() {
        let program = r#"
            (define apply-twice (lambda (f x) (f (f x))))
            (define add-pair (lambda (p) (cons (car p) (car p))))
            (apply-twice add-pair (cons 5 10))
        "#;
        let result = eval_program(program).unwrap();
        if let Value::Cons(car, cdr) = result {
            assert_eq!(*car, Value::Number(5));
            assert_eq!(*cdr, Value::Number(5));
        } else {
            panic!("Expected cons cell");
        }
    }

    #[test]
    fn test_conditional_with_predicates() {
        let program = r#"
            (define check (lambda (x)
                (if (null? x)
                    "empty"
                    "not empty")))
            (check ())
        "#;
        let result = eval_program(program).unwrap();
        assert_eq!(result, Value::String("empty".to_string()));
    }

    #[test]
    fn test_nested_data_structures() {
        let program = r#"
            (define person '((name . "Alice") (age . 30) (city . "NYC")))
            (define get-name (lambda (p) (cdr (assoc 'name p))))
            (get-name person)
        "#;
        let result = eval_program(program).unwrap();
        assert_eq!(result, Value::String("Alice".to_string()));
    }
}
