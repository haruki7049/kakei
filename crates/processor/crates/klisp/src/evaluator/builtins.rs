//! Built-in primitive functions for the Lisp dialect.
//!
//! This module defines the standard library of built-in functions.

use super::value::{Environment, EvalError, Value};
use std::collections::HashMap;
use std::rc::Rc;

/// Create a global environment with all built-in functions.
pub fn create_global_env() -> Environment {
    let mut env = Environment::new();

    // Basic list operations
    env.define("cons".to_string(), Value::Primitive(builtin_cons));
    env.define("car".to_string(), Value::Primitive(builtin_car));
    env.define("cdr".to_string(), Value::Primitive(builtin_cdr));
    env.define("null?".to_string(), Value::Primitive(builtin_null));

    // Comparison and equality
    env.define("equal?".to_string(), Value::Primitive(builtin_equal));

    // Association list operations
    env.define("assoc".to_string(), Value::Primitive(builtin_assoc));

    // Table manipulation
    env.define("group-by".to_string(), Value::Primitive(builtin_group_by));

    env
}

/// Built-in cons function: (cons a b) creates a cons cell.
fn builtin_cons(args: &[Value], _env: &mut Environment) -> Result<Value, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::ArityError {
            expected: "2".to_string(),
            got: args.len(),
        });
    }
    Ok(Value::Cons(
        Rc::new(args[0].clone()),
        Rc::new(args[1].clone()),
    ))
}

/// Built-in car function: (car pair) gets the first element.
fn builtin_car(args: &[Value], _env: &mut Environment) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::ArityError {
            expected: "1".to_string(),
            got: args.len(),
        });
    }
    match &args[0] {
        Value::Cons(car, _) => Ok(car.as_ref().clone()),
        _ => Err(EvalError::TypeError("car requires a cons cell".to_string())),
    }
}

/// Built-in cdr function: (cdr pair) gets the second element.
fn builtin_cdr(args: &[Value], _env: &mut Environment) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::ArityError {
            expected: "1".to_string(),
            got: args.len(),
        });
    }
    match &args[0] {
        Value::Cons(_, cdr) => Ok(cdr.as_ref().clone()),
        _ => Err(EvalError::TypeError("cdr requires a cons cell".to_string())),
    }
}

/// Built-in null? function: (null? val) checks if value is nil.
fn builtin_null(args: &[Value], _env: &mut Environment) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::ArityError {
            expected: "1".to_string(),
            got: args.len(),
        });
    }
    Ok(Value::Bool(matches!(args[0], Value::Nil)))
}

/// Built-in equal? function: (equal? a b) checks if two values are equal.
fn builtin_equal(args: &[Value], _env: &mut Environment) -> Result<Value, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::ArityError {
            expected: "2".to_string(),
            got: args.len(),
        });
    }
    Ok(Value::Bool(values_equal(&args[0], &args[1])))
}

/// Helper function to check if two values are equal.
fn values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Nil, Value::Nil) => true,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::Number(a), Value::Number(b)) => a == b,
        (Value::String(a), Value::String(b)) => a == b,
        (Value::Symbol(a), Value::Symbol(b)) => a == b,
        (Value::Cons(car_a, cdr_a), Value::Cons(car_b, cdr_b)) => {
            values_equal(car_a, car_b) && values_equal(cdr_a, cdr_b)
        }
        _ => false,
    }
}

/// Built-in assoc function: (assoc key alist) searches an association list.
/// Returns the first pair whose car equals the key, or nil if not found.
fn builtin_assoc(args: &[Value], _env: &mut Environment) -> Result<Value, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::ArityError {
            expected: "2".to_string(),
            got: args.len(),
        });
    }

    let key = &args[0];
    let mut current = &args[1];

    // Traverse the association list
    loop {
        match current {
            Value::Nil => return Ok(Value::Nil),
            Value::Cons(car, cdr) => {
                // Each element should be a cons cell (key . value)
                if let Value::Cons(pair_key, _pair_value) = car.as_ref()
                    && values_equal(key, pair_key)
                {
                    return Ok(car.as_ref().clone());
                }
                current = cdr.as_ref();
            }
            _ => {
                return Err(EvalError::TypeError(
                    "assoc requires a proper list".to_string(),
                ));
            }
        }
    }
}

/// Built-in group-by function: (group-by table key-fn)
/// Groups table rows by the result of applying key-fn to each row.
///
/// Table format: list of (row-id . row-data) pairs
/// key-fn: a lambda that takes a row pair and returns a grouping key
/// Returns: list of (group-key . grouped-rows) pairs
fn builtin_group_by(args: &[Value], env: &mut Environment) -> Result<Value, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::ArityError {
            expected: "2".to_string(),
            got: args.len(),
        });
    }

    let table = &args[0];
    let key_fn = &args[1];

    // Convert table to vector of rows
    let mut rows = Vec::new();
    let mut current = table;
    loop {
        match current {
            Value::Nil => break,
            Value::Cons(car, cdr) => {
                rows.push(car.as_ref().clone());
                current = cdr.as_ref();
            }
            _ => {
                return Err(EvalError::TypeError(
                    "group-by requires a proper list as table".to_string(),
                ));
            }
        }
    }

    // Group rows by key
    let mut groups: HashMap<String, Vec<Value>> = HashMap::new();

    for row in rows {
        // Apply key function to get the group key
        let key_value = apply_function(key_fn, std::slice::from_ref(&row), env)?;
        let key_str = value_to_string(&key_value)?;

        groups.entry(key_str).or_default().push(row);
    }

    // Convert groups to association list format
    let mut result = Value::Nil;
    for (key, group_rows) in groups.into_iter() {
        // Convert group_rows to a list
        let group_list = group_rows.into_iter().rev().fold(Value::Nil, |acc, row| {
            Value::Cons(Rc::new(row), Rc::new(acc))
        });

        // Create (key . group-list) pair
        let pair = Value::Cons(Rc::new(Value::String(key)), Rc::new(group_list));

        // Add to result list
        result = Value::Cons(Rc::new(pair), Rc::new(result));
    }

    Ok(result)
}

