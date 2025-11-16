//! Value types for the Lisp evaluator.
//!
//! This module defines the runtime value types that result from evaluating
//! S-expressions. These are different from the AST types in that they include
//! functions and other runtime-specific values.

use crate::parser::Sexpr;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

/// Runtime value types that result from evaluation.
#[derive(Debug, Clone)]
pub enum Value {
    /// The empty list (nil).
    Nil,
    /// A boolean value.
    Bool(bool),
    /// An integer number.
    Number(i64),
    /// A string value.
    String(String),
    /// A symbol (unevaluated).
    Symbol(String),
    /// A cons cell (pair).
    Cons(Rc<Value>, Rc<Value>),
    /// A built-in primitive function.
    Primitive(PrimitiveFn),
    /// A user-defined lambda function.
    Lambda {
        params: Vec<String>,
        body: Vec<Sexpr>,
        closure: Rc<Environment>,
    },
}

// Manual PartialEq implementation that skips function pointer comparison
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Symbol(a), Value::Symbol(b)) => a == b,
            (Value::Cons(car_a, cdr_a), Value::Cons(car_b, cdr_b)) => {
                car_a == car_b && cdr_a == cdr_b
            }
            // Functions are compared by identity (never equal unless same instance)
            (Value::Primitive(_), Value::Primitive(_)) => false,
            (Value::Lambda { .. }, Value::Lambda { .. }) => false,
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "()"),
            Value::Bool(b) => write!(f, "{}", if *b { "#t" } else { "#f" }),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Symbol(s) => write!(f, "{}", s),
            Value::Cons(car, cdr) => {
                write!(f, "(")?;
                write!(f, "{}", car)?;
                let mut current = cdr.as_ref();
                loop {
                    match current {
                        Value::Nil => break,
                        Value::Cons(car, cdr) => {
                            write!(f, " {}", car)?;
                            current = cdr.as_ref();
                        }
                        _ => {
                            write!(f, " . {}", current)?;
                            break;
                        }
                    }
                }
                write!(f, ")")
            }
            Value::Primitive(_) => write!(f, "#<primitive>"),
            Value::Lambda { .. } => write!(f, "#<lambda>"),
        }
    }
}

/// Type for primitive (built-in) functions.
pub type PrimitiveFn = fn(&[Value], &mut Environment) -> Result<Value, EvalError>;

/// Evaluation error type.
#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    /// Undefined variable.
    UndefinedVariable(String),
    /// Type error.
    TypeError(String),
    /// Arity error (wrong number of arguments).
    ArityError { expected: String, got: usize },
    /// General evaluation error.
    RuntimeError(String),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::UndefinedVariable(name) => write!(f, "Undefined variable: {}", name),
            EvalError::TypeError(msg) => write!(f, "Type error: {}", msg),
            EvalError::ArityError { expected, got } => {
                write!(f, "Arity error: expected {}, got {}", expected, got)
            }
            EvalError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
        }
    }
}

impl std::error::Error for EvalError {}

/// Environment for variable bindings.
#[derive(Debug, Clone)]
pub struct Environment {
    bindings: HashMap<String, Value>,
    parent: Option<Rc<Environment>>,
}

impl Environment {
    /// Create a new empty environment.
    pub fn new() -> Self {
        Environment {
            bindings: HashMap::new(),
            parent: None,
        }
    }

    /// Create a new environment with a parent.
    pub fn with_parent(parent: Rc<Environment>) -> Self {
        Environment {
            bindings: HashMap::new(),
            parent: Some(parent),
        }
    }

    /// Define a variable in this environment.
    pub fn define(&mut self, name: String, value: Value) {
        self.bindings.insert(name, value);
    }

    /// Get a variable from this environment or its parents.
    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.bindings.get(name) {
            Some(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.get(name)
        } else {
            None
        }
    }

    /// Set a variable in this environment or its parents.
    pub fn set(&mut self, name: &str, value: Value) -> Result<(), EvalError> {
        if self.bindings.contains_key(name) {
            self.bindings.insert(name.to_string(), value);
            Ok(())
        } else if let Some(_parent) = &self.parent {
            // For now, we don't support mutation of parent environments
            // This is a simplified implementation
            Err(EvalError::UndefinedVariable(name.to_string()))
        } else {
            Err(EvalError::UndefinedVariable(name.to_string()))
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
