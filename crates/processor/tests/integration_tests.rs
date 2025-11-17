//! Integration tests for the kakei_processor crate.
//!
//! These tests verify that the table transformation functionality works
//! correctly end-to-end with realistic data scenarios.

use kakei_database::TransactionDetail;
use kakei_money::Money;
use kakei_processor::{transactions_to_table, transform_table, value_to_display_rows, value_to_grouped_tables, is_grouped_result};
use chrono::NaiveDate;
use kakei_database::TransactionId;

/// Helper to create a test transaction
fn create_transaction(id: i64, date: &str, amount: i64, category: &str, account: &str, memo: Option<&str>) -> TransactionDetail {
    TransactionDetail {
        transaction_id: TransactionId(id),
        date: NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap(),
        amount: Money::jpy(amount),
        memo: memo.map(|s| s.to_string()),
        category_name: category.to_string(),
        account_name: account.to_string(),
    }
}

#[test]
fn test_realistic_monthly_transactions() {
    // Simulate a month of transactions
    let transactions = vec![
        create_transaction(1, "2025-01-01", -1000, "Food", "Cash", Some("Lunch")),
        create_transaction(2, "2025-01-05", -2000, "Transport", "Card", Some("Train pass")),
        create_transaction(3, "2025-01-10", -500, "Food", "Cash", None),
        create_transaction(4, "2025-01-15", 50000, "Salary", "Bank", Some("Monthly salary")),
        create_transaction(5, "2025-01-20", -3000, "Hobby", "Card", Some("Books")),
        create_transaction(6, "2025-01-25", -800, "Food", "Cash", Some("Dinner")),
        create_transaction(7, "2025-01-30", -1500, "Transport", "Card", Some("Gas")),
    ];

    let table = transactions_to_table(&transactions);
    let rows = value_to_display_rows(&table).unwrap();

    assert_eq!(rows.len(), 7);
    assert_eq!(rows[0].date, "2025-01-01");
    assert_eq!(rows[3].amount, "¥50000");
}

#[test]
fn test_group_by_category_realistic() {
    let transactions = vec![
        create_transaction(1, "2025-01-01", -1000, "Food", "Cash", None),
        create_transaction(2, "2025-01-02", -1500, "Food", "Card", None),
        create_transaction(3, "2025-01-03", -2000, "Transport", "Cash", None),
        create_transaction(4, "2025-01-04", -3000, "Transport", "Card", None),
        create_transaction(5, "2025-01-05", -500, "Hobby", "Cash", None),
        create_transaction(6, "2025-01-06", 50000, "Salary", "Bank", None),
    ];

    let table = transactions_to_table(&transactions);
    let program = "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))";
    let result = transform_table(table, program).unwrap();

    assert!(is_grouped_result(&result));

    let groups = value_to_grouped_tables(&result).unwrap();
    assert_eq!(groups.len(), 4); // Food, Transport, Hobby, Salary

    // Verify each group
    let food = groups.iter().find(|g| g.group_name == "Food").unwrap();
    assert_eq!(food.rows.len(), 2);

    let transport = groups.iter().find(|g| g.group_name == "Transport").unwrap();
    assert_eq!(transport.rows.len(), 2);

    let hobby = groups.iter().find(|g| g.group_name == "Hobby").unwrap();
    assert_eq!(hobby.rows.len(), 1);

    let salary = groups.iter().find(|g| g.group_name == "Salary").unwrap();
    assert_eq!(salary.rows.len(), 1);
}

#[test]
fn test_empty_transactions_list() {
    let transactions: Vec<TransactionDetail> = vec![];
    let table = transactions_to_table(&transactions);

    let rows = value_to_display_rows(&table).unwrap();
    assert!(rows.is_empty());
}

#[test]
fn test_single_transaction() {
    let transactions = vec![
        create_transaction(1, "2025-01-01", -1000, "Food", "Cash", Some("Only one")),
    ];

    let table = transactions_to_table(&transactions);
    let rows = value_to_display_rows(&table).unwrap();

    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].date, "2025-01-01");
    assert_eq!(rows[0].amount, "¥-1000");
    assert_eq!(rows[0].category, "Food");
    assert_eq!(rows[0].account, "Cash");
    assert_eq!(rows[0].memo, "Only one");
}

#[test]
fn test_invalid_lisp_program() {
    let transactions = vec![
        create_transaction(1, "2025-01-01", -1000, "Food", "Cash", None),
    ];

    let table = transactions_to_table(&transactions);
    
    // Invalid syntax
    let program = "(group-by table";
    let result = transform_table(table, program);
    
    assert!(result.is_err());
}