/// Helper function to apply a function value to arguments.
fn apply_function(func: &Value, args: &[Value], env: &mut Environment) -> Result<Value, EvalError> {
    match func {
        Value::Primitive(prim) => prim(args, env),
        Value::Lambda {
            params,
            body,
            closure,
        } => {
            if params.len() != args.len() {
                return Err(EvalError::ArityError {
                    expected: params.len().to_string(),
                    got: args.len(),
                });
            }

            // Create new environment with closure as parent
            let mut new_env = Environment::with_parent(closure.clone());
            for (param, arg) in params.iter().zip(args.iter()) {
                new_env.define(param.clone(), arg.clone());
            }

            // Evaluate body expressions
            let mut result = Value::Nil;
            for expr in body {
                result = crate::evaluator::eval(expr, &mut new_env)?;
            }
            Ok(result)
        }
        _ => Err(EvalError::TypeError(format!(
            "Cannot apply non-function: {}",
            func
        ))),
    }
}

/// Helper function to convert a value to a string representation for grouping.
fn value_to_string(value: &Value) -> Result<String, EvalError> {
    match value {
        Value::String(s) => Ok(s.clone()),
        Value::Symbol(s) => Ok(s.clone()),
        Value::Number(n) => Ok(n.to_string()),
        _ => Err(EvalError::TypeError(
            "group-by key must be string, symbol, or number".to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cons() {
        let mut env = Environment::new();
        let args = vec![Value::Number(1), Value::Number(2)];
        let result = builtin_cons(&args, &mut env).unwrap();
        assert!(matches!(result, Value::Cons(_, _)));
    }

    #[test]
    fn test_car() {
        let mut env = Environment::new();
        let pair = Value::Cons(Rc::new(Value::Number(1)), Rc::new(Value::Number(2)));
        let result = builtin_car(&[pair], &mut env).unwrap();
        assert_eq!(result, Value::Number(1));
    }

    #[test]
    fn test_cdr() {
        let mut env = Environment::new();
        let pair = Value::Cons(Rc::new(Value::Number(1)), Rc::new(Value::Number(2)));
        let result = builtin_cdr(&[pair], &mut env).unwrap();
        assert_eq!(result, Value::Number(2));
    }

    #[test]
    fn test_null_true() {
        let mut env = Environment::new();
        let result = builtin_null(&[Value::Nil], &mut env).unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_null_false() {
        let mut env = Environment::new();
        let result = builtin_null(&[Value::Number(42)], &mut env).unwrap();
        assert_eq!(result, Value::Bool(false));
    }

    #[test]
    fn test_equal_true() {
        let mut env = Environment::new();
        let args = vec![Value::Number(42), Value::Number(42)];
        let result = builtin_equal(&args, &mut env).unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_equal_false() {
        let mut env = Environment::new();
        let args = vec![Value::Number(42), Value::Number(43)];
        let result = builtin_equal(&args, &mut env).unwrap();
        assert_eq!(result, Value::Bool(false));
    }

    #[test]
    fn test_assoc_found() {
        let mut env = Environment::new();
        // Create alist: ((a . 1) (b . 2))
        let pair1 = Value::Cons(
            Rc::new(Value::Symbol("a".to_string())),
            Rc::new(Value::Number(1)),
        );
        let pair2 = Value::Cons(
            Rc::new(Value::Symbol("b".to_string())),
            Rc::new(Value::Number(2)),
        );
        let alist = Value::Cons(
            Rc::new(pair1.clone()),
            Rc::new(Value::Cons(Rc::new(pair2), Rc::new(Value::Nil))),
        );

        let result = builtin_assoc(&[Value::Symbol("a".to_string()), alist], &mut env).unwrap();
        assert_eq!(result, pair1);
    }

    #[test]
    fn test_assoc_not_found() {
        let mut env = Environment::new();
        // Create alist: ((a . 1) (b . 2))
        let pair1 = Value::Cons(
            Rc::new(Value::Symbol("a".to_string())),
            Rc::new(Value::Number(1)),
        );
        let pair2 = Value::Cons(
            Rc::new(Value::Symbol("b".to_string())),
            Rc::new(Value::Number(2)),
        );
        let alist = Value::Cons(
            Rc::new(pair1),
            Rc::new(Value::Cons(Rc::new(pair2), Rc::new(Value::Nil))),
        );

        let result = builtin_assoc(&[Value::Symbol("c".to_string()), alist], &mut env).unwrap();
        assert_eq!(result, Value::Nil);
    }
}
