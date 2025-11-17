//! Table transformation module using kakei_lisp.
//!
//! This module provides functionality to transform transaction data into
//! different table formats using Lisp programs.

use kakei_database::TransactionDetail;
use kakei_lisp::{create_global_env, eval, parse, EvalError, Value};
use std::rc::Rc;
use thiserror::Error;

/// Errors that can occur during table transformation.
#[derive(Debug, Error)]
pub enum TransformError {
    /// Parse error from the Lisp parser.
    #[error("Parse error: {0}")]
    ParseError(String),
    /// Evaluation error from the Lisp evaluator.
    #[error("Evaluation error: {0}")]
    EvalError(#[from] EvalError),
    /// General transformation error.
    #[error("Transformation error: {0}")]
    TransformError(String),
}

/// Convert a TransactionDetail into a Lisp Value (association list).
///
/// The format is: `(row-id . ((date . "2025-01-01") (amount . -1000) (category . "Food") ...))`
fn transaction_to_value(tx: &TransactionDetail, row_id: usize) -> Value {
    // Create the row data as an association list
    let mut row_data = Value::Nil;

    // Build in reverse order since we're consing onto the front
    // memo
    let memo_val = match &tx.memo {
        Some(m) => Value::String(m.clone()),
        None => Value::String(String::new()),
    };
    let memo_pair = Value::Cons(
        Rc::new(Value::Symbol("memo".to_string())),
        Rc::new(memo_val),
    );
    row_data = Value::Cons(Rc::new(memo_pair), Rc::new(row_data));

    // account
    let account_pair = Value::Cons(
        Rc::new(Value::Symbol("account".to_string())),
        Rc::new(Value::String(tx.account_name.clone())),
    );
    row_data = Value::Cons(Rc::new(account_pair), Rc::new(row_data));

    // category
    let category_pair = Value::Cons(
        Rc::new(Value::Symbol("category".to_string())),
        Rc::new(Value::String(tx.category_name.clone())),
    );
    row_data = Value::Cons(Rc::new(category_pair), Rc::new(row_data));

    // amount (convert to number, minor units)
    let amount_val = Value::Number(tx.amount.to_minor().unwrap_or(0));
    let amount_pair = Value::Cons(
        Rc::new(Value::Symbol("amount".to_string())),
        Rc::new(amount_val),
    );
    row_data = Value::Cons(Rc::new(amount_pair), Rc::new(row_data));

    // date
    let date_pair = Value::Cons(
        Rc::new(Value::Symbol("date".to_string())),
        Rc::new(Value::String(tx.date.to_string())),
    );
    row_data = Value::Cons(Rc::new(date_pair), Rc::new(row_data));

    // Create the row ID
    let row_id_sym = Value::Symbol(format!("ID-{:03}", row_id));

    // Create the pair (row-id . row-data)
    Value::Cons(Rc::new(row_id_sym), Rc::new(row_data))
}

/// Convert a list of TransactionDetail into a Lisp table (list of rows).
pub fn transactions_to_table(transactions: &[TransactionDetail]) -> Value {
    let mut table = Value::Nil;

    // Build in reverse order
    for (i, tx) in transactions.iter().enumerate().rev() {
        let row = transaction_to_value(tx, i + 1);
        table = Value::Cons(Rc::new(row), Rc::new(table));
    }

    table
}

/// Transform a table using a Lisp program.
///
/// The Lisp program receives the table as a variable named `table`.
/// The program should return the transformed table.
pub fn transform_table(
    table: Value,
    lisp_program: &str,
) -> Result<Value, TransformError> {
    // Parse the Lisp program
    let (remaining, sexprs) = parse(lisp_program)
        .map_err(|e| TransformError::ParseError(format!("{:?}", e)))?;

    if !remaining.is_empty() {
        return Err(TransformError::ParseError(format!(
            "Unparsed input: {}",
            remaining
        )));
    }

    // Create environment with built-ins
    let mut env = create_global_env();

    // Define the table variable
    env.define("table".to_string(), table);

    // Evaluate all expressions and return the last result
    let mut result = Value::Nil;
    for sexpr in sexprs {
        result = eval(&sexpr, &mut env)?;
    }

    Ok(result)
}

/// Format a Value into a human-readable string for display.
///
/// This is a simplified formatter that handles the common cases.
pub fn format_value(value: &Value) -> String {
    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use kakei_database::TransactionId;
    use kakei_money::Money;

    fn create_test_transaction(
        id: i64,
        date: &str,
        amount: i64,
        category: &str,
        account: &str,
    ) -> TransactionDetail {
        TransactionDetail {
            transaction_id: TransactionId(id),
            date: NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap(),
            amount: Money::jpy(amount),
            memo: None,
            category_name: category.to_string(),
            account_name: account.to_string(),
        }
    }

    #[test]
    fn test_transaction_to_value() {
        let tx = create_test_transaction(1, "2025-01-01", -1000, "Food", "Cash");
        let value = transaction_to_value(&tx, 1);

        // Verify it's a cons cell
        assert!(matches!(value, Value::Cons(_, _)));
    }

    #[test]
    fn test_transactions_to_table() {
        let transactions = vec![
            create_test_transaction(1, "2025-01-01", -1000, "Food", "Cash"),
            create_test_transaction(2, "2025-01-02", -2000, "Transport", "Card"),
        ];

        let table = transactions_to_table(&transactions);

        // Verify it's a list
        assert!(matches!(table, Value::Cons(_, _)));
    }

    #[test]
    fn test_transform_table_simple() {
        let transactions = vec![create_test_transaction(1, "2025-01-01", -1000, "Food", "Cash")];
        let table = transactions_to_table(&transactions);

        // Simple program that just returns the table
        let program = "table";
        let result = transform_table(table, program).unwrap();

        assert!(matches!(result, Value::Cons(_, _)));
    }

    #[test]
    fn test_transform_table_with_group_by() {
        let transactions = vec![
            create_test_transaction(1, "2025-01-01", -1000, "Food", "Cash"),
            create_test_transaction(2, "2025-01-02", -2000, "Food", "Card"),
            create_test_transaction(3, "2025-01-03", -3000, "Transport", "Cash"),
        ];
        let table = transactions_to_table(&transactions);

        // Group by category
        let program = "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))";

        let result = transform_table(table, program).unwrap();

        // Verify the result is a list (grouped table)
        assert!(matches!(result, Value::Cons(_, _)));
    }
}
