//! Integration tests for the kakei CLI application.
//!
//! These tests verify that the CLI commands work correctly end-to-end
//! by executing the actual binary with various arguments and checking
//! the output and exit codes.

use assert_cmd::{Command, cargo::cargo_bin_cmd};
use predicates::prelude::*;
use tempfile::TempDir;

/// Helper function to create a test command with a temporary database
fn setup_test_cmd() -> (Command, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let mut cmd = cargo_bin_cmd!();

    // Set HOME to ensure macOS uses temp directory (directories crate on macOS uses ~/Library/Application Support)
    cmd.env("HOME", temp_dir.path());
    // Set up environment to use temp directory for config and data (for XDG-based systems)
    cmd.env("XDG_CONFIG_HOME", temp_dir.path().join("config"));
    cmd.env("XDG_DATA_HOME", temp_dir.path().join("data"));

    (cmd, temp_dir)
}

/// Helper function to initialize a database for testing
fn init_database(temp_dir: &TempDir) {
    let mut cmd = cargo_bin_cmd!();
    // Set HOME to ensure macOS uses temp directory (directories crate on macOS uses ~/Library/Application Support)
    cmd.env("HOME", temp_dir.path());
    // Set up environment to use temp directory for config and data (for XDG-based systems)
    cmd.env("XDG_CONFIG_HOME", temp_dir.path().join("config"));
    cmd.env("XDG_DATA_HOME", temp_dir.path().join("data"));
    cmd.arg("init");

    cmd.assert().success();
}

#[test]
fn test_cli_version() {
    let mut cmd = cargo_bin_cmd!();
    cmd.arg("--version");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("kakei"));
}

#[test]
fn test_cli_help() {
    let mut cmd = cargo_bin_cmd!();
    cmd.arg("--help");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("Commands:"))
        .stdout(predicate::str::contains("init"))
        .stdout(predicate::str::contains("add"))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("transform"));
}

#[test]
fn test_init_command_success() {
    let (mut cmd, temp_dir) = setup_test_cmd();
    cmd.arg("init");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Initialization complete"));

    // Verify database file was created
    // The database path depends on platform and XDG environment variables:
    // - macOS: ~/Library/Application Support/dev.haruki7049.kakei/kakei.db (ignores XDG)
    // - Linux with XDG_DATA_HOME: $XDG_DATA_HOME/kakei/kakei.db
    // - Windows: ~\AppData\Roaming\dev.haruki7049\kakei\data\kakei.db
    let db_path = if cfg!(target_os = "macos") {
        temp_dir.path().join("Library/Application Support/dev.haruki7049.kakei/kakei.db")
    } else if cfg!(target_os = "windows") {
        temp_dir.path().join("AppData/Roaming/dev.haruki7049/kakei/data/kakei.db")
    } else {
        // Linux and other Unix-like systems use XDG_DATA_HOME when set
        temp_dir.path().join("data/kakei/kakei.db")
    };
    assert!(db_path.exists(), "Database file should exist after init at {:?}", db_path);
}

#[test]
fn test_add_command_success() {
    let (mut cmd, temp_dir) = setup_test_cmd();

    // Initialize database first
    init_database(&temp_dir);

    // Add a transaction
    cmd.arg("add")
        .arg("--date")
        .arg("2025-01-01")
        .arg("--amount")
        .arg("-1000")
        .arg("--category")
        .arg("Food")
        .arg("--account")
        .arg("Cash");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Transaction added successfully"));
}

#[test]
fn test_add_command_with_memo() {
    let (mut cmd, temp_dir) = setup_test_cmd();

    // Initialize database first
    init_database(&temp_dir);

    // Add a transaction with memo
    cmd.arg("add")
        .arg("--date")
        .arg("2025-01-01")
        .arg("--amount")
        .arg("-1500")
        .arg("--category")
        .arg("Food")
        .arg("--account")
        .arg("Cash")
        .arg("--memo")
        .arg("Lunch at restaurant");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Transaction added successfully"));
}

#[test]
fn test_add_command_with_positive_amount() {
    let (mut cmd, temp_dir) = setup_test_cmd();

    // Initialize database first
    init_database(&temp_dir);

    // Add income transaction
    cmd.arg("add")
        .arg("--date")
        .arg("2025-01-15")
        .arg("--amount")
        .arg("50000")
        .arg("--category")
        .arg("Salary")
        .arg("--account")
        .arg("Bank")
        .arg("--memo")
        .arg("Monthly salary");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Transaction added successfully"));
}

