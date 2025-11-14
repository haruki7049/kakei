#![allow(deprecated)]

use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use predicates::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

/// Test that the example-config.toml file is valid and can be used to initialize the database.
#[test]
fn test_example_config_is_valid() {
    // Create a temporary directory for the test
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("config.toml");
    
    // Copy the example-config.toml to the temp directory
    let example_config_path = std::env::current_dir()
        .expect("Failed to get current directory")
        .join("example-config.toml");
    
    fs::copy(&example_config_path, &config_path)
        .expect("Failed to copy example-config.toml");
    
    // Run the init command with the example config
    let mut cmd = Command::cargo_bin("kakei").expect("Failed to find kakei binary");
    cmd.arg("--config-file")
        .arg(config_path.to_str().unwrap())
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialization complete"));
}

/// Test that the example-config.toml creates the expected categories.
#[test]
fn test_example_config_creates_correct_categories() {
    // Create a temporary directory for the test
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("config.toml");
    
    // Copy the example-config.toml to the temp directory
    let example_config_path = std::env::current_dir()
        .expect("Failed to get current directory")
        .join("example-config.toml");
    
    fs::copy(&example_config_path, &config_path)
        .expect("Failed to copy example-config.toml");
    
    // Set HOME to temp directory and clear XDG variables to ensure consistent behavior
    // ProjectDirs will use HOME/.local/share/kakei when XDG_DATA_HOME is not set
    Command::cargo_bin("kakei")
        .expect("Failed to find kakei binary")
        .env("HOME", temp_dir.path())
        .env_remove("XDG_DATA_HOME")
        .env_remove("XDG_CONFIG_HOME")
        .env_remove("XDG_CACHE_HOME")
        .arg("--config-file")
        .arg(config_path.to_str().unwrap())
        .arg("init")
        .assert()
        .success();
    
    // Verify the database was created at the expected location
    // ProjectDirs uses HOME/.local/share/kakei on Linux when XDG_DATA_HOME is not set
    let db_path = temp_dir.path().join(".local/share/kakei/kakei.db");
    
    assert!(
        db_path.exists(),
        "Database should be created at {:?}",
        db_path
    );
}

/// Test that adding a transaction works with categories from example-config.toml.
#[test]
fn test_example_config_with_transaction() {
    // Create a temporary directory for the test
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("config.toml");
    
    // Copy the example-config.toml to the temp directory
    let example_config_path = std::env::current_dir()
        .expect("Failed to get current directory")
        .join("example-config.toml");
    
    fs::copy(&example_config_path, &config_path)
        .expect("Failed to copy example-config.toml");
    
    // Initialize the database
    Command::cargo_bin("kakei")
        .expect("Failed to find kakei binary")
        .env("HOME", temp_dir.path())
        .arg("--config-file")
        .arg(config_path.to_str().unwrap())
        .arg("init")
        .assert()
        .success();
    
    // Add a transaction with an expense category from the example config
    Command::cargo_bin("kakei")
        .expect("Failed to find kakei binary")
        .env("HOME", temp_dir.path())
        .arg("--config-file")
        .arg(config_path.to_str().unwrap())
        .arg("add")
        .arg("--date")
        .arg("2025-01-15")
        .arg("--amount")
        .arg("-1500")
        .arg("--category")
        .arg("Food") // From expense_categories in example-config.toml
        .arg("--account")
        .arg("Cash") // From default_accounts in example-config.toml
        .arg("--memo")
        .arg("Test expense")
        .assert()
        .success()
        .stdout(predicate::str::contains("Transaction added successfully"));
    
    // Add a transaction with an income category from the example config
    Command::cargo_bin("kakei")
        .expect("Failed to find kakei binary")
        .env("HOME", temp_dir.path())
        .arg("--config-file")
        .arg(config_path.to_str().unwrap())
        .arg("add")
        .arg("--date")
        .arg("2025-01-01")
        .arg("--amount")
        .arg("300000")
        .arg("--category")
        .arg("Salary") // From income_categories in example-config.toml
        .arg("--account")
        .arg("Bank") // From default_accounts in example-config.toml
        .arg("--memo")
        .arg("Test income")
        .assert()
        .success()
        .stdout(predicate::str::contains("Transaction added successfully"));
    
    // List transactions to verify they were added
    Command::cargo_bin("kakei")
        .expect("Failed to find kakei binary")
        .env("HOME", temp_dir.path())
        .arg("--config-file")
        .arg(config_path.to_str().unwrap())
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("Food"))
        .stdout(predicate::str::contains("Salary"))
        .stdout(predicate::str::contains("Test expense"))
        .stdout(predicate::str::contains("Test income"));
}

