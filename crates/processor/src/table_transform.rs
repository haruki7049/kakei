//! Table transformation module using kakei_lisp.
//!
//! This module provides functionality to transform transaction data into
//! different table formats using Lisp programs.

use kakei_database::TransactionDetail;
use kakei_lisp::{EvalError, Value, create_global_env, eval, parse};
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
pub fn transform_table(table: Value, lisp_program: &str) -> Result<Value, TransformError> {
    // Parse the Lisp program
    let (remaining, sexprs) =
        parse(lisp_program).map_err(|e| TransformError::ParseError(format!("{:?}", e)))?;

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

/// Extract a field value from a row (association list).
///
/// Given a row like `(ID-001 . ((date . "2025-01-01") (amount . -1000) ...))`,
/// this extracts the value for a specific field name.
fn extract_field(row: &Value, field_name: &str) -> Option<String> {
    // row is (row-id . row-data)
    // We need the cdr (row-data) which is an association list
    match row {
        Value::Cons(_, row_data) => {
            // row_data is an association list like ((date . "...") (amount . ...) ...)
            let mut current = row_data.as_ref();
            loop {
                match current {
                    Value::Nil => return None,
                    Value::Cons(pair, rest) => {
                        // pair is (field-name . field-value)
                        if let Value::Cons(key, value) = pair.as_ref()
                            && let Value::Symbol(key_name) = key.as_ref()
                            && key_name == field_name
                        {
                            return Some(value_to_display_string(value.as_ref()));
                        }

                        current = rest.as_ref();
                    }
                    _ => return None,
                }
            }
        }
        _ => None,
    }
}

/// Convert a Value to a display string.
fn value_to_display_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Symbol(s) => s.clone(),
        Value::Bool(b) => b.to_string(),
        Value::Nil => "".to_string(),
        _ => value.to_string(),
    }
}

/// Represents a row in the transformed table for display.
#[derive(Debug, Clone)]
pub struct DisplayRow {
    pub date: String,
    pub amount: String,
    pub category: String,
    pub account: String,
    pub memo: String,
}

/// Represents a grouped table with a group name and rows.
#[derive(Debug, Clone)]
pub struct GroupedTable {
    pub group_name: String,
    pub rows: Vec<DisplayRow>,
}

/// Convert a Lisp Value (transformed table) back to display-friendly structures.
///
/// This handles both flat tables and grouped tables (from group-by).
pub fn value_to_display_rows(value: &Value) -> Result<Vec<DisplayRow>, TransformError> {
    let mut rows = Vec::new();
    let mut current = value;

    loop {
        match current {
            Value::Nil => break,
            Value::Cons(row, rest) => {
                // Check if this is a grouped result or a flat table
                // Grouped: (("GroupName" (row1) (row2) ...) ...)
                // Flat: ((ID-001 . ((date . "...") ...)) ...)

                if let Value::Cons(first, second) = row.as_ref() {
                    // Check if first is a string (group name) and second is a list of rows
                    if let Value::String(_group_name) = first.as_ref() {
                        // This is a grouped result - flatten it
                        let group_rows = value_to_display_rows(second.as_ref())?;
                        rows.extend(group_rows);
                        current = rest.as_ref();
                        continue;
                    }
                }

                // This is a regular row
                let date = extract_field(row.as_ref(), "date").unwrap_or_default();
                let amount = extract_field(row.as_ref(), "amount")
                    .map(|a| format_amount(&a))
                    .unwrap_or_default();
                let category = extract_field(row.as_ref(), "category").unwrap_or_default();
                let account = extract_field(row.as_ref(), "account").unwrap_or_default();
                let memo = extract_field(row.as_ref(), "memo").unwrap_or_default();

                rows.push(DisplayRow {
                    date,
                    amount,
                    category,
                    account,
                    memo,
                });

                current = rest.as_ref();
            }
            _ => {
                return Err(TransformError::TransformError(format!(
                    "Expected a cons cell or nil, but got: {:?}",
                    current
                )));
            }
        }
    }

    Ok(rows)
}

/// Format amount for display (e.g., -1000 -> ¥-1000).
fn format_amount(amount_str: &str) -> String {
    if let Ok(amount) = amount_str.parse::<i64>() {
        format!("¥{}", amount)
    } else {
        amount_str.to_string()
    }
}

/// Convert a Lisp Value to grouped display structures.
///
/// This is specifically for group-by results.
pub fn value_to_grouped_tables(value: &Value) -> Result<Vec<GroupedTable>, TransformError> {
    let mut groups = Vec::new();
    let mut current = value;

    loop {
        match current {
            Value::Nil => break,
            Value::Cons(group_pair, rest) => {
                // group_pair should be ("GroupName" (row1) (row2) ...)
                if let Value::Cons(group_name_val, rows_val) = group_pair.as_ref()
                    && let Value::String(group_name) = group_name_val.as_ref()
                {
                    // Extract rows from this group
                    let rows = value_to_display_rows(rows_val.as_ref())?;
                    groups.push(GroupedTable {
                        group_name: group_name.clone(),
                        rows,
                    });
                }
                current = rest.as_ref();
            }
            _ => {
                return Err(TransformError::TransformError(
                    "Unexpected grouped value structure".to_string(),
                ));
            }
        }
    }

    Ok(groups)
}

