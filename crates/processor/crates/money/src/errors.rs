use thiserror::Error;
use crate::types::Currency;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Errors related to money operations.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum MoneyError {
    /// Error returned when attempting to perform operations on moneys with different currencies.
    #[error("Currency mismatch: cannot perform operation between {0} and {1}")]
    CurrencyMismatch(Currency, Currency),

    /// Error returned when parsing an invalid currency string.
    #[error("Invalid currency code: {0}")]
    InvalidCurrency(String),

    /// Error returned when conversion to minor units (i64) overflows.
    #[error("Amount overflow: value is too large to fit in i64")]
    Overflow,
}
