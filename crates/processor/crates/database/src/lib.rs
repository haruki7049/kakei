//! kakei_database crate
//!
//! Handles all database logic using sqlx.
//! This crate provides a strongly-typed interface to the SQLite database via the Repository pattern.

use chrono::NaiveDate;
use sqlx::sqlite::{Sqlite, SqlitePoolOptions};
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

// --- 3. Table Structs (Mapping DB rows) ---

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

/// Represents a row in the `Accounts` table.
#[derive(Debug, FromRow)]
pub struct Account {
    /// The unique ID of the account.
    pub account_id: AccountId,
    /// The display name of the account (unique).
    pub name: String,
    /// The initial balance of the account (in yen).
    pub initial_balance: i64,
}

/// Represents a row in the `Transactions` table.
#[derive(Debug, FromRow)]
pub struct Transaction {
    /// The unique ID of the transaction.
    pub transaction_id: TransactionId,
    /// The date of the transaction.
    /// Automatically converted between SQLite TEXT (ISO8601) and `NaiveDate`.
    pub date: NaiveDate,
    /// The amount of the transaction (negative for expense, positive for income).
    pub amount: i64,
    /// An optional memo or note for the transaction.
    pub memo: Option<String>,
    /// The ID of the associated category.
    pub category_id: CategoryId,
    /// The ID of the associated account (source/destination).
    pub account_id: AccountId,
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
    /// * `amount` - The transaction amount.
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
        amount: i64,
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
    /// `Sqlite` implements the `sqlx::Database` trait.
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
        // Append ?create=true to ensure the file is created if missing.
        let connection_string = format!("sqlite:{}", db_path);

        // Configure SQLite options explicitly
        let options = connection_string
            .parse::<sqlx::sqlite::SqliteConnectOptions>()?
            .create_if_missing(true) // This enables automatic file creation
            .foreign_keys(true); // This enables foreign key constraints

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;

        Ok(Self { pool })
    }

    /// Runs database migrations to initialize the schema tables.
    ///
    /// Creates `Categories`, `Accounts`, and `Transactions` tables if they do not exist.
    pub async fn migrate(&self) -> Result<(), DbError> {
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

        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS Accounts (
                account_id INTEGER PRIMARY KEY AUTOINCREMENT, 
                name TEXT NOT NULL UNIQUE,
                initial_balance INTEGER NOT NULL DEFAULT 0
            );
            ",
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS Transactions (
                transaction_id INTEGER PRIMARY KEY AUTOINCREMENT, 
                date TEXT NOT NULL,
                amount INTEGER NOT NULL, 
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
        amount: i64,
        memo: Option<&str>,
        category_id: CategoryId,
        account_id: AccountId,
    ) -> Result<TransactionId, DbError> {
        // Use sqlx functionality (execute) to run the specific SQL query.
        let last_id = sqlx::query(
            "
            INSERT INTO Transactions (date, amount, memo, category_id, account_id)
            VALUES (?, ?, ?, ?, ?)
            ",
        )
        .bind(date)
        .bind(amount)
        .bind(memo)
        .bind(category_id)
        .bind(account_id)
        .execute(&self.pool)
        .await?
        .last_insert_rowid();

        Ok(TransactionId(last_id))
    }

    async fn get_all_categories(&self) -> Result<Vec<Category>, DbError> {
        let categories =
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
        use sqlx::Row;

        /// Helper function to create an in-memory database and run migrations.
        /// This ensures each test runs in a clean, isolated environment.
        async fn create_test_repo() -> SqliteKakeiRepository {
            // Use ":memory:" for a temporary in-memory database
            let repo = SqliteKakeiRepository::new(":memory:")
                .await
                .expect("Failed to create in-memory database");

            repo.migrate().await.expect("Failed to run migrations");

            repo
        }

        /// Helper to seed necessary master data (Category & Account) for foreign keys.
        async fn seed_master_data(repo: &SqliteKakeiRepository) -> (CategoryId, AccountId) {
            // Insert a dummy category
            let cat_id: i64 = sqlx::query(
                "INSERT INTO Categories (name, type) VALUES ('Test Food', 'expense') RETURNING category_id"
            )
            .fetch_one(&repo.pool)
            .await
            .expect("Failed to seed category")
            .get(0);

            // Insert a dummy account
            let acc_id: i64 = sqlx::query(
                "INSERT INTO Accounts (name, initial_balance) VALUES ('Test Cash', 1000) RETURNING account_id"
            )
            .fetch_one(&repo.pool)
            .await
            .expect("Failed to seed account")
            .get(0);

            (CategoryId(cat_id), AccountId(acc_id))
        }

        #[tokio::test]
        async fn test_migration_creates_tables() {
            let repo = create_test_repo().await;

            // Verify tables exist by querying sqlite_schema
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
        async fn test_add_transaction_success() {
            let repo = create_test_repo().await;
            let (cat_id, acc_id) = seed_master_data(&repo).await;

            let date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
            let amount = -500;
            let memo = Some("Test Lunch");

            // Execute the method under test
            let result = repo
                .add_transaction(date, amount, memo, cat_id, acc_id)
                .await;

            assert!(result.is_ok());
            let tx_id = result.unwrap();

            // Verify data was actually inserted
            let row = sqlx::query("SELECT amount, memo FROM Transactions WHERE transaction_id = ?")
                .bind(tx_id)
                .fetch_one(&repo.pool)
                .await
                .expect("Failed to fetch inserted transaction");

            let db_amount: i64 = row.get("amount");
            let db_memo: String = row.get("memo");

            assert_eq!(db_amount, amount);
            assert_eq!(db_memo, "Test Lunch");
        }

        #[tokio::test]
        async fn test_get_all_categories() {
            let repo = create_test_repo().await;

            // Insert multiple categories directly
            sqlx::query("INSERT INTO Categories (name, type) VALUES ('Salary', 'income')")
                .execute(&repo.pool)
                .await
                .unwrap();
            sqlx::query("INSERT INTO Categories (name, type) VALUES ('Transport', 'expense')")
                .execute(&repo.pool)
                .await
                .unwrap();

            // Execute the method under test
            let categories = repo
                .get_all_categories()
                .await
                .expect("Failed to get categories");

            // Verify results
            assert!(categories.len() >= 2);

            let salary = categories.iter().find(|c| c.name == "Salary").unwrap();
            assert_eq!(salary.type_, CategoryType::Income);

            let transport = categories.iter().find(|c| c.name == "Transport").unwrap();
            assert_eq!(transport.type_, CategoryType::Expense);
        }

        #[tokio::test]
        async fn test_add_transaction_foreign_key_error() {
            let repo = create_test_repo().await;
            // No master data seeded

            let date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
            let amount = -500;

            // Specify non-existent IDs
            let invalid_cat_id = CategoryId(999);
            let invalid_acc_id = AccountId(999);

            // Execute
            let result = repo
                .add_transaction(date, amount, None, invalid_cat_id, invalid_acc_id)
                .await;

            // Verify: Should fail due to foreign key constraint
            assert!(result.is_err(), "Should fail due to foreign key constraint");

            match result {
                Err(DbError::Sqlx(e)) => println!("Caught expected SQL error: {}", e),
                _ => panic!("Expected DbError::Sqlx"),
            }
        }

        #[tokio::test]
        async fn test_add_transaction_with_none_memo() {
            let repo = create_test_repo().await;
            let (cat_id, acc_id) = seed_master_data(&repo).await;

            let date = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();

            // Pass None to memo
            let result = repo.add_transaction(date, -100, None, cat_id, acc_id).await;

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
    }
}
