//! kakei_money crate
//!
//! Provides a strongly-typed `Money` struct that handles currency and precision safely.
//! This crate wraps `rust_decimal` to provide financial calculations with currency safety checks.

mod errors;
mod types;

pub use errors::MoneyError;
pub use types::{Currency, Money};
