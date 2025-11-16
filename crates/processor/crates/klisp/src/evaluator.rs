//! Evaluator for the Lisp dialect.
//!
//! This module implements the evaluation of S-expressions into runtime values.

use crate::ast::{Atom, Sexpr};
use crate::value::{Environment, EvalError, Value};
use std::rc::Rc;

/// Evaluate a single S-expression in the given environment.
pub fn eval(expr: &Sexpr, env: &mut Environment) -> Result<Value, EvalError> {
    match expr {
        Sexpr::Atom(atom) => eval_atom(atom, env),
        Sexpr::List(list) => {
            if list.is_empty() {
                return Ok(Value::Nil);
            }
            eval_list(list, env)
        }
        Sexpr::DottedList(list, tail) => eval_dotted_list(list, tail, env),
    }
}

/// Evaluate an atom.
fn eval_atom(atom: &Atom, env: &mut Environment) -> Result<Value, EvalError> {
    match atom {
        Atom::Nil => Ok(Value::Nil),
        Atom::Number(n) => Ok(Value::Number(*n)),
        Atom::String(s) => Ok(Value::String(s.clone())),
        Atom::Symbol(s) => {
            // Look up the symbol in the environment
            env.get(s)
                .ok_or_else(|| EvalError::UndefinedVariable(s.clone()))
        }
    }
}

/// Evaluate a list (function application or special form).
fn eval_list(list: &[Sexpr], env: &mut Environment) -> Result<Value, EvalError> {
    if list.is_empty() {
        return Ok(Value::Nil);
    }

    // Check if the first element is a special form
    if let Sexpr::Atom(Atom::Symbol(name)) = &list[0] {
        match name.as_str() {
            "quote" => return eval_quote(&list[1..]),
            "define" => return eval_define(&list[1..], env),
            "lambda" => return eval_lambda(&list[1..], env),
            "if" => return eval_if(&list[1..], env),
            _ => {}
        }
    }

    // Otherwise, evaluate as a function application
    let func = eval(&list[0], env)?;
    let args = list[1..]
        .iter()
        .map(|arg| eval(arg, env))
        .collect::<Result<Vec<_>, _>>()?;

    apply(func, &args, env)
}

/// Evaluate a dotted list (convert to cons cell).
fn eval_dotted_list(
    list: &[Sexpr],
    tail: &Sexpr,
    env: &mut Environment,
) -> Result<Value, EvalError> {
    let tail_val = eval(tail, env)?;
    list.iter().rev().try_fold(tail_val, |acc, expr| {
        let val = eval(expr, env)?;
        Ok(Value::Cons(Rc::new(val), Rc::new(acc)))
    })
}

/// Evaluate a quote special form.
fn eval_quote(args: &[Sexpr]) -> Result<Value, EvalError> {
    if args.len() != 1 {
        return Err(EvalError::ArityError {
            expected: "1".to_string(),
            got: args.len(),
        });
    }
    sexpr_to_value(&args[0])
}

/// Convert an S-expression to a value without evaluation.
fn sexpr_to_value(expr: &Sexpr) -> Result<Value, EvalError> {
    match expr {
        Sexpr::Atom(Atom::Nil) => Ok(Value::Nil),
        Sexpr::Atom(Atom::Number(n)) => Ok(Value::Number(*n)),
        Sexpr::Atom(Atom::String(s)) => Ok(Value::String(s.clone())),
        Sexpr::Atom(Atom::Symbol(s)) => Ok(Value::Symbol(s.clone())),
        Sexpr::List(list) => {
            let values = list
                .iter()
                .map(sexpr_to_value)
                .collect::<Result<Vec<_>, _>>()?;
            list_to_cons(&values)
        }
        Sexpr::DottedList(list, tail) => {
            let tail_val = sexpr_to_value(tail)?;
            list.iter().rev().try_fold(tail_val, |acc, expr| {
                let val = sexpr_to_value(expr)?;
                Ok(Value::Cons(Rc::new(val), Rc::new(acc)))
            })
        }
    }
}

/// Convert a list of values to a cons list.
fn list_to_cons(values: &[Value]) -> Result<Value, EvalError> {
    Ok(values.iter().rev().fold(Value::Nil, |acc, val| {
        Value::Cons(Rc::new(val.clone()), Rc::new(acc))
    }))
}

/// Evaluate a define special form.
fn eval_define(args: &[Sexpr], env: &mut Environment) -> Result<Value, EvalError> {
    if args.len() != 2 {
        return Err(EvalError::ArityError {
            expected: "2".to_string(),
            got: args.len(),
        });
    }

    let name = match &args[0] {
        Sexpr::Atom(Atom::Symbol(s)) => s.clone(),
        _ => {
            return Err(EvalError::TypeError(
                "define requires a symbol as first argument".to_string(),
            ));
        }
    };

    let value = eval(&args[1], env)?;
    env.define(name, value.clone());
    Ok(value)
}

