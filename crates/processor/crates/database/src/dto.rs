use crate::error::DbError;
use crate::models::{Account, Transaction, TransactionDetail};
use crate::types::{AccountId, CategoryId, TransactionId};
use chrono::NaiveDate;
use kakei_money::{Currency, Money};
use sqlx::FromRow;

// --- Internal DTOs ---
// DTOs (Data Transfer Objects) used to map between database rows (flat structure)
// and domain models (rich types like Money).

#[derive(FromRow)]
pub(crate) struct AccountDto {
    pub account_id: AccountId,
    pub name: String,
    pub initial_balance: i64, // Stored as minor units (e.g., cents)
    pub currency: String,     // Stored as currency code (e.g., "USD")
}

impl TryFrom<AccountDto> for Account {
    type Error = DbError;

    fn try_from(dto: AccountDto) -> Result<Self, Self::Error> {
        let currency: Currency = dto.currency.parse()?;
        Ok(Account {
            account_id: dto.account_id,
            name: dto.name,
            initial_balance: Money::from_minor(dto.initial_balance, currency),
        })
    }
}

#[derive(FromRow)]
pub(crate) struct TransactionDto {
    pub transaction_id: TransactionId,
    pub date: NaiveDate,
    pub amount: i64,      // Stored as minor units
    pub currency: String, // Stored as currency code
    pub memo: Option<String>,
    pub category_id: CategoryId,
    pub account_id: AccountId,
}

impl TryFrom<TransactionDto> for Transaction {
    type Error = DbError;

    fn try_from(dto: TransactionDto) -> Result<Self, Self::Error> {
        let currency: Currency = dto.currency.parse()?;
        Ok(Transaction {
            transaction_id: dto.transaction_id,
            date: dto.date,
            amount: Money::from_minor(dto.amount, currency),
            memo: dto.memo,
            category_id: dto.category_id,
            account_id: dto.account_id,
        })
    }
}

/// A Data Transfer Object representing a transaction joined with category and account names.
/// This structure maps directly to the SQL query result row.
#[derive(FromRow)]
pub(crate) struct TransactionDetailDto {
    pub transaction_id: TransactionId,
    pub date: NaiveDate,
    pub amount: i64,
    pub currency: String,
    pub memo: Option<String>,
    pub category_name: String,
    pub account_name: String,
}

impl TryFrom<TransactionDetailDto> for TransactionDetail {
    type Error = DbError;

    fn try_from(dto: TransactionDetailDto) -> Result<Self, Self::Error> {
        let currency: Currency = dto.currency.parse()?;
        Ok(TransactionDetail {
            transaction_id: dto.transaction_id,
            date: dto.date,
            amount: Money::from_minor(dto.amount, currency),
            memo: dto.memo,
            category_name: dto.category_name,
            account_name: dto.account_name,
        })
    }
}