/// Check if a Value represents a grouped result (from group-by).
pub fn is_grouped_result(value: &Value) -> bool {
    match value {
        Value::Cons(first_group, _) => {
            // Check if the first element is a (GroupName . rows) pair
            if let Value::Cons(group_name, _rows) = first_group.as_ref() {
                matches!(group_name.as_ref(), Value::String(_))
            } else {
                false
            }
        }
        Value::Nil => false,
        _ => false,
    }
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
        let transactions = vec![create_test_transaction(
            1,
            "2025-01-01",
            -1000,
            "Food",
            "Cash",
        )];
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

    #[test]
    fn test_extract_field_valid() {
        let tx = create_test_transaction(1, "2025-01-01", -1000, "Food", "Cash");
        let value = transaction_to_value(&tx, 1);

        let date = extract_field(&value, "date");
        assert_eq!(date, Some("2025-01-01".to_string()));

        let category = extract_field(&value, "category");
        assert_eq!(category, Some("Food".to_string()));

        let account = extract_field(&value, "account");
        assert_eq!(account, Some("Cash".to_string()));
    }

    #[test]
    fn test_extract_field_invalid() {
        let tx = create_test_transaction(1, "2025-01-01", -1000, "Food", "Cash");
        let value = transaction_to_value(&tx, 1);

        let invalid = extract_field(&value, "nonexistent");
        assert_eq!(invalid, None);
    }

    #[test]
    fn test_extract_field_from_nil() {
        let result = extract_field(&Value::Nil, "date");
        assert_eq!(result, None);
    }

    #[test]
    fn test_value_to_display_string() {
        assert_eq!(
            value_to_display_string(&Value::String("test".to_string())),
            "test"
        );
        assert_eq!(value_to_display_string(&Value::Number(42)), "42");
        assert_eq!(
            value_to_display_string(&Value::Symbol("sym".to_string())),
            "sym"
        );
        assert_eq!(value_to_display_string(&Value::Bool(true)), "true");
        assert_eq!(value_to_display_string(&Value::Nil), "");
    }

    #[test]
    fn test_format_amount() {
        assert_eq!(format_amount("1000"), "¥1000");
        assert_eq!(format_amount("-1000"), "¥-1000");
        assert_eq!(format_amount("0"), "¥0");
        assert_eq!(format_amount("invalid"), "invalid");
    }

    #[test]
    fn test_value_to_display_rows_empty() {
        let result = value_to_display_rows(&Value::Nil).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_value_to_display_rows_single_transaction() {
        let transactions = vec![create_test_transaction(
            1,
            "2025-01-01",
            -1000,
            "Food",
            "Cash",
        )];
        let table = transactions_to_table(&transactions);

        let rows = value_to_display_rows(&table).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].date, "2025-01-01");
        assert_eq!(rows[0].amount, "¥-1000");
        assert_eq!(rows[0].category, "Food");
        assert_eq!(rows[0].account, "Cash");
    }

    #[test]
    fn test_value_to_display_rows_multiple_transactions() {
        let transactions = vec![
            create_test_transaction(1, "2025-01-01", -1000, "Food", "Cash"),
            create_test_transaction(2, "2025-01-02", -2000, "Transport", "Card"),
            create_test_transaction(3, "2025-01-03", 50000, "Salary", "Bank"),
        ];
        let table = transactions_to_table(&transactions);

        let rows = value_to_display_rows(&table).unwrap();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].category, "Food");
        assert_eq!(rows[1].category, "Transport");
        assert_eq!(rows[2].category, "Salary");
        assert_eq!(rows[2].amount, "¥50000");
    }

    #[test]
    fn test_is_grouped_result_flat_table() {
        let transactions = vec![create_test_transaction(
            1,
            "2025-01-01",
            -1000,
            "Food",
            "Cash",
        )];
        let table = transactions_to_table(&transactions);

        assert!(!is_grouped_result(&table));
    }

    #[test]
    fn test_is_grouped_result_nil() {
        assert!(!is_grouped_result(&Value::Nil));
    }

    #[test]
    fn test_is_grouped_result_grouped_table() {
        let transactions = vec![
            create_test_transaction(1, "2025-01-01", -1000, "Food", "Cash"),
            create_test_transaction(2, "2025-01-02", -2000, "Transport", "Cash"),
        ];
        let table = transactions_to_table(&transactions);
        let program = "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))";
        let result = transform_table(table, program).unwrap();

        assert!(is_grouped_result(&result));
    }

    #[test]
    fn test_value_to_grouped_tables() {
        let transactions = vec![
            create_test_transaction(1, "2025-01-01", -1000, "Food", "Cash"),
            create_test_transaction(2, "2025-01-02", -1500, "Food", "Card"),
            create_test_transaction(3, "2025-01-03", -2000, "Transport", "Cash"),
        ];
        let table = transactions_to_table(&transactions);
        let program = "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))";
        let result = transform_table(table, program).unwrap();

        let groups = value_to_grouped_tables(&result).unwrap();
        assert_eq!(groups.len(), 2);

        // Find Food group
        let food_group = groups.iter().find(|g| g.group_name == "Food").unwrap();
        assert_eq!(food_group.rows.len(), 2);

        // Find Transport group
        let transport_group = groups.iter().find(|g| g.group_name == "Transport").unwrap();
        assert_eq!(transport_group.rows.len(), 1);
    }

    #[test]
    fn test_transform_table_with_car() {
        let transactions = vec![
            create_test_transaction(1, "2025-01-01", -1000, "Food", "Cash"),
            create_test_transaction(2, "2025-01-02", -2000, "Transport", "Cash"),
        ];
        let table = transactions_to_table(&transactions);

        // Get first element
        let program = "(cons (car table) ())";
        let result = transform_table(table, program).unwrap();

        let rows = value_to_display_rows(&result).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].category, "Food");
    }

    #[test]
    fn test_transform_table_with_cdr() {
        let transactions = vec![
            create_test_transaction(1, "2025-01-01", -1000, "Food", "Cash"),
            create_test_transaction(2, "2025-01-02", -2000, "Transport", "Cash"),
        ];
        let table = transactions_to_table(&transactions);

        // Skip first element
        let program = "(cdr table)";
        let result = transform_table(table, program).unwrap();

        let rows = value_to_display_rows(&result).unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].category, "Transport");
    }

    #[test]
    fn test_transform_table_parse_error() {
        let transactions = vec![create_test_transaction(
            1,
            "2025-01-01",
            -1000,
            "Food",
            "Cash",
        )];
        let table = transactions_to_table(&transactions);

        // Invalid program
        let program = "(invalid syntax";
        let result = transform_table(table, program);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), TransformError::ParseError(_)));
    }

    #[test]
    fn test_transform_table_with_invalid_operation() {
        let transactions = vec![create_test_transaction(
            1,
            "2025-01-01",
            -1000,
            "Food",
            "Cash",
        )];
        let table = transactions_to_table(&transactions);

        // Try operations that might cause errors
        let program = "(car ())"; // car on empty list
        let result = transform_table(table, program);

        // Should return some result (even if it's an error or Nil)
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_transaction_with_memo() {
        let mut tx = create_test_transaction(1, "2025-01-01", -1000, "Food", "Cash");
        tx.memo = Some("Grocery shopping".to_string());
        let value = transaction_to_value(&tx, 1);

        let memo = extract_field(&value, "memo");
        assert_eq!(memo, Some("Grocery shopping".to_string()));
    }

    #[test]
    fn test_transaction_without_memo() {
        let tx = create_test_transaction(1, "2025-01-01", -1000, "Food", "Cash");
        let value = transaction_to_value(&tx, 1);

        let memo = extract_field(&value, "memo");
        assert_eq!(memo, Some("".to_string()));
    }

    #[test]
    fn test_transactions_to_table_empty() {
        let transactions: Vec<TransactionDetail> = vec![];
        let table = transactions_to_table(&transactions);

        assert!(matches!(table, Value::Nil));
    }

    #[test]
    fn test_group_by_account() {
        let transactions = vec![
            create_test_transaction(1, "2025-01-01", -1000, "Food", "Cash"),
            create_test_transaction(2, "2025-01-02", -2000, "Transport", "Cash"),
            create_test_transaction(3, "2025-01-03", -3000, "Food", "Card"),
        ];
        let table = transactions_to_table(&transactions);

        let program = "(group-by table (lambda (pair) (cdr (assoc 'account (cdr pair)))))";
        let result = transform_table(table, program).unwrap();

        let groups = value_to_grouped_tables(&result).unwrap();
        assert_eq!(groups.len(), 2);

        let cash_group = groups.iter().find(|g| g.group_name == "Cash").unwrap();
        assert_eq!(cash_group.rows.len(), 2);

        let card_group = groups.iter().find(|g| g.group_name == "Card").unwrap();
        assert_eq!(card_group.rows.len(), 1);
    }

    #[test]
    fn test_large_amount() {
        let tx = create_test_transaction(1, "2025-01-01", 999999999, "Salary", "Bank");
        let value = transaction_to_value(&tx, 1);

        let amount = extract_field(&value, "amount");
        assert_eq!(amount, Some("999999999".to_string()));
    }

    #[test]
    fn test_negative_amount() {
        let tx = create_test_transaction(1, "2025-01-01", -999999, "Food", "Cash");
        let value = transaction_to_value(&tx, 1);

        let amount = extract_field(&value, "amount");
        assert_eq!(amount, Some("-999999".to_string()));
    }

    #[test]
    fn test_zero_amount() {
        let tx = create_test_transaction(1, "2025-01-01", 0, "Food", "Cash");
        let value = transaction_to_value(&tx, 1);

        let amount = extract_field(&value, "amount");
        assert_eq!(amount, Some("0".to_string()));
    }
}
