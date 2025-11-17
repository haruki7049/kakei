//! Processor layer for the kakei application.
//!
//! This module provides high-level business logic for managing financial transactions,
//! including adding transactions, initializing master data, and retrieving transaction history.

pub mod table_transform;

/// Default limit for the number of recent transactions to fetch
const DEFAULT_TRANSACTION_LIMIT: i64 = 20;

use chrono::NaiveDate;
use kakei_database::{
    CategoryType, DbError, KakeiRepository, SqliteKakeiRepository, TransactionDetail, TransactionId,
};
use kakei_money::{Currency, Money, MoneyError};
use rust_decimal::Decimal;
use std::str::FromStr;
use thiserror::Error;
use tracing::{debug, info, instrument, warn};

pub use table_transform::{
    DisplayRow, GroupedTable, TransformError, format_value, is_grouped_result,
    transactions_to_table, transform_table, value_to_display_rows, value_to_grouped_tables,
};

/// Errors specific to the Processor layer.
#[derive(Debug, Error)]
pub enum ProcessorError {
    /// Database operation failed.
    #[error("Database error: {0}")]
    Database(#[from] DbError),
    /// Money-related operation failed (e.g., currency mismatch).
    #[error("Money error: {0}")]
    Money(#[from] MoneyError),

    /// Date string parsing failed.
    #[error("Invalid date format: {0}")]
    InvalidDate(#[from] chrono::ParseError),

    /// Amount string parsing failed.
    #[error("Invalid amount format: {0}")]
    InvalidAmount(#[from] rust_decimal::Error),

    /// The specified category was not found in the database.
    #[error("Category not found: {0}")]
    CategoryNotFound(String),

    /// The specified account was not found in the database.
    #[error("Account not found: {0}")]
    AccountNotFound(String),

    /// Table transformation error.
    #[error("Table transformation error: {0}")]
    Transform(#[from] TransformError),
}

/// The main processor for handling kakeibo operations.
///
/// This struct provides methods for managing transactions, categories, and accounts
/// in the kakeibo (household financial ledger) application.
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
        let transactions = self.repo.get_recent_transactions(DEFAULT_TRANSACTION_LIMIT).await?;
        debug!("Retrieved {} transactions", transactions.len());
        Ok(transactions)
    }

    /// Transform transactions using a Lisp program.
    ///
    /// This method retrieves recent transactions, converts them to a Lisp table,
    /// applies the transformation program, and returns the result as a Lisp Value.
    ///
    /// # Arguments
    ///
    /// * `lisp_program` - The Lisp program to transform the table
    ///
    /// # Returns
    ///
    /// Returns the transformed table as a Lisp Value.
    #[instrument(skip(self, lisp_program))]
    pub async fn transform_transactions(
        &self,
        lisp_program: &str,
    ) -> Result<kakei_lisp::Value, ProcessorError> {
        info!("Transforming transactions with Lisp program");

        // Get recent transactions
        let transactions = self.get_recent_transactions().await?;
        debug!(
            "Converting {} transactions to Lisp table",
            transactions.len()
        );

        // Convert to Lisp table
        let table = transactions_to_table(&transactions);

        // Apply transformation
        debug!("Applying Lisp transformation");
        let result = transform_table(table, lisp_program)?;

        info!("Transformation complete");
        Ok(result)
    }
}