#[test]
fn test_add_command_with_custom_currency() {
    let (mut cmd, temp_dir) = setup_test_cmd();

    // Initialize database first
    init_database(&temp_dir);

    // Add transaction with default currency (JPY)
    // Note: Using different currency would require all transactions to use same currency
    cmd.arg("add")
        .arg("--date")
        .arg("2025-01-01")
        .arg("--amount")
        .arg("-100")
        .arg("--currency")
        .arg("JPY")
        .arg("--category")
        .arg("Food")
        .arg("--account")
        .arg("Cash");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Transaction added successfully"));
}

#[test]
fn test_list_command_empty() {
    let (mut cmd, temp_dir) = setup_test_cmd();

    // Initialize database first
    init_database(&temp_dir);

    // List transactions (should be empty)
    cmd.arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No transactions found"));
}

#[test]
fn test_list_command_with_transactions() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Initialize database
    init_database(&temp_dir);

    // Add some transactions
    let mut add_cmd1 = cargo_bin_cmd!();
    add_cmd1.env("HOME", temp_dir.path());
    add_cmd1.env("XDG_CONFIG_HOME", temp_dir.path().join("config"));
    add_cmd1.env("XDG_DATA_HOME", temp_dir.path().join("data"));
    add_cmd1
        .arg("add")
        .arg("--date")
        .arg("2025-01-01")
        .arg("--amount")
        .arg("-1000")
        .arg("--category")
        .arg("Food")
        .arg("--account")
        .arg("Cash");
    add_cmd1.assert().success();

    let mut add_cmd2 = cargo_bin_cmd!();
    add_cmd2.env("HOME", temp_dir.path());
    add_cmd2.env("XDG_CONFIG_HOME", temp_dir.path().join("config"));
    add_cmd2.env("XDG_DATA_HOME", temp_dir.path().join("data"));
    add_cmd2
        .arg("add")
        .arg("--date")
        .arg("2025-01-02")
        .arg("--amount")
        .arg("-2000")
        .arg("--category")
        .arg("Transport")
        .arg("--account")
        .arg("Bank"); // Use Bank instead of Card (default account)
    add_cmd2.assert().success();

    // List transactions
    let mut list_cmd = cargo_bin_cmd!();
    list_cmd.env("HOME", temp_dir.path());
    list_cmd.env("XDG_CONFIG_HOME", temp_dir.path().join("config"));
    list_cmd.env("XDG_DATA_HOME", temp_dir.path().join("data"));
    list_cmd.arg("list");

    list_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("Date"))
        .stdout(predicate::str::contains("Amount"))
        .stdout(predicate::str::contains("Category"))
        .stdout(predicate::str::contains("Account"))
        .stdout(predicate::str::contains("Food"))
        .stdout(predicate::str::contains("Transport"));
}

#[test]
fn test_transform_command_table() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Initialize database
    init_database(&temp_dir);

    // Add a transaction
    let mut add_cmd = cargo_bin_cmd!();
    add_cmd.env("HOME", temp_dir.path());
    add_cmd.env("XDG_CONFIG_HOME", temp_dir.path().join("config"));
    add_cmd.env("XDG_DATA_HOME", temp_dir.path().join("data"));
    add_cmd
        .arg("add")
        .arg("--date")
        .arg("2025-01-01")
        .arg("--amount")
        .arg("-1000")
        .arg("--category")
        .arg("Food")
        .arg("--account")
        .arg("Cash");
    add_cmd.assert().success();

    // Transform with "table" program
    let mut transform_cmd = cargo_bin_cmd!();
    transform_cmd.env("HOME", temp_dir.path());
    transform_cmd.env("XDG_CONFIG_HOME", temp_dir.path().join("config"));
    transform_cmd.env("XDG_DATA_HOME", temp_dir.path().join("data"));
    transform_cmd.arg("transform").arg("--program").arg("table");

    transform_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("Food"))
        .stdout(predicate::str::contains("Cash"));
}

#[test]
fn test_transform_command_empty_table() {
    let (mut cmd, temp_dir) = setup_test_cmd();

    // Initialize database
    init_database(&temp_dir);

    // Transform with "table" program (empty table)
    cmd.arg("transform").arg("--program").arg("table");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("No transactions found"));
}

