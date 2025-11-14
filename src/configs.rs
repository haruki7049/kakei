//! Kakei configurations.

use serde::{Deserialize, Serialize};

/// Configuration for the Kakei application.
/// Saved in ~/.config/kakei/config.toml (typically).
#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    /// List of expense categories to create during initialization.
    pub expense_categories: Vec<String>,

    /// List of income categories to create during initialization.
    pub income_categories: Vec<String>,

    /// List of default accounts to create during initialization.
    pub default_accounts: Vec<String>,
}

/// Provides default values for the configuration.
impl Default for Configuration {
    fn default() -> Self {
        Self {
            expense_categories: vec![
                "Food".to_string(),
                "Transport".to_string(),
                "Daily Goods".to_string(),
                "Hobby".to_string(),
            ],
            income_categories: vec!["Salary".to_string(), "Bonus".to_string()],
            default_accounts: vec!["Cash".to_string(), "Bank".to_string()],
        }
    }
}
