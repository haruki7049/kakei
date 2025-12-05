//! Kakei configurations.

use serde::{Deserialize, Serialize};

/// Configuration for the Kakei application.
/// Saved in ~/.config/kakei/default.kakei (typically).
#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    /// List of default categories to create during initialization.
    pub default_categories: Vec<String>,

    /// List of default accounts to create during initialization.
    pub default_accounts: Vec<String>,
}

/// Provides default values for the configuration.
impl Default for Configuration {
    fn default() -> Self {
        Self {
            default_categories: vec![
                "Food".to_string(),
                "Transport".to_string(),
                "Daily Goods".to_string(),
                "Hobby".to_string(),
                "Salary".to_string(), // Income category example
            ],
            default_accounts: vec!["Cash".to_string(), "Bank".to_string()],
        }
    }
}
