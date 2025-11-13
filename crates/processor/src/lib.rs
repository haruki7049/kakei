use chrono::NaiveDate;
use kakei_database::{
    CategoryType, DbError, KakeiRepository, SqliteKakeiRepository, TransactionDetail, TransactionId,
};
use kakei_money::{Currency, Money, MoneyError};
use rust_decimal::Decimal;
use std::str::FromStr;
use thiserror::Error;

/// Errors specific to the Processor layer.
#[derive(Debug, Error)]
pub enum ProcessorError {
    #[error("Database error: {0}")]
    Database(#[from] DbError),
    #[error("Money error: {0}")]
    Money(#[from] MoneyError),

    #[error("Invalid date format: {0}")]
    InvalidDate(#[from] chrono::ParseError),

    #[error("Invalid amount format: {0}")]
    InvalidAmount(#[from] rust_decimal::Error),

    #[error("Category not found: {0}")]
    CategoryNotFound(String),

    #[error("Account not found: {0}")]
    AccountNotFound(String),
}

pub struct Processor {
    repo: SqliteKakeiRepository,
}

impl Processor {
    /// Creates a new Processor and connects to the database.
    pub async fn new(db_path: &str) -> Result<Self, ProcessorError> {
        let repo = SqliteKakeiRepository::new(db_path).await?;
        repo.migrate().await?;
        Ok(Self { repo })
    }

    /// Initializes the database with default categories and accounts.
    /// Used by the `init` command.
    pub async fn init_default_data(&self) -> Result<(), ProcessorError> {
        // Default Categories
        self.repo
            .create_category("Food", CategoryType::Expense)
            .await?;
        self.repo
            .create_category("Transport", CategoryType::Expense)
            .await?;
        self.repo
            .create_category("Daily Goods", CategoryType::Expense)
            .await?;
        self.repo
            .create_category("Hobby", CategoryType::Expense)
            .await?;
        self.repo
            .create_category("Salary", CategoryType::Income)
            .await?;

        // Default Accounts (Initial balance 0 JPY)
        self.repo.create_account("Cash", Money::jpy(0)).await?;
        self.repo.create_account("Bank", Money::jpy(0)).await?;

        Ok(())
    }

    /// Adds a transaction based on user input strings.
    ///
    /// Performs parsing, validation, and database insertion.
    pub async fn add_transaction(
        &self,
        date_str: &str,
        amount_str: &str,
        currency_str: &str,
        category_name: &str,
        account_name: &str,
        memo: Option<String>,
    ) -> Result<TransactionId, ProcessorError> {
        // 1. Parse inputs
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;

        let currency = Currency::from_str(currency_str)?;

        let amount_decimal = Decimal::from_str(amount_str)?;
        let amount = Money::new(amount_decimal, currency);

        // 2. Resolve Category ID
        let category = self
            .repo
            .find_category_by_name(category_name)
            .await?
            .ok_or_else(|| ProcessorError::CategoryNotFound(category_name.to_string()))?;

        // 3. Resolve Account ID
        let account = self
            .repo
            .find_account_by_name(account_name)
            .await?
            .ok_or_else(|| ProcessorError::AccountNotFound(account_name.to_string()))?;

        // 4. Call Repository
        let tx_id = self
            .repo
            .add_transaction(
                date,
                amount,
                memo.as_deref(),
                category.category_id,
                account.account_id,
            )
            .await?;

        Ok(tx_id)
    }

    /// Retrieves a list of recent transactions for display purposes.
    ///
    /// Currently defaults to fetching the latest 20 transactions.
    ///
    /// # Returns
    ///
    /// Returns a list of `TransactionDetail` containing readable names instead of IDs.
    pub async fn get_recent_transactions(&self) -> Result<Vec<TransactionDetail>, ProcessorError> {
        // Default limit to 20 for now
        let transactions = self.repo.get_recent_transactions(20).await?;
        Ok(transactions)
    }
}
