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
    pub async fn new(db_path: &str) -> Result<Self, ProcessorError> {
        info!("Initializing Processor with database at: {}", db_path);
        let repo = SqliteKakeiRepository::new(db_path).await?;
        repo.migrate().await?;
        info!("Processor initialized successfully");
        Ok(Self { repo })
    }

    /// Initializes the database with master data provided as arguments.
    ///
    /// This method accepts separate lists for expense and income categories,
    /// allowing users to freely define their own category structure without hardcoding.
    #[instrument(skip(self))]
    pub async fn init_master_data(
        &self,
        expense_categories: &[String],
        income_categories: &[String],
        accounts: &[String],
    ) -> Result<(), ProcessorError> {
        info!(
            "Initializing master data with {} expense categories, {} income categories, and {} accounts",
            expense_categories.len(),
            income_categories.len(),
            accounts.len()
        );

        // 1. Create Expense Categories
        for cat_name in expense_categories {
            debug!("Processing expense category: {}", cat_name);
            self.repo
                .create_category(cat_name, CategoryType::Expense)
                .await?;
        }

        // 2. Create Income Categories
        for cat_name in income_categories {
            debug!("Processing income category: {}", cat_name);
            self.repo
                .create_category(cat_name, CategoryType::Income)
                .await?;
        }

        // 3. Create Accounts
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that init_master_data correctly creates expense and income categories
    #[tokio::test]
    async fn test_init_master_data_with_separate_categories() {
        let processor = Processor::new(":memory:")
            .await
            .expect("Failed to create processor");

        let expense_categories = vec![
            "Food".to_string(),
            "Transport".to_string(),
            "Utilities".to_string(),
        ];
        let income_categories = vec!["Salary".to_string(), "Freelance".to_string()];
        let accounts = vec!["Cash".to_string(), "Bank".to_string()];

        let result = processor
            .init_master_data(&expense_categories, &income_categories, &accounts)
            .await;

        assert!(result.is_ok(), "init_master_data should succeed");

        // Verify categories were created
        let categories = processor
            .repo
            .get_all_categories()
            .await
            .expect("Failed to get categories");

        assert_eq!(categories.len(), 5, "Should have 5 categories");

        // Check expense categories
        let expense_cats: Vec<_> = categories
            .iter()
            .filter(|c| c.type_ == CategoryType::Expense)
            .collect();
        assert_eq!(expense_cats.len(), 3, "Should have 3 expense categories");

        // Check income categories
        let income_cats: Vec<_> = categories
            .iter()
            .filter(|c| c.type_ == CategoryType::Income)
            .collect();
        assert_eq!(income_cats.len(), 2, "Should have 2 income categories");
    }

    /// Test that categories are correctly categorized as expense or income
    #[tokio::test]
    async fn test_category_types_are_correct() {
        let processor = Processor::new(":memory:")
            .await
            .expect("Failed to create processor");

        let expense_categories = vec!["Groceries".to_string()];
        let income_categories = vec!["Investment".to_string()];
        let accounts = vec!["Wallet".to_string()];

        processor
            .init_master_data(&expense_categories, &income_categories, &accounts)
            .await
            .expect("init_master_data should succeed");

        // Verify the "Groceries" category is Expense
        let groceries = processor
            .repo
            .find_category_by_name("Groceries")
            .await
            .expect("Failed to find category")
            .expect("Category should exist");
        assert_eq!(groceries.type_, CategoryType::Expense);

        // Verify the "Investment" category is Income
        let investment = processor
            .repo
            .find_category_by_name("Investment")
            .await
            .expect("Failed to find category")
            .expect("Category should exist");
        assert_eq!(investment.type_, CategoryType::Income);
    }
}
