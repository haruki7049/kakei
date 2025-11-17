//! Editor support for transaction input.
//!
//! This module provides functionality to open a text editor for inputting
//! transaction details, similar to how Git opens an editor for commit messages.

use std::error::Error;
use std::fmt;

/// Represents a parsed transaction from the editor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorTransaction {
    pub date: String,
    pub amount: String,
    pub currency: String,
    pub category: String,
    pub account: String,
    pub memo: Option<String>,
}

/// Error type for editor-related operations.
#[derive(Debug)]
pub enum EditorError {
    /// Error opening or using the editor.
    EditorFailed(String),
    /// Error parsing the transaction template.
    ParseError(String),
    /// Transaction was cancelled (empty or only comments).
    Cancelled,
}

impl fmt::Display for EditorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EditorError::EditorFailed(msg) => write!(f, "Editor error: {}", msg),
            EditorError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            EditorError::Cancelled => write!(f, "Transaction cancelled"),
        }
    }
}

impl Error for EditorError {}

/// Generate a transaction template for the editor.
fn generate_template(currency: &str) -> String {
    format!(
        r#"# Enter transaction details below.
# Lines starting with '#' are comments and will be ignored.
# Format: key: value
#
# Example expense:
#   date: 2025-01-15
#   amount: -1000
#   currency: JPY
#   category: Food
#   account: Cash
#   memo: Lunch at restaurant
#
# Example income:
#   date: 2025-01-15
#   amount: 50000
#   currency: JPY
#   category: Salary
#   account: Bank
#   memo: Monthly salary
#
# Delete this template and fill in your transaction:

date: 
amount: 
currency: {}
category: 
account: 
memo: 
"#,
        currency
    )
}

/// Parse the edited template into a transaction.
fn parse_template(content: &str) -> Result<EditorTransaction, EditorError> {
    let mut date = None;
    let mut amount = None;
    let mut currency = None;
    let mut category = None;
    let mut account = None;
    let mut memo = None;

    let mut has_content = false;

    for line in content.lines() {
        let line = line.trim();
        
        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        has_content = true;

        // Parse key: value pairs
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim().to_lowercase();
            let value = value.trim();

            match key.as_str() {
                "date" => {
                    if !value.is_empty() {
                        date = Some(value.to_string());
                    }
                }
                "amount" => {
                    if !value.is_empty() {
                        amount = Some(value.to_string());
                    }
                }
                "currency" => {
                    if !value.is_empty() {
                        currency = Some(value.to_string());
                    }
                }
                "category" => {
                    if !value.is_empty() {
                        category = Some(value.to_string());
                    }
                }
                "account" => {
                    if !value.is_empty() {
                        account = Some(value.to_string());
                    }
                }
                "memo" => {
                    if !value.is_empty() {
                        memo = Some(value.to_string());
                    }
                }
                _ => {
                    // Unknown key, skip or warn
                    eprintln!("Warning: Unknown field '{}' will be ignored", key);
                }
            }
        }
    }

    if !has_content {
        return Err(EditorError::Cancelled);
    }

    // Validate required fields
    let date = date.ok_or_else(|| EditorError::ParseError("date is required".to_string()))?;
    let amount = amount.ok_or_else(|| EditorError::ParseError("amount is required".to_string()))?;
    let currency = currency.ok_or_else(|| EditorError::ParseError("currency is required".to_string()))?;
    let category = category.ok_or_else(|| EditorError::ParseError("category is required".to_string()))?;
    let account = account.ok_or_else(|| EditorError::ParseError("account is required".to_string()))?;

    Ok(EditorTransaction {
        date,
        amount,
        currency,
        category,
        account,
        memo,
    })
}

/// Open a text editor to input a transaction.
///
/// This function opens the user's preferred text editor (using VISUAL or EDITOR
/// environment variables, or a platform-appropriate default) with a template
/// for entering transaction details.
///
/// Returns the parsed transaction or an error if the operation failed.
pub fn edit_transaction(default_currency: &str) -> Result<EditorTransaction, EditorError> {
    let template = generate_template(default_currency);
    
    let edited = edit::edit(&template)
        .map_err(|e| EditorError::EditorFailed(e.to_string()))?;
    
    parse_template(&edited)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_template_valid() {
        let content = r#"
# This is a comment
date: 2025-01-15
amount: -1000
currency: JPY
category: Food
account: Cash
memo: Test memo
"#;
        let result = parse_template(content).unwrap();
        assert_eq!(result.date, "2025-01-15");
        assert_eq!(result.amount, "-1000");
        assert_eq!(result.currency, "JPY");
        assert_eq!(result.category, "Food");
        assert_eq!(result.account, "Cash");
        assert_eq!(result.memo, Some("Test memo".to_string()));
    }

    #[test]
    fn test_parse_template_without_memo() {
        let content = r#"
date: 2025-01-15
amount: -1000
currency: JPY
category: Food
account: Cash
"#;
        let result = parse_template(content).unwrap();
        assert_eq!(result.date, "2025-01-15");
        assert_eq!(result.memo, None);
    }

    #[test]
    fn test_parse_template_empty_memo() {
        let content = r#"
date: 2025-01-15
amount: -1000
currency: JPY
category: Food
account: Cash
memo: 
"#;
        let result = parse_template(content).unwrap();
        assert_eq!(result.memo, None);
    }

    #[test]
    fn test_parse_template_missing_required_field() {
        let content = r#"
date: 2025-01-15
amount: -1000
currency: JPY
# Missing category
account: Cash
"#;
        let result = parse_template(content);
        assert!(result.is_err());
        assert!(matches!(result, Err(EditorError::ParseError(_))));
    }

    #[test]
    fn test_parse_template_only_comments() {
        let content = r#"
# Just a comment
# Another comment
"#;
        let result = parse_template(content);
        assert!(result.is_err());
        assert!(matches!(result, Err(EditorError::Cancelled)));
    }

    #[test]
    fn test_parse_template_empty() {
        let content = "";
        let result = parse_template(content);
        assert!(result.is_err());
        assert!(matches!(result, Err(EditorError::Cancelled)));
    }

    #[test]
    fn test_parse_template_with_spaces() {
        let content = r#"
  date  :  2025-01-15  
  amount  :  -1000  
  currency  :  JPY  
  category  :  Food  
  account  :  Cash  
"#;
        let result = parse_template(content).unwrap();
        assert_eq!(result.date, "2025-01-15");
        assert_eq!(result.amount, "-1000");
    }

    #[test]
    fn test_parse_template_unknown_field() {
        let content = r#"
date: 2025-01-15
amount: -1000
currency: JPY
category: Food
account: Cash
unknown: value
"#;
        // Should succeed but ignore unknown field
        let result = parse_template(content);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_template() {
        let template = generate_template("USD");
        assert!(template.contains("currency: USD"));
        assert!(template.contains("date: "));
        assert!(template.contains("amount: "));
    }
}
