use sqlx::Type;

// --- Strong Types (Newtype Pattern & Enums) ---

/// A strongly-typed identifier for a Category.
/// Wraps an `i64` value and maps transparently to the database INTEGER type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(transparent)]
pub struct CategoryId(pub i64);

/// A strongly-typed identifier for an Account.
/// Wraps an `i64` value and maps transparently to the database INTEGER type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(transparent)]
pub struct AccountId(pub i64);

/// A strongly-typed identifier for a Transaction.
/// Wraps an `i64` value and maps transparently to the database INTEGER type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Type)]
#[sqlx(transparent)]
pub struct TransactionId(pub i64);

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