#[test]
fn test_transform_command_group_by() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Initialize database
    init_database(&temp_dir);

    // Add multiple transactions in different categories
    let mut add_cmd1 = cargo_bin_cmd!();
    add_cmd1.env("HOME", temp_dir.path());
    add_cmd1.env("XDG_CONFIG_HOME", temp_dir.path().join("config"));
    add_cmd1.env("XDG_DATA_HOME", temp_dir.path().join("data"));
    add_cmd1
        .arg("add")
        .arg("--date")
        .arg("2025-01-01")
        .arg("--amount")
        .arg("-1000")
        .arg("--category")
        .arg("Food")
        .arg("--account")
        .arg("Cash");
    add_cmd1.assert().success();

    let mut add_cmd2 = cargo_bin_cmd!();
    add_cmd2.env("HOME", temp_dir.path());
    add_cmd2.env("XDG_CONFIG_HOME", temp_dir.path().join("config"));
    add_cmd2.env("XDG_DATA_HOME", temp_dir.path().join("data"));
    add_cmd2
        .arg("add")
        .arg("--date")
        .arg("2025-01-02")
        .arg("--amount")
        .arg("-1500")
        .arg("--category")
        .arg("Food")
        .arg("--account")
        .arg("Bank"); // Use Bank instead of Card (default account)
    add_cmd2.assert().success();

    let mut add_cmd3 = cargo_bin_cmd!();
    add_cmd3.env("HOME", temp_dir.path());
    add_cmd3.env("XDG_CONFIG_HOME", temp_dir.path().join("config"));
    add_cmd3.env("XDG_DATA_HOME", temp_dir.path().join("data"));
    add_cmd3
        .arg("add")
        .arg("--date")
        .arg("2025-01-03")
        .arg("--amount")
        .arg("-2000")
        .arg("--category")
        .arg("Transport")
        .arg("--account")
        .arg("Cash");
    add_cmd3.assert().success();

    // Transform with group-by program
    let mut transform_cmd = cargo_bin_cmd!();
    transform_cmd.env("HOME", temp_dir.path());
    transform_cmd.env("XDG_CONFIG_HOME", temp_dir.path().join("config"));
    transform_cmd.env("XDG_DATA_HOME", temp_dir.path().join("data"));
    transform_cmd
        .arg("transform")
        .arg("--program")
        .arg("(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))");

    transform_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("=== Food ==="))
        .stdout(predicate::str::contains("=== Transport ==="));
}

#[test]
fn test_transform_command_invalid_program() {
    let (mut cmd, temp_dir) = setup_test_cmd();

    // Initialize database
    init_database(&temp_dir);

    // Transform with invalid program
    cmd.arg("transform").arg("--program").arg("(invalid syntax");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to transform transactions"));
}

#[test]
fn test_add_command_missing_required_args() {
    let (mut cmd, temp_dir) = setup_test_cmd();

    // Initialize database
    init_database(&temp_dir);

    // Try to add without required arguments
    cmd.arg("add").arg("--date").arg("2025-01-01");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("required"));
}

#[test]
fn test_add_command_invalid_date_format() {
    let (mut cmd, temp_dir) = setup_test_cmd();

    // Initialize database
    init_database(&temp_dir);

    // Try to add with invalid date format
    cmd.arg("add")
        .arg("--date")
        .arg("01/01/2025") // Wrong format, should be YYYY-MM-DD
        .arg("--amount")
        .arg("-1000")
        .arg("--category")
        .arg("Food")
        .arg("--account")
        .arg("Cash");

    cmd.assert().failure();
}

#[test]
fn test_init_command_idempotent() {
    let (mut cmd, temp_dir) = setup_test_cmd();

    // Initialize database first time
    init_database(&temp_dir);

    // Initialize again (should succeed)
    cmd.arg("init");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Initialization complete"));
}

#[test]
fn test_list_command_shows_formatted_table() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Initialize database
    init_database(&temp_dir);

    // Add a transaction
    let mut add_cmd = cargo_bin_cmd!();
    add_cmd.env("HOME", temp_dir.path());
    add_cmd.env("XDG_CONFIG_HOME", temp_dir.path().join("config"));
    add_cmd.env("XDG_DATA_HOME", temp_dir.path().join("data"));
    add_cmd
        .arg("add")
        .arg("--date")
        .arg("2025-01-15")
        .arg("--amount")
        .arg("-1000")
        .arg("--category")
        .arg("Food")
        .arg("--account")
        .arg("Cash")
        .arg("--memo")
        .arg("Test memo");
    add_cmd.assert().success();

    // List transactions
    let mut list_cmd = cargo_bin_cmd!();
    list_cmd.env("HOME", temp_dir.path());
    list_cmd.env("XDG_CONFIG_HOME", temp_dir.path().join("config"));
    list_cmd.env("XDG_DATA_HOME", temp_dir.path().join("data"));
    list_cmd.arg("list");

    list_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("2025-01-15"))
        .stdout(predicate::str::contains("¥-1000"))
        .stdout(predicate::str::contains("Test memo"))
        .stdout(predicate::str::contains("╭")) // Table border character
        .stdout(predicate::str::contains("╰")); // Table border character
}