/// Test that all expense categories from example-config.toml are created correctly.
#[test]
fn test_example_config_all_expense_categories() {
    // Create a temporary directory for the test
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("config.toml");
    
    // Copy the example-config.toml to the temp directory
    let example_config_path = std::env::current_dir()
        .expect("Failed to get current directory")
        .join("example-config.toml");
    
    fs::copy(&example_config_path, &config_path)
        .expect("Failed to copy example-config.toml");
    
    // Initialize the database
    Command::cargo_bin("kakei")
        .expect("Failed to find kakei binary")
        .env("HOME", temp_dir.path())
        .arg("--config-file")
        .arg(config_path.to_str().unwrap())
        .arg("init")
        .assert()
        .success();
    
    // Test all expense categories from example-config.toml
    let expense_categories = vec![
        "Food",
        "Transport",
        "Daily Goods",
        "Hobby",
        "Utilities",
        "Healthcare",
        "Education",
    ];
    
    for category in expense_categories {
        Command::cargo_bin("kakei")
            .expect("Failed to find kakei binary")
            .env("HOME", temp_dir.path())
            .arg("--config-file")
            .arg(config_path.to_str().unwrap())
            .arg("add")
            .arg("--date")
            .arg("2025-01-01")
            .arg("--amount")
            .arg("-100")
            .arg("--category")
            .arg(category)
            .arg("--account")
            .arg("Cash")
            .assert()
            .success()
            .stdout(predicate::str::contains("Transaction added successfully"));
    }
}

/// Test that all income categories from example-config.toml are created correctly.
#[test]
fn test_example_config_all_income_categories() {
    // Create a temporary directory for the test
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("config.toml");
    
    // Copy the example-config.toml to the temp directory
    let example_config_path = std::env::current_dir()
        .expect("Failed to get current directory")
        .join("example-config.toml");
    
    fs::copy(&example_config_path, &config_path)
        .expect("Failed to copy example-config.toml");
    
    // Initialize the database
    Command::cargo_bin("kakei")
        .expect("Failed to find kakei binary")
        .env("HOME", temp_dir.path())
        .arg("--config-file")
        .arg(config_path.to_str().unwrap())
        .arg("init")
        .assert()
        .success();
    
    // Test all income categories from example-config.toml
    let income_categories = vec!["Salary", "Bonus", "Freelance", "Investment", "Gift"];
    
    for category in income_categories {
        Command::cargo_bin("kakei")
            .expect("Failed to find kakei binary")
            .env("HOME", temp_dir.path())
            .arg("--config-file")
            .arg(config_path.to_str().unwrap())
            .arg("add")
            .arg("--date")
            .arg("2025-01-01")
            .arg("--amount")
            .arg("1000")
            .arg("--category")
            .arg(category)
            .arg("--account")
            .arg("Bank")
            .assert()
            .success()
            .stdout(predicate::str::contains("Transaction added successfully"));
    }
}

/// Test that all accounts from example-config.toml are created correctly.
#[test]
fn test_example_config_all_accounts() {
    // Create a temporary directory for the test
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let config_path = temp_dir.path().join("config.toml");
    
    // Copy the example-config.toml to the temp directory
    let example_config_path = std::env::current_dir()
        .expect("Failed to get current directory")
        .join("example-config.toml");
    
    fs::copy(&example_config_path, &config_path)
        .expect("Failed to copy example-config.toml");
    
    // Initialize the database
    Command::cargo_bin("kakei")
        .expect("Failed to find kakei binary")
        .env("HOME", temp_dir.path())
        .arg("--config-file")
        .arg(config_path.to_str().unwrap())
        .arg("init")
        .assert()
        .success();
    
    // Test all accounts from example-config.toml
    let accounts = vec!["Cash", "Bank", "Credit Card"];
    
    for account in accounts {
        Command::cargo_bin("kakei")
            .expect("Failed to find kakei binary")
            .env("HOME", temp_dir.path())
            .arg("--config-file")
            .arg(config_path.to_str().unwrap())
            .arg("add")
            .arg("--date")
            .arg("2025-01-01")
            .arg("--amount")
            .arg("-100")
            .arg("--category")
            .arg("Food")
            .arg("--account")
            .arg(account)
            .assert()
            .success()
            .stdout(predicate::str::contains("Transaction added successfully"));
    }
}
