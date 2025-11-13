use crate::error::DbError;
use crate::models::Category; // Account is used in DTO conversion logic internally
use crate::types::{AccountId, CategoryId, TransactionId};
use chrono::NaiveDate;
use kakei_money::{Currency, Money, MoneyError};
use sqlx::Pool;
use sqlx::sqlite::{Sqlite, SqliteConnectOptions, SqlitePoolOptions};

// --- Repository Trait (Abstraction) ---

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

// --- Database Implementation (Concrete) ---

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

    /// Exposes the underlying connection pool.
    /// This is useful for testing or advanced operations.
    pub fn get_pool(&self) -> &Pool<Sqlite> {
        &self.pool
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

// --- Trait Implementation ---

impl KakeiRepository for SqliteKakeiRepository {
    async fn add_transaction(
        &self,
        date: NaiveDate,
        amount: Money,
        memo: Option<&str>,
        category_id: CategoryId,
        account_id: AccountId,
    ) -> Result<TransactionId, DbError> {
        // 1. Validate that the transaction currency matches the account currency
        // Fetch the account's currency
        let account_currency_str: String =
            sqlx::query_scalar("SELECT currency FROM Accounts WHERE account_id = ?")
                .bind(account_id)
                .fetch_optional(&self.pool)
                .await?
                .ok_or_else(|| DbError::NotFound(format!("Account not found: {:?}", account_id)))?;

        // Parse the DB string into a Currency enum
        let account_currency: Currency = account_currency_str.parse()?;

        // Check for mismatch
        if amount.currency() != account_currency {
            return Err(DbError::Money(MoneyError::CurrencyMismatch(
                amount.currency(),
                account_currency,
            )));
        }

        // 2. Deconstruct Money into minor units (integer) and currency code (string) for storage
        // Use '?' to propagate overflow errors
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

// --- Unit Tests ---

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::CategoryType;

    /// Group tests for SqliteKakeiRepository
    mod sqlite_kakei_repository {
        use super::*;
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
            let cat_id: i64 =
                sqlx::query("INSERT INTO Categories (name, type) VALUES ('Test Food', 'expense')")
                    .execute(&repo.pool)
                    .await
                    .expect("Failed to seed category")
                    .last_insert_rowid();

            let acc_id: i64 = sqlx::query(
                "INSERT INTO Accounts (name, initial_balance, currency) VALUES ('Test Cash', 1000, 'JPY')"
            )
            .execute(&repo.pool)
            .await
            .expect("Failed to seed account")
            .last_insert_rowid();

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

            assert_eq!(db_amount, -500);
            assert_eq!(db_currency, "JPY");
            assert_eq!(db_memo, "Test Lunch");
        }

        #[tokio::test]
        async fn test_add_transaction_success_usd() {
            let repo: SqliteKakeiRepository = create_test_repo().await;

            let cat_id: i64 = sqlx::query(
                "INSERT INTO Categories (name, type) VALUES ('Test Food USD', 'expense')",
            )
            .execute(&repo.pool)
            .await
            .expect("Failed to seed category")
            .last_insert_rowid();

            let acc_id: i64 = sqlx::query(
                "INSERT INTO Accounts (name, initial_balance, currency) VALUES ('Test Card USD', 0, 'USD')"
            )
            .execute(&repo.pool)
            .await
            .expect("Failed to seed account")
            .last_insert_rowid();

            let date: NaiveDate = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
            let amount: Money = Money::usd(dec!(-10.50));
            let memo: Option<&str> = Some("Test Lunch USD");

            let result: Result<TransactionId, DbError> = repo
                .add_transaction(date, amount, memo, CategoryId(cat_id), AccountId(acc_id))
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

            assert_eq!(db_amount, -1050);
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

            assert!(result.is_err());
        }

        #[tokio::test]
        async fn test_add_transaction_with_none_memo() {
            let repo: SqliteKakeiRepository = create_test_repo().await;
            let (cat_id, acc_id): (CategoryId, AccountId) = seed_master_data(&repo).await;

            let date: NaiveDate = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
            let amount: Money = Money::jpy(-100);

            let result: Result<TransactionId, DbError> = repo
                .add_transaction(date, amount, None, cat_id, acc_id)
                .await;

            assert!(result.is_ok());

            let row = sqlx::query("SELECT memo FROM Transactions WHERE transaction_id = ?")
                .bind(result.unwrap())
                .fetch_one(&repo.pool)
                .await
                .unwrap();

            let db_memo: Option<String> = row.get("memo");
            assert_eq!(db_memo, None);
        }

        #[tokio::test]
        async fn test_transaction_date_format_constraint() {
            let repo: SqliteKakeiRepository = create_test_repo().await;
            let (cat_id, acc_id): (CategoryId, AccountId) = seed_master_data(&repo).await;

            let result = sqlx::query(
                "INSERT INTO Transactions (date, amount, currency, memo, category_id, account_id) VALUES (?, ?, ?, ?, ?, ?)"
            )
            .bind("2025/01/01")
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

        #[tokio::test]
        async fn test_add_transaction_currency_mismatch() {
            let repo: SqliteKakeiRepository = create_test_repo().await;
            let (cat_id, acc_id): (CategoryId, AccountId) = seed_master_data(&repo).await;

            let date: NaiveDate = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
            let amount: Money = Money::usd(dec!(10.00));

            let result: Result<TransactionId, DbError> = repo
                .add_transaction(date, amount, None, cat_id, acc_id)
                .await;

            match result {
                Err(DbError::Money(MoneyError::CurrencyMismatch(got, expected))) => {
                    assert_eq!(got, Currency::USD);
                    assert_eq!(expected, Currency::JPY);
                }
                _ => panic!("Expected MoneyError::CurrencyMismatch, got {:?}", result),
            }
        }
    }
}
