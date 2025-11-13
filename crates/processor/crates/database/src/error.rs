use kakei_money::MoneyError;
use thiserror::Error;

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
