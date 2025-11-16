//! Core types for handling monetary values.
//!
//! This module provides the `Money` struct and `Currency` enum for type-safe monetary
//! calculations. It ensures that operations are only performed between money values of
//! the same currency and handles precision correctly for each currency type.

use crate::errors::MoneyError;
use rust_decimal::prelude::*;
use std::fmt;
use std::ops::{Add, Sub};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Supported currencies.
/// Currently supports JPY (Japanese Yen) and USD (United States Dollar).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Currency {
    /// Japanese Yen (0 decimal places).
    JPY,
    /// United States Dollar (2 decimal places).
    USD,
}

/// Implements formatting for `Currency`.
///
/// Displays the currency as its debug representation (e.g., "JPY", "USD").
impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for Currency {
    type Err = MoneyError;

    /// Parses a string into a `Currency`.
    /// Case-insensitive (e.g., "jpy", "JPY", "Jpy" are all valid).
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "JPY" => Ok(Currency::JPY),
            "USD" => Ok(Currency::USD),
            _ => Err(MoneyError::InvalidCurrency(s.to_string())),
        }
    }
}

impl Currency {
    /// Returns the number of decimal places for the currency.
    ///
    /// * `JPY`: 0 (e.g. 100)
    /// * `USD`: 2 (e.g. 10.50)
    pub fn decimal_places(&self) -> u32 {
        match self {
            Currency::JPY => 0,
            Currency::USD => 2,
        }
    }
}

/// Represents a monetary value with a specific currency.
///
/// Ensures that operations like addition and subtraction are only performed
/// between money of the same currency.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Money {
    amount: Decimal,
    currency: Currency,
}

impl Money {
    /// Creates a new Money instance.
    ///
    /// The amount is automatically rounded to the currency's standard precision.
    ///
    /// # Arguments
    /// * `amount` - The amount in major units (e.g., 10.50 for $10.50).
    /// * `currency` - The currency of the money.
    pub fn new(amount: Decimal, currency: Currency) -> Self {
        // Ensure precision matches the currency standard
        let amount: Decimal = amount.round_dp(currency.decimal_places());
        Self { amount, currency }
    }

    /// Helper constructor to create JPY (integers only).
    ///
    /// # Example
    /// ```
    /// use kakei_money::Money;
    /// let yen = Money::jpy(1000);
    /// ```
    pub fn jpy(amount: i64) -> Self {
        Self::new(Decimal::from(amount), Currency::JPY)
    }

    /// Helper constructor to create USD.
    ///
    /// # Example
    /// ```
    /// use kakei_money::Money;
    /// use rust_decimal::prelude::*;
    /// let dollars = Money::usd(dec!(10.50));
    /// ```
    pub fn usd(amount: Decimal) -> Self {
        Self::new(amount, Currency::USD)
    }

    /// Creates Money from "minor units" (e.g., cents).
    /// Useful for database interaction where money might be stored as integer.
    ///
    /// # Conversions
    /// * JPY 100 -> 100 yen
    /// * USD 1050 -> 10.50 dollars
    pub fn from_minor(minor_amount: i64, currency: Currency) -> Self {
        let scale: u32 = currency.decimal_places();
        // Convert minor amount (i64) to Decimal and divide by 10^scale
        let amount: Decimal = Decimal::from(minor_amount) / Decimal::from(10u32.pow(scale));
        Self { amount, currency }
    }

    /// Returns the amount in "minor units" (e.g., cents).
    ///
    /// # Returns
    /// The amount scaled by the currency's decimal places, rounded to the nearest integer.
    /// Returns `Result` to handle potential overflow when converting to `i64`.
    pub fn to_minor(&self) -> Result<i64, MoneyError> {
        let scale: u32 = self.currency.decimal_places();
        let minor: Decimal = self.amount * Decimal::from(10u32.pow(scale));
        // Replace unwrap_or(0) with proper error handling
        minor.round().to_i64().ok_or(MoneyError::Overflow)
    }

    /// Returns the underlying Decimal amount.
    pub fn amount(&self) -> Decimal {
        self.amount
    }

    /// Returns the currency.
    pub fn currency(&self) -> Currency {
        self.currency
    }
}

/// Implements formatting for `Money`.
///
/// Displays the money value with its currency symbol:
/// - JPY: ¥100
/// - USD: $10.50
impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.currency {
            Currency::JPY => write!(f, "¥{}", self.amount),
            Currency::USD => write!(f, "${}", self.amount),
        }
    }
}

// --- Arithmetic Implementations ---

impl Add for Money {
    type Output = Result<Money, MoneyError>;

    /// Adds two `Money` instances.
    ///
    /// # Errors
    /// Returns `MoneyError::CurrencyMismatch` if currencies differ.
    fn add(self, other: Self) -> Self::Output {
        if self.currency != other.currency {
            return Err(MoneyError::CurrencyMismatch(self.currency, other.currency));
        }
        Ok(Money::new(self.amount + other.amount, self.currency))
    }
}

impl Sub for Money {
    type Output = Result<Money, MoneyError>;

    /// Subtracts two `Money` instances.
    ///
    /// # Errors
    /// Returns `MoneyError::CurrencyMismatch` if currencies differ.
    fn sub(self, other: Self) -> Self::Output {
        if self.currency != other.currency {
            return Err(MoneyError::CurrencyMismatch(self.currency, other.currency));
        }
        Ok(Money::new(self.amount - other.amount, self.currency))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod money {
        use super::*;

        #[test]
        fn test_jpy_creation() {
            let m: Money = Money::jpy(100);
            assert_eq!(m.to_string(), "¥100");
            assert_eq!(m.to_minor().unwrap(), 100);
        }

        #[test]
        fn test_usd_creation() {
            let m: Money = Money::usd(dec!(10.50));
            assert_eq!(m.to_string(), "$10.50");
            assert_eq!(m.to_minor().unwrap(), 1050);
        }

        #[test]
        fn test_addition_success() {
            let m1: Money = Money::jpy(100);
            let m2: Money = Money::jpy(200);
            let sum: Money = (m1 + m2).unwrap();
            assert_eq!(sum.amount(), dec!(300));
            assert_eq!(sum.currency(), Currency::JPY);
        }

        #[test]
        fn test_addition_mismatch() {
            let m1: Money = Money::jpy(100);
            let m2: Money = Money::usd(dec!(1.00));
            assert_eq!(
                m1 + m2,
                Err(MoneyError::CurrencyMismatch(Currency::JPY, Currency::USD))
            );
        }

        #[test]
        fn test_minor_conversion() {
            // USD 10.50 -> 1050 cents
            let m: Money = Money::from_minor(1050, Currency::USD);
            assert_eq!(m.amount(), dec!(10.50));

            // JPY 1050 -> 1050 yen
            let m_jpy: Money = Money::from_minor(1050, Currency::JPY);
            assert_eq!(m_jpy.amount(), dec!(1050));
        }

        #[test]
        fn test_minor_conversion_overflow() {
            // Decimal::MAX causes overflow when converting to i64 minor units
            let m: Money = Money::new(Decimal::MAX, Currency::JPY);
            assert_eq!(m.to_minor(), Err(MoneyError::Overflow));
        }
    }
}
