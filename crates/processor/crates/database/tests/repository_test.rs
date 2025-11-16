use chrono::NaiveDate;
use kakei_database::{
    AccountId, Category, CategoryId, CategoryType, DbError, KakeiRepository, SqliteKakeiRepository,
    TransactionId,
};
use kakei_money::Money;
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
            .execute(repo.get_pool())
            .await
            .expect("Failed to seed category")
            .last_insert_rowid();

    let acc_id: i64 = sqlx::query(
        "INSERT INTO Accounts (name, initial_balance, currency) VALUES ('Test Cash', 1000, 'JPY')",
    )
    .execute(repo.get_pool())
    .await
    .expect("Failed to seed account")
    .last_insert_rowid();

    (CategoryId(cat_id), AccountId(acc_id))
}

#[tokio::test]
async fn test_integration_add_transaction_success() {
    let repo: SqliteKakeiRepository = create_test_repo().await;
    let (cat_id, acc_id): (CategoryId, AccountId) = seed_master_data(&repo).await;

    let date: NaiveDate = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
    let amount: Money = Money::jpy(-500);
    let memo: Option<&str> = Some("Integration Test");

    let result: Result<TransactionId, DbError> = repo
        .add_transaction(date, amount, memo, cat_id, acc_id)
        .await;

    assert!(result.is_ok());
    let tx_id: TransactionId = result.unwrap();

    // Verify using raw SQL via get_pool()
    let row =
        sqlx::query("SELECT amount, currency, memo FROM Transactions WHERE transaction_id = ?")
            .bind(tx_id)
            .fetch_one(repo.get_pool())
            .await
            .expect("Failed to fetch inserted transaction");

    let db_amount: i64 = row.get("amount");
    assert_eq!(db_amount, -500);
}

#[tokio::test]
async fn test_integration_get_categories() {
    let repo: SqliteKakeiRepository = create_test_repo().await;
    seed_master_data(&repo).await; // Creates 'Test Food'

    let categories: Vec<Category> = repo
        .get_all_categories()
        .await
        .expect("Failed to get categories");

    assert!(!categories.is_empty());
    let cat: &Category = &categories[0];
    assert_eq!(cat.name, "Test Food");
    assert_eq!(cat.type_, CategoryType::Expense);
}
