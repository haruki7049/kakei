//! kakei_database crate
//!
//! Handles all database logic using sqlx.
//! This crate provides a strongly-typed interface to the SQLite database via the Repository pattern.

mod dto;
mod error;
mod models;
mod repository;
mod types;

// Re-export useful types for external use
pub use error::DbError;
pub use models::{Account, Category, Transaction, TransactionDetail};
pub use repository::{KakeiRepository, SqliteKakeiRepository};
pub use types::{AccountId, CategoryId, CategoryType, TransactionId};
