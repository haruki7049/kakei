use crate::types::{AccountId, CategoryId, CategoryType, TransactionId};
use chrono::NaiveDate;
use kakei_money::Money;
use sqlx::FromRow;

// --- Table Structs (Domain Models) ---

/// Represents a row in the `Categories` table.
#[derive(Debug, FromRow)]
pub struct Category {
    /// The unique ID of the category.
    pub category_id: CategoryId,
    /// The display name of the category (unique).
    pub name: String,
    /// The type of the category (Expense or Income).
    /// Mapped from the 'type' column in the database.
    #[sqlx(rename = "type")]
    pub type_: CategoryType,
}

/// Represents an Account in the domain.
///
/// Note: The balance is represented by the `Money` type, which includes currency information.
#[derive(Debug)]
pub struct Account {
    /// The unique ID of the account.
    pub account_id: AccountId,
    /// The display name of the account (unique).
    pub name: String,
    /// The initial balance of the account.
    pub initial_balance: Money,
}

/// Represents a Transaction in the domain.
///
/// Note: The amount is represented by the `Money` type, which includes currency information.
#[derive(Debug)]
pub struct Transaction {
    /// The unique ID of the transaction.
    pub transaction_id: TransactionId,
    /// The date of the transaction.
    pub date: NaiveDate,
    /// The amount of the transaction.
    pub amount: Money,
    /// An optional memo or note for the transaction.
    pub memo: Option<String>,
    /// The ID of the associated category.
    pub category_id: CategoryId,
    /// The ID of the associated account (source/destination).
    pub account_id: AccountId,
}
