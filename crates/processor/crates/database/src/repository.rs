use crate::dto::{AccountDto, TransactionDetailDto};
use crate::error::DbError;
use crate::models::{Account, Category, TransactionDetail};
use crate::types::{AccountId, CategoryId, CategoryType, TransactionId};
use chrono::NaiveDate;
use kakei_money::{Currency, Money, MoneyError};
use sqlx::Pool;
use sqlx::sqlite::{Sqlite, SqliteConnectOptions, SqlitePoolOptions};
use tracing::{debug, info, instrument};

/// Maximum number of connections in the SQLite connection pool
const MAX_POOL_CONNECTIONS: u32 = 5;

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

    /// Finds a category by its name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the category to find.
    ///
    /// # Returns
    ///
    /// Returns `Some(Category)` if found, or `None` if it does not exist.
    fn find_category_by_name(
        &self,
        name: &str,
    ) -> impl std::future::Future<Output = Result<Option<Category>, DbError>> + Send;

    /// Finds an account by its name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the account to find.
    ///
    /// # Returns
    ///
    /// Returns `Some(Account)` if found, or `None` if it does not exist.
    fn find_account_by_name(
        &self,
        name: &str,
    ) -> impl std::future::Future<Output = Result<Option<Account>, DbError>> + Send;

    /// Creates a new category if it doesn't exist, or returns the existing one.
    /// This method is idempotent (safe to call multiple times).
    ///
    /// # Arguments
    /// * `name` - The name of the category.
    /// * `type_` - The type of the category.
    fn create_category(
        &self,
        name: &str,
        type_: CategoryType,
    ) -> impl std::future::Future<Output = Result<CategoryId, DbError>> + Send;

    /// Creates a new account if it doesn't exist, or returns the existing one.
    /// This method is idempotent (safe to call multiple times).
    ///
    /// # Arguments
    /// * `name` - The name of the account.
    /// * `initial_balance` - The initial balance.
    fn create_account(
        &self,
        name: &str,
        initial_balance: Money,
    ) -> impl std::future::Future<Output = Result<AccountId, DbError>> + Send;

    /// Retrieves a list of recent transactions, including joined category and account names.
    ///
    /// The results are ordered by date descending (newest first).
    ///
    /// # Arguments
    ///
    /// * `limit` - The maximum number of transactions to return.
    ///
    /// # Returns
    ///
    /// Returns a vector of `TransactionDetail` structs.
    fn get_recent_transactions(
        &self,
        limit: i64,
    ) -> impl std::future::Future<Output = Result<Vec<TransactionDetail>, DbError>> + Send;
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
    #[instrument]
    pub async fn new(db_path: &str) -> Result<Self, DbError> {
        info!("Initializing database connection at path: {}", db_path);
        let connection_string: String = format!("sqlite:{}", db_path);

        // Configure SQLite options explicitly
        let options: SqliteConnectOptions = connection_string
            .parse::<SqliteConnectOptions>()?
            .create_if_missing(true) // Enable automatic file creation
            .foreign_keys(true); // Enable foreign key constraints

        debug!(
            "Creating connection pool with max {} connections",
            MAX_POOL_CONNECTIONS
        );
        let pool: Pool<Sqlite> = SqlitePoolOptions::new()
            .max_connections(MAX_POOL_CONNECTIONS)
            .connect_with(options)
            .await?;

        info!("Database connection established successfully");
        Ok(Self { pool })
    }

    /// Exposes the underlying connection pool.
    /// This is useful for testing or advanced operations.
    pub fn get_pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }
}

// --- Trait Implementation ---

impl KakeiRepository for SqliteKakeiRepository {
    #[instrument(skip(self))]
    async fn add_transaction(
        &self,
        date: NaiveDate,
        amount: Money,
        memo: Option<&str>,
        category_id: CategoryId,
        account_id: AccountId,
    ) -> Result<TransactionId, DbError> {
        debug!(
            "Adding transaction: date={}, amount={:?}, category_id={:?}, account_id={:?}",
            date, amount, category_id, account_id
        );

        // 1. Validate that the transaction currency matches the account currency
        // Fetch the account's currency
        debug!("Validating account currency");
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
            debug!(
                "Currency mismatch detected: transaction={:?}, account={:?}",
                amount.currency(),
                account_currency
            );
            return Err(DbError::Money(MoneyError::CurrencyMismatch(
                amount.currency(),
                account_currency,
            )));
        }

        // 2. Deconstruct Money into minor units (integer) and currency code (string) for storage
        // Use '?' to propagate overflow errors
        let amount_minor: i64 = amount.to_minor()?;
        let currency_code: String = amount.currency().to_string();

        debug!("Inserting transaction into database");
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

