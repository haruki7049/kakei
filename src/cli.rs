use clap::{Parser, Subcommand};

/// A CLI expense tracker for programmers.
#[derive(Parser, Debug)]
#[command(version, author, about)]
pub struct CLIArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Add a new transaction.
    /// Example: kakei add --date 2025-01-01 --amount -1000 --category Food --account Cash
    Add {
        /// Date of the transaction (YYYY-MM-DD).
        #[arg(long)]
        date: String,

        /// Amount of the transaction.
        /// Use negative for expense (e.g. -1000), positive for income.
        #[arg(long, allow_hyphen_values = true)]
        amount: String,

        /// Currency code (e.g., JPY, USD).
        #[arg(long, default_value = "JPY")]
        currency: String,

        /// Category name (e.g., Food, Salary).
        #[arg(long)]
        category: String,

        /// Account name (e.g., Cash, Bank).
        #[arg(long)]
        account: String,

        /// Optional memo.
        #[arg(long)]
        memo: Option<String>,
    },

    /// Initialize configuration and database.
    Init,

    /// (Future) List recent transactions.
    List,
}