#[test]
fn test_transform_with_car_operation() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Initialize database
    init_database(&temp_dir);

    // Add multiple transactions
    let mut add_cmd1 = cargo_bin_cmd!();
    add_cmd1.env("HOME", temp_dir.path());
    add_cmd1.env("XDG_CONFIG_HOME", temp_dir.path().join("config"));
    add_cmd1.env("XDG_DATA_HOME", temp_dir.path().join("data"));
    add_cmd1
        .arg("add")
        .arg("--date")
        .arg("2025-01-01")
        .arg("--amount")
        .arg("-1000")
        .arg("--category")
        .arg("Food")
        .arg("--account")
        .arg("Cash");
    add_cmd1.assert().success();

    let mut add_cmd2 = cargo_bin_cmd!();
    add_cmd2.env("HOME", temp_dir.path());
    add_cmd2.env("XDG_CONFIG_HOME", temp_dir.path().join("config"));
    add_cmd2.env("XDG_DATA_HOME", temp_dir.path().join("data"));
    add_cmd2
        .arg("add")
        .arg("--date")
        .arg("2025-01-02")
        .arg("--amount")
        .arg("-2000")
        .arg("--category")
        .arg("Transport")
        .arg("--account")
        .arg("Bank"); // Use Bank instead of Card (default account)
    add_cmd2.assert().success();

    // Get only first transaction using (cons (car table) ())
    // Note: transactions are returned newest first, so car gets the Transport transaction
    let mut transform_cmd = cargo_bin_cmd!();
    transform_cmd.env("HOME", temp_dir.path());
    transform_cmd.env("XDG_CONFIG_HOME", temp_dir.path().join("config"));
    transform_cmd.env("XDG_DATA_HOME", temp_dir.path().join("data"));
    transform_cmd
        .arg("transform")
        .arg("--program")
        .arg("(cons (car table) ())");

    transform_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("Transport"))
        .stdout(predicate::str::contains("2025-01-02"));
}

#[test]
fn test_transform_with_cdr_operation() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");

    // Initialize database
    init_database(&temp_dir);

    // Add multiple transactions
    let mut add_cmd1 = cargo_bin_cmd!();
    add_cmd1.env("HOME", temp_dir.path());
    add_cmd1.env("XDG_CONFIG_HOME", temp_dir.path().join("config"));
    add_cmd1.env("XDG_DATA_HOME", temp_dir.path().join("data"));
    add_cmd1
        .arg("add")
        .arg("--date")
        .arg("2025-01-01")
        .arg("--amount")
        .arg("-1000")
        .arg("--category")
        .arg("Food")
        .arg("--account")
        .arg("Cash");
    add_cmd1.assert().success();

    let mut add_cmd2 = cargo_bin_cmd!();
    add_cmd2.env("HOME", temp_dir.path());
    add_cmd2.env("XDG_CONFIG_HOME", temp_dir.path().join("config"));
    add_cmd2.env("XDG_DATA_HOME", temp_dir.path().join("data"));
    add_cmd2
        .arg("add")
        .arg("--date")
        .arg("2025-01-02")
        .arg("--amount")
        .arg("-2000")
        .arg("--category")
        .arg("Transport")
        .arg("--account")
        .arg("Bank"); // Use Bank instead of Card (default account)
    add_cmd2.assert().success();

    // Skip first transaction using (cdr table)
    // Note: transactions are returned newest first, so cdr skips Transport and returns Food
    let mut transform_cmd = cargo_bin_cmd!();
    transform_cmd.env("HOME", temp_dir.path());
    transform_cmd.env("XDG_CONFIG_HOME", temp_dir.path().join("config"));
    transform_cmd.env("XDG_DATA_HOME", temp_dir.path().join("data"));
    transform_cmd
        .arg("transform")
        .arg("--program")
        .arg("(cdr table)");

    transform_cmd
        .assert()
        .success()
        .stdout(predicate::str::contains("Food"))
        .stdout(predicate::str::contains("2025-01-01"));
}
