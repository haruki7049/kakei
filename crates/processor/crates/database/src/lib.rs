//! kakei_database crate
//!
//! Handles all database logic using sqlx.
//! This crate provides a strongly-typed interface to the SQLite database via the Repository pattern.

use chrono::NaiveDate;
use kakei_money::{Currency, Money, MoneyError};
use sqlx::sqlite::{Sqlite, SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{FromRow, Pool, Type};
use thiserror::Error;

// --- 1. Error Type ---

/// Represents errors that can occur within the database layer.
#[derive(Error, Debug)]
pub enum DbError {
    /// An error occurred within the sqlx library.
    #[error("Database query failed: {0}")]
    Sqlx(#[from] sqlx::Error),

    /// An error occurred during database migration.
    #[error("Database migration failed: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    /// The requested item was not found in the database.
    #[error("Item not found: {0}")]
    NotFound(String),

    /// An error occurred related to Money operations.
    #[error("Money error: {0}")]
    Money(#[from] MoneyError),
}

// --- 2. Strong Types (Newtype Pattern & Enums) ---

/// A strongly-typed identifier for a Category.
/// Wraps an `i64` value and maps transparently to the database INTEGER type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(transparent)]
pub struct CategoryId(i64);

/// A strongly-typed identifier for an Account.
/// Wraps an `i64` value and maps transparently to the database INTEGER type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(transparent)]
pub struct AccountId(i64);

/// A strongly-typed identifier for a Transaction.
/// Wraps an `i64` value and maps transparently to the database INTEGER type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(transparent)]
pub struct TransactionId(i64);

/// Represents the type of a category (Expense or Income).
/// Maps to the TEXT column 'type' in the database ('expense' or 'income').
#[derive(Debug, PartialEq, Eq, Type)]
#[sqlx(rename_all = "lowercase")]
pub enum CategoryType {
    /// Represents an expense category (e.g., Food, Transport).
    Expense,
    /// Represents an income category (e.g., Salary, Bonus).
    Income,
}

// --- 3. Table Structs (Domain Models) ---

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

// --- Internal DTOs ---
// DTOs (Data Transfer Objects) used to map between database rows (flat structure)
// and domain models (rich types like Money).

#[derive(FromRow)]
struct AccountDto {
    account_id: AccountId,
    name: String,
    initial_balance: i64, // Stored as minor units (e.g., cents)
    currency: String,     // Stored as currency code (e.g., "USD")
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
struct TransactionDto {
    transaction_id: TransactionId,
    date: NaiveDate,
    amount: i64,      // Stored as minor units
    currency: String, // Stored as currency code
    memo: Option<String>,
    category_id: CategoryId,
    account_id: AccountId,
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

// --- 4. Repository Trait (Abstraction) ---

/// Defines the interface for database operations.
/// The application core depends on this trait, allowing for dependency injection and easier testing.
pub trait KakeiRepository {
    /// Adds a new transaction record to the repository.
    ///
    /// # Arguments
    ///
    /// * `date` - The date of the transaction.
    /// * `amount` - The transaction amount (Money type including currency).
    /// * `memo` - An optional memo.
    /// * `category_id` - The ID of the category.
    /// * `account_id` - The ID of the account.
    ///
    /// # Returns
    ///
    /// Returns the `TransactionId` of the newly created record on success.
    fn add_transaction(
        &self,
        date: NaiveDate,
        amount: Money,
        memo: Option<&str>,
        category_id: CategoryId,
        account_id: AccountId,
    ) -> impl std::future::Future<Output = Result<TransactionId, DbError>> + Send;

    /// Retrieves a list of all categories from the repository.
    ///
    /// # Returns
    ///
    /// Returns a vector of `Category` structs.
    fn get_all_categories(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Category>, DbError>> + Send;
}

// --- 5. Database Implementation (Concrete) ---

/// The concrete implementation of `KakeiRepository` using `sqlx` and SQLite.
pub struct SqliteKakeiRepository {
    /// The connection pool to the SQLite database.
    pool: Pool<Sqlite>,
}

impl SqliteKakeiRepository {
    /// Creates a new instance of `SqliteKakeiRepository` and establishes a connection pool.
    ///
    /// If the database file does not exist at the specified path, it will be created.
    /// **Important:** This method enables Foreign Key constraints for SQLite.
    ///
    /// # Arguments
    ///
    /// * `db_path` - The file path to the SQLite database (e.g., "kakei.db").
    pub async fn new(db_path: &str) -> Result<Self, DbError> {
        let connection_string: String = format!("sqlite:{}", db_path);

        // Configure SQLite options explicitly
        let options: SqliteConnectOptions = connection_string
            .parse::<SqliteConnectOptions>()?
            .create_if_missing(true) // Enable automatic file creation
            .foreign_keys(true); // Enable foreign key constraints

        let pool: Pool<Sqlite> = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        Ok(Self { pool })
    }

    /// Runs database migrations to initialize the schema tables.
    ///
    /// Creates `Categories`, `Accounts`, and `Transactions` tables if they do not exist.
    pub async fn migrate(&self) -> Result<(), DbError> {
        // Categories table
        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS Categories (
                category_id INTEGER PRIMARY KEY AUTOINCREMENT, 
                name TEXT NOT NULL UNIQUE,
                type TEXT NOT NULL CHECK(type IN ('expense', 'income'))
            );
            ",
        )
        .execute(&self.pool)
        .await?;

        // Accounts table (includes currency)
        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS Accounts (
                account_id INTEGER PRIMARY KEY AUTOINCREMENT, 
                name TEXT NOT NULL UNIQUE,
                initial_balance INTEGER NOT NULL DEFAULT 0,
                currency TEXT NOT NULL DEFAULT 'JPY'
            );
            ",
        )
        .execute(&self.pool)
        .await?;

        // Transactions table (includes currency)
        // Added CHECK constraint for ISO 8601 date format (YYYY-MM-DD)
        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS Transactions (
                transaction_id INTEGER PRIMARY KEY AUTOINCREMENT, 
                date TEXT NOT NULL CHECK (date GLOB '[0-9][0-9][0-9][0-9]-[0-1][0-9]-[0-3][0-9]'),
                amount INTEGER NOT NULL, 
                currency TEXT NOT NULL DEFAULT 'JPY',
                memo TEXT,
                category_id INTEGER NOT NULL, 
                account_id INTEGER NOT NULL,
                FOREIGN KEY (category_id) REFERENCES Categories(category_id),
                FOREIGN KEY (account_id) REFERENCES Accounts(account_id)
            );
            ",
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

// --- 6. Trait Implementation ---

impl KakeiRepository for SqliteKakeiRepository {
    async fn add_transaction(
        &self,
        date: NaiveDate,
        amount: Money,
        memo: Option<&str>,
        category_id: CategoryId,
        account_id: AccountId,
    ) -> Result<TransactionId, DbError> {
        // Deconstruct Money into minor units (integer) and currency code (string) for storage
        let amount_minor: i64 = amount.to_minor()?;
        let currency_code: String = amount.currency().to_string();

        let last_id: i64 = sqlx::query(
            "
            INSERT INTO Transactions (date, amount, currency, memo, category_id, account_id)
            VALUES (?, ?, ?, ?, ?, ?)
            ",
        )
        .bind(date)
        .bind(amount_minor)
        .bind(currency_code)
        .bind(memo)
        .bind(category_id)
        .bind(account_id)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(TransactionId(last_id))
    }

    async fn get_all_categories(&self) -> Result<Vec<Category>, DbError> {
        let categories: Vec<Category> =
            sqlx::query_as::<_, Category>("SELECT category_id, name, type FROM Categories")
                .fetch_all(&self.pool)
                .await?;

        Ok(categories)
    }
}

// --- 7. Tests ---

#[cfg(test)]
mod tests {
    /// Group tests for SqliteKakeiRepository
    mod sqlite_kakei_repository {
        use crate::*;
        use rust_decimal::prelude::*;
        use sqlx::Row;

        /// Helper function to create an in-memory database and run migrations.
        async fn create_test_repo() -> SqliteKakeiRepository {
            let repo: SqliteKakeiRepository = SqliteKakeiRepository::new(":memory:")
                .await
                .expect("Failed to create in-memory database");

            repo.migrate().await.expect("Failed to run migrations");

            repo
        }

        /// Helper to seed necessary master data (Category & Account) for foreign keys.
        async fn seed_master_data(repo: &SqliteKakeiRepository) -> (CategoryId, AccountId) {
            let cat_result =
                sqlx::query("INSERT INTO Categories (name, type) VALUES ('Test Food', 'expense')")
                    .execute(&repo.pool)
                    .await
                    .expect("Failed to seed category");
            let cat_id: i64 = cat_result.last_insert_rowid();

            // Insert a dummy account with JPY currency
            let acc_result = sqlx::query(
                "INSERT INTO Accounts (name, initial_balance, currency) VALUES ('Test Cash', 1000, 'JPY')"
            )
            .execute(&repo.pool)
            .await
            .expect("Failed to seed account");
            let acc_id: i64 = acc_result.last_insert_rowid();

            (CategoryId(cat_id), AccountId(acc_id))
        }

        #[tokio::test]
        async fn test_migration_creates_tables() {
            let repo: SqliteKakeiRepository = create_test_repo().await;

            let table_exists: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM sqlite_schema WHERE type='table' AND name='Transactions')"
            )
            .fetch_one(&repo.pool)
            .await
            .unwrap();

            assert!(
                table_exists,
                "Transactions table should be created by migration"
            );
        }

        #[tokio::test]
        async fn test_add_transaction_success_jpy() {
            let repo: SqliteKakeiRepository = create_test_repo().await;
            let (cat_id, acc_id): (CategoryId, AccountId) = seed_master_data(&repo).await;

            let date: NaiveDate = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
            let amount: Money = Money::jpy(-500);
            let memo: Option<&str> = Some("Test Lunch");

            let result: Result<TransactionId, DbError> = repo
                .add_transaction(date, amount, memo, cat_id, acc_id)
                .await;

            assert!(result.is_ok());
            let tx_id: TransactionId = result.unwrap();

            let row = sqlx::query(
                "SELECT amount, currency, memo FROM Transactions WHERE transaction_id = ?",
            )
            .bind(tx_id)
            .fetch_one(&repo.pool)
            .await
            .expect("Failed to fetch inserted transaction");

            let db_amount: i64 = row.get("amount");
            let db_currency: String = row.get("currency");
            let db_memo: String = row.get("memo");

            assert_eq!(db_amount, -500); // JPY minor unit is same as integer
            assert_eq!(db_currency, "JPY");
            assert_eq!(db_memo, "Test Lunch");
        }

        #[tokio::test]
        async fn test_add_transaction_success_usd() {
            let repo: SqliteKakeiRepository = create_test_repo().await;
            let (cat_id, acc_id): (CategoryId, AccountId) = seed_master_data(&repo).await;

            let date: NaiveDate = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
            let amount: Money = Money::usd(dec!(-10.50));
            let memo: Option<&str> = Some("Test Lunch USD");

            let result: Result<TransactionId, DbError> = repo
                .add_transaction(date, amount, memo, cat_id, acc_id)
                .await;

            assert!(result.is_ok());
            let tx_id: TransactionId = result.unwrap();

            let row =
                sqlx::query("SELECT amount, currency FROM Transactions WHERE transaction_id = ?")
                    .bind(tx_id)
                    .fetch_one(&repo.pool)
                    .await
                    .expect("Failed to fetch inserted transaction");

            let db_amount: i64 = row.get("amount");
            let db_currency: String = row.get("currency");

            assert_eq!(db_amount, -1050); // 10.50 * 100
            assert_eq!(db_currency, "USD");
        }

        #[tokio::test]
        async fn test_get_all_categories() {
            let repo: SqliteKakeiRepository = create_test_repo().await;

            sqlx::query("INSERT INTO Categories (name, type) VALUES ('Salary', 'income')")
                .execute(&repo.pool)
                .await
                .unwrap();
            sqlx::query("INSERT INTO Categories (name, type) VALUES ('Transport', 'expense')")
                .execute(&repo.pool)
                .await
                .unwrap();

            let categories: Vec<Category> = repo
                .get_all_categories()
                .await
                .expect("Failed to get categories");

            assert!(categories.len() >= 2);

            let salary: &Category = categories.iter().find(|c| c.name == "Salary").unwrap();
            assert_eq!(salary.type_, CategoryType::Income);

            let transport: &Category = categories.iter().find(|c| c.name == "Transport").unwrap();
            assert_eq!(transport.type_, CategoryType::Expense);
        }

        #[tokio::test]
        async fn test_add_transaction_foreign_key_error() {
            let repo: SqliteKakeiRepository = create_test_repo().await;
            let date: NaiveDate = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
            let amount: Money = Money::jpy(-500);
            let invalid_cat_id: CategoryId = CategoryId(999);
            let invalid_acc_id: AccountId = AccountId(999);

            let result: Result<TransactionId, DbError> = repo
                .add_transaction(date, amount, None, invalid_cat_id, invalid_acc_id)
                .await;

            assert!(result.is_err(), "Should fail due to foreign key constraint");
        }

        #[tokio::test]
        async fn test_add_transaction_with_none_memo() {
            let repo: SqliteKakeiRepository = create_test_repo().await;
            let (cat_id, acc_id): (CategoryId, AccountId) = seed_master_data(&repo).await;

            let date: NaiveDate = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
            let amount: Money = Money::jpy(-100);

            // Pass None to memo
            let result: Result<TransactionId, DbError> = repo
                .add_transaction(date, amount, None, cat_id, acc_id)
                .await;

            assert!(result.is_ok());

            // Verify stored as NULL
            let row = sqlx::query("SELECT memo FROM Transactions WHERE transaction_id = ?")
                .bind(result.unwrap())
                .fetch_one(&repo.pool)
                .await
                .unwrap();

            let db_memo: Option<String> = row.get("memo");
            assert_eq!(db_memo, None);
        }

        // New test for Date Format Check constraint
        #[tokio::test]
        async fn test_transaction_date_format_constraint() {
            let repo: SqliteKakeiRepository = create_test_repo().await;
            let (cat_id, acc_id): (CategoryId, AccountId) = seed_master_data(&repo).await;

            // Attempt to insert an invalid date format using raw SQL
            // (add_transaction uses NaiveDate which guarantees format, so we must use raw SQL to test DB constraint)
            let result = sqlx::query(
                "INSERT INTO Transactions (date, amount, currency, memo, category_id, account_id) VALUES (?, ?, ?, ?, ?, ?)"
            )
            .bind("2025/01/01") // Invalid format (slashes instead of dashes)
            .bind(-100)
            .bind("JPY")
            .bind("Invalid Date")
            .bind(cat_id)
            .bind(acc_id)
            .execute(&repo.pool)
            .await;

            assert!(
                result.is_err(),
                "Should fail due to CHECK constraint on date format"
            );
        }
    }
}