/// Evaluate a lambda special form.
fn eval_lambda(args: &[Sexpr], env: &mut Environment) -> Result<Value, EvalError> {
    if args.len() < 2 {
        return Err(EvalError::ArityError {
            expected: "at least 2".to_string(),
            got: args.len(),
        });
    }

    let params = match &args[0] {
        Sexpr::List(list) => list
            .iter()
            .map(|expr| match expr {
                Sexpr::Atom(Atom::Symbol(s)) => Ok(s.clone()),
                _ => Err(EvalError::TypeError(
                    "lambda parameters must be symbols".to_string(),
                )),
            })
            .collect::<Result<Vec<_>, _>>()?,
        _ => {
            return Err(EvalError::TypeError(
                "lambda requires a parameter list".to_string(),
            ));
        }
    };

    let body = args[1..].to_vec();
    let closure = Rc::new(env.clone());

    Ok(Value::Lambda {
        params,
        body,
        closure,
    })
}

/// Evaluate an if special form.
fn eval_if(args: &[Sexpr], env: &mut Environment) -> Result<Value, EvalError> {
    if args.len() != 3 {
        return Err(EvalError::ArityError {
            expected: "3".to_string(),
            got: args.len(),
        });
    }

    let test = eval(&args[0], env)?;
    if is_truthy(&test) {
        eval(&args[1], env)
    } else {
        eval(&args[2], env)
    }
}

/// Check if a value is truthy (anything except #f and ()).
fn is_truthy(value: &Value) -> bool {
    !matches!(value, Value::Bool(false) | Value::Nil)
}

/// Apply a function to arguments.
fn apply(func: Value, args: &[Value], env: &mut Environment) -> Result<Value, EvalError> {
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
            let mut new_env = Environment::with_parent(closure);
            for (param, arg) in params.iter().zip(args.iter()) {
                new_env.define(param.clone(), arg.clone());
            }

            // Evaluate body expressions
            let mut result = Value::Nil;
            for expr in &body {
                result = eval(expr, &mut new_env)?;
            }
            Ok(result)
        }
        _ => Err(EvalError::TypeError(format!(
            "Cannot apply non-function: {}",
            func
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builtins::create_global_env;

    #[test]
    fn test_eval_number() {
        let mut env = Environment::new();
        let expr = Sexpr::Atom(Atom::Number(42));
        let result = eval(&expr, &mut env).unwrap();
        assert_eq!(result, Value::Number(42));
    }

    #[test]
    fn test_eval_string() {
        let mut env = Environment::new();
        let expr = Sexpr::Atom(Atom::String("hello".to_string()));
        let result = eval(&expr, &mut env).unwrap();
        assert_eq!(result, Value::String("hello".to_string()));
    }

    #[test]
    fn test_eval_quote() {
        let mut env = Environment::new();
        let expr = Sexpr::List(vec![
            Sexpr::Atom(Atom::Symbol("quote".to_string())),
            Sexpr::Atom(Atom::Symbol("x".to_string())),
        ]);
        let result = eval(&expr, &mut env).unwrap();
        assert_eq!(result, Value::Symbol("x".to_string()));
    }

    #[test]
    fn test_eval_define() {
        let mut env = Environment::new();
        let expr = Sexpr::List(vec![
            Sexpr::Atom(Atom::Symbol("define".to_string())),
            Sexpr::Atom(Atom::Symbol("x".to_string())),
            Sexpr::Atom(Atom::Number(42)),
        ]);
        eval(&expr, &mut env).unwrap();
        assert_eq!(env.get("x"), Some(Value::Number(42)));
    }

    #[test]
    fn test_eval_lambda() {
        let mut env = Environment::new();
        let expr = Sexpr::List(vec![
            Sexpr::Atom(Atom::Symbol("lambda".to_string())),
            Sexpr::List(vec![Sexpr::Atom(Atom::Symbol("x".to_string()))]),
            Sexpr::Atom(Atom::Symbol("x".to_string())),
        ]);
        let result = eval(&expr, &mut env).unwrap();
        assert!(matches!(result, Value::Lambda { .. }));
    }

    #[test]
    fn test_eval_if_true() {
        let mut env = create_global_env();
        let expr = Sexpr::List(vec![
            Sexpr::Atom(Atom::Symbol("if".to_string())),
            Sexpr::Atom(Atom::Number(1)), // truthy
            Sexpr::Atom(Atom::Number(10)),
            Sexpr::Atom(Atom::Number(20)),
        ]);
        let result = eval(&expr, &mut env).unwrap();
        assert_eq!(result, Value::Number(10));
    }

    #[test]
    fn test_eval_if_false() {
        let mut env = create_global_env();
        let expr = Sexpr::List(vec![
            Sexpr::Atom(Atom::Symbol("if".to_string())),
            Sexpr::List(vec![]), // Nil is falsy
            Sexpr::Atom(Atom::Number(10)),
            Sexpr::Atom(Atom::Number(20)),
        ]);
        let result = eval(&expr, &mut env).unwrap();
        assert_eq!(result, Value::Number(20));
    }
}
