use chrono::NaiveDate;
use kakei_database::{
    CategoryType, DbError, KakeiRepository, SqliteKakeiRepository, TransactionDetail, TransactionId,
};
use kakei_money::{Currency, Money, MoneyError};
use rust_decimal::Decimal;
use std::str::FromStr;
use thiserror::Error;
use tracing::{debug, info, instrument, warn};

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
    #[instrument]
    pub async fn new(db_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        info!("Initializing Processor with database at: {}", db_path);
        let repo = SqliteKakeiRepository::new(db_path).await?;

        // Database Migration
        debug!("Running database migrations from sqpx::migrate!()...");
        sqlx::migrate!("crates/database/db/migrations")
            .run(repo.get_pool())
            .await?;

        info!("Processor initialized successfully");
        Ok(Self { repo })
    }

    /// Initializes the database with master data provided as arguments.
    ///
    /// This replaces the old hardcoded initialization logic.
    #[instrument(skip(self))]
    pub async fn init_master_data(
        &self,
        categories: &[String],
        accounts: &[String],
    ) -> Result<(), ProcessorError> {
        info!(
            "Initializing master data with {} categories and {} accounts",
            categories.len(),
            accounts.len()
        );

        // 1. Create Categories
        for cat_name in categories {
            debug!("Processing category: {}", cat_name);
            // Determine category type (simple logic for now)
            let type_ = if cat_name.eq_ignore_ascii_case("Salary")
                || cat_name.eq_ignore_ascii_case("Bonus")
                || cat_name.eq_ignore_ascii_case("Income")
            {
                CategoryType::Income
            } else {
                CategoryType::Expense
            };

            self.repo.create_category(cat_name, type_).await?;
        }

        // 2. Create Accounts
        for acc_name in accounts {
            debug!("Processing account: {}", acc_name);
            // Default initial balance 0 JPY
            self.repo.create_account(acc_name, Money::jpy(0)).await?;
        }

        info!("Master data initialization complete");
        Ok(())
    }

    /// Adds a transaction based on user input strings.
    ///
    /// Performs parsing, validation, and database insertion.
    #[instrument(skip(self))]
    pub async fn add_transaction(
        &self,
        date_str: &str,
        amount_str: &str,
        currency_str: &str,
        category_name: &str,
        account_name: &str,
        memo: Option<String>,
    ) -> Result<TransactionId, ProcessorError> {
        info!("Processing add transaction request");
        debug!(
            "Date: {}, Amount: {}, Currency: {}, Category: {}, Account: {}",
            date_str, amount_str, currency_str, category_name, account_name
        );

        // 1. Parse inputs
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
        debug!("Parsed date: {}", date);

        let currency = Currency::from_str(currency_str)?;
        debug!("Parsed currency: {:?}", currency);

        let amount_decimal = Decimal::from_str(amount_str)?;
        let amount = Money::new(amount_decimal, currency);
        debug!("Parsed amount: {:?}", amount);

        // 2. Resolve Category ID
        debug!("Resolving category: {}", category_name);
        let category = self
            .repo
            .find_category_by_name(category_name)
            .await?
            .ok_or_else(|| {
                warn!("Category not found: {}", category_name);
                ProcessorError::CategoryNotFound(category_name.to_string())
            })?;

        // 3. Resolve Account ID
        debug!("Resolving account: {}", account_name);
        let account = self
            .repo
            .find_account_by_name(account_name)
            .await?
            .ok_or_else(|| {
                warn!("Account not found: {}", account_name);
                ProcessorError::AccountNotFound(account_name.to_string())
            })?;

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

        info!("Transaction processed successfully: {:?}", tx_id);
        Ok(tx_id)
    }

    /// Retrieves a list of recent transactions for display purposes.
    ///
    /// Currently defaults to fetching the latest 20 transactions.
    ///
    /// # Returns
    ///
    /// Returns a list of `TransactionDetail` containing readable names instead of IDs.
    #[instrument(skip(self))]
    pub async fn get_recent_transactions(&self) -> Result<Vec<TransactionDetail>, ProcessorError> {
        info!("Fetching recent transactions");
        // Default limit to 20 for now
        let transactions = self.repo.get_recent_transactions(20).await?;
        debug!("Retrieved {} transactions", transactions.len());
        Ok(transactions)
    }
}