        info!("Transaction added successfully with ID: {}", last_id);
        Ok(TransactionId(last_id))
    }

    #[instrument(skip(self))]
    async fn get_all_categories(&self) -> Result<Vec<Category>, DbError> {
        debug!("Fetching all categories from database");
        let categories: Vec<Category> =
            sqlx::query_as::<_, Category>("SELECT category_id, name, type FROM Categories")
                .fetch_all(&self.pool)
                .await?;

        debug!("Found {} categories", categories.len());
        Ok(categories)
    }

    #[instrument(skip(self))]
    async fn find_category_by_name(&self, name: &str) -> Result<Option<Category>, DbError> {
        debug!("Finding category by name: {}", name);
        let category = sqlx::query_as::<_, Category>(
            "SELECT category_id, name, type FROM Categories WHERE name = ?",
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;

        match &category {
            Some(c) => debug!("Category found: {:?}", c.category_id),
            None => debug!("Category not found"),
        }
        Ok(category)
    }

    #[instrument(skip(self))]
    async fn find_account_by_name(&self, name: &str) -> Result<Option<Account>, DbError> {
        debug!("Finding account by name: {}", name);
        // Use DTO to handle Money type conversion
        let dto = sqlx::query_as::<_, AccountDto>(
            "SELECT account_id, name, initial_balance, currency FROM Accounts WHERE name = ?",
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;

        // Convert DTO to Domain Model if present
        match dto {
            Some(d) => {
                debug!("Account found: {:?}", d.account_id);
                Ok(Some(d.try_into()?))
            }
            None => {
                debug!("Account not found");
                Ok(None)
            }
        }
    }

    #[instrument(skip(self))]
    async fn create_category(
        &self,
        name: &str,
        type_: CategoryType,
    ) -> Result<CategoryId, DbError> {
        debug!("Creating category: name={}, type={:?}", name, type_);
        // Do not error if it already exists (INSERT OR IGNORE)
        let result = sqlx::query("INSERT OR IGNORE INTO Categories (name, type) VALUES (?, ?)")
            .bind(name)
            .bind(type_)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() > 0 {
            let category_id = CategoryId(result.last_insert_rowid());
            info!("Category created: {:?}", category_id);
            Ok(category_id)
        } else {
            // If it already exists, fetch and return the ID
            debug!("Category already exists, fetching existing ID");
            let id: i64 = sqlx::query_scalar("SELECT category_id FROM Categories WHERE name = ?")
                .bind(name)
                .fetch_one(&self.pool)
                .await?;
            debug!("Found existing category ID: {}", id);
            Ok(CategoryId(id))
        }
    }

    #[instrument(skip(self, initial_balance))]
    async fn create_account(
        &self,
        name: &str,
        initial_balance: Money,
    ) -> Result<AccountId, DbError> {
        debug!(
            "Creating account: name={}, initial_balance={:?}",
            name, initial_balance
        );
        let amount_minor = initial_balance.to_minor()?;
        let currency_code = initial_balance.currency().to_string();

        let result = sqlx::query(
            "INSERT OR IGNORE INTO Accounts (name, initial_balance, currency) VALUES (?, ?, ?)",
        )
        .bind(name)
        .bind(amount_minor)
        .bind(currency_code)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() > 0 {
            let account_id = AccountId(result.last_insert_rowid());
            info!("Account created: {:?}", account_id);
            Ok(account_id)
        } else {
            debug!("Account already exists, fetching existing ID");
            let id: i64 = sqlx::query_scalar("SELECT account_id FROM Accounts WHERE name = ?")
                .bind(name)
                .fetch_one(&self.pool)
                .await?;
            debug!("Found existing account ID: {}", id);
            Ok(AccountId(id))
        }
    }

    #[instrument(skip(self))]
    async fn get_recent_transactions(&self, limit: i64) -> Result<Vec<TransactionDetail>, DbError> {
        debug!("Fetching recent {} transactions", limit);
        // Implementation details: performs a JOIN across Transactions, Categories, and Accounts.
        let dtos = sqlx::query_as::<_, TransactionDetailDto>(
            "
            SELECT
                t.transaction_id,
                t.date,
                t.amount,
                t.currency,
                t.memo,
                c.name as category_name,
                a.name as account_name
            FROM Transactions t
            JOIN Categories c ON t.category_id = c.category_id
            JOIN Accounts a ON t.account_id = a.account_id
            ORDER BY t.date DESC, t.transaction_id DESC
            LIMIT ?
            ",
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        // DTO -> Domain Model
        let mut result = Vec::new();
        for dto in dtos {
            result.push(dto.try_into()?);
        }

        info!("Retrieved {} transactions", result.len());
        Ok(result)
    }
}

// --- Unit Tests ---

#[cfg(test)]
mod tests {
    use super::*;

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

            sqlx::migrate!("db/migrations")
                .run(repo.get_pool())
                .await
                .expect("Failed to run migrations");

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
