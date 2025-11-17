//! Evaluator module for the Lisp dialect.
//!
//! This module contains the evaluator implementation that interprets
//! AST expressions into runtime values.

mod builtins;
mod eval;
mod value;

// Re-export public API
pub use builtins::create_global_env;
pub use eval::eval;
pub use value::{Environment, EvalError, Value};
