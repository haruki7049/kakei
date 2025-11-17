use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use std::{
    path::PathBuf,
    sync::{LazyLock, Mutex},
};

/// A CLI expense tracker for programmers.
#[derive(Parser, Debug)]
#[command(version, author, about)]
pub struct CLIArgs {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value = DEFAULT_CONFIG_PATH.lock().unwrap().display().to_string())]
    config_file: PathBuf,
}

impl CLIArgs {
    /// Returns a reference to the subcommand to execute.
    pub fn command(&self) -> &Commands {
        &self.command
    }

    /// Returns a reference to the path to the configuration file.
    pub fn config_file(&self) -> &PathBuf {
        &self.config_file
    }
}

/// Available subcommands for the kakei application.
#[derive(Subcommand, Debug, Clone, PartialEq, Eq)]
pub enum Commands {
    /// Add a new transaction.
    ///
    /// Examples:
    ///   # Using command-line arguments
    ///   kakei add --date 2025-01-01 --amount -1000 --category Food --account Cash
    ///
    ///   # Using text editor (like git commit)
    ///   kakei add --edit
    Add {
        /// Open text editor to input transaction (like git commit).
        #[arg(long, short = 'e')]
        edit: bool,

        /// Date of the transaction (YYYY-MM-DD).
        /// Required unless --edit is used.
        #[arg(long, required_unless_present = "edit")]
        date: Option<String>,

        /// Amount of the transaction.
        /// Use negative for expense (e.g. -1000), positive for income.
        /// Required unless --edit is used.
        #[arg(long, allow_hyphen_values = true, required_unless_present = "edit")]
        amount: Option<String>,

        /// Currency code (e.g., JPY, USD).
        #[arg(long, default_value = "JPY")]
        currency: String,

        /// Category name (e.g., Food, Salary).
        /// Required unless --edit is used.
        #[arg(long, required_unless_present = "edit")]
        category: Option<String>,

        /// Account name (e.g., Cash, Bank).
        /// Required unless --edit is used.
        #[arg(long, required_unless_present = "edit")]
        account: Option<String>,

        /// Optional memo.
        #[arg(long)]
        memo: Option<String>,
    },

    /// Initialize configuration and database.
    Init,

    /// List recent transactions.
    List,

    /// Transform transactions using a Lisp program.
    ///
    /// Examples:
    ///   # View all transactions
    ///   kakei transform --program "table"
    ///
    ///   # Group by category
    ///   kakei transform --program "(group-by table (lambda (pair) (cdr (assoc 'category (cdr pair)))))"
    Transform {
        /// Lisp program to transform the table.
        #[arg(long)]
        program: String,
    },
}

/// Default Configuration Path, using directories crate to calculate ProjectDirs (~/.config/kakei)
static DEFAULT_CONFIG_PATH: LazyLock<Mutex<PathBuf>> = LazyLock::new(|| {
    let proj_dirs = ProjectDirs::from("dev", "haruki7049", "kakei")
        .expect("Failed to search ProjectDirs for dev.haruki7049.kakei");
    let mut config_path: PathBuf = proj_dirs.config_dir().to_path_buf();
    let filename: &str = "config.toml";

    config_path.push(filename);
    Mutex::new(config_path)
});
