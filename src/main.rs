//! Kakei - A kakeibo (household financial ledger) CLI application.
//!
//! This is the main entry point for the kakei application, which provides
//! a command-line interface for managing personal finances using the Japanese
//! kakeibo method.

use clap::Parser;
use directories::ProjectDirs;
use kakei::{
    cli::{CLIArgs, Commands},
    configs::Configuration,
};
use kakei_processor::Processor;
use std::path::{Path, PathBuf};
use tabled::{Table, Tabled, settings::Style};
use tracing::debug;
use tracing_subscriber::filter::EnvFilter;

/// Display struct for transactions in table format
#[derive(Tabled)]
struct TransactionDisplay {
    #[tabled(rename = "Date")]
    date: String,
    #[tabled(rename = "Amount")]
    amount: String,
    #[tabled(rename = "Category")]
    category: String,
    #[tabled(rename = "Account")]
    account: String,
    #[tabled(rename = "Memo")]
    memo: String,
}

impl TransactionDisplay {
    /// Create a TransactionDisplay from a DisplayRow
    fn from_display_row(row: kakei_processor::DisplayRow) -> Self {
        Self {
            date: row.date,
            amount: row.amount,
            category: row.category,
            account: row.account,
            memo: row.memo,
        }
    }
}

/// Handle the Add command
async fn handle_add_command(
    processor: &Processor,
    date: &str,
    amount: &str,
    currency: &str,
    category: &str,
    account: &str,
    memo: &Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    match processor
        .add_transaction(date, amount, currency, category, account, memo.clone())
        .await
    {
        Ok(tx_id) => {
            println!("✅ Transaction added successfully! (ID: {:?})", tx_id);
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ Failed to add transaction: {}", e);
            Err(e.into())
        }
    }
}

/// Handle the Init command
async fn handle_init_command(
    processor: &Processor,
    config: &Configuration,
    db_path_str: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match processor
        .init_master_data(&config.default_categories, &config.default_accounts)
        .await
    {
        Ok(_) => {
            println!(
                "✅ Initialization complete. Database ready at: {}",
                db_path_str
            );
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ Failed to initialize database: {}", e);
            Err(e.into())
        }
    }
}

/// Handle the List command
async fn handle_list_command(processor: &Processor) -> Result<(), Box<dyn std::error::Error>> {
    match processor.get_recent_transactions().await {
        Ok(transactions) => {
            if transactions.is_empty() {
                println!("No transactions found.");
            } else {
                // Convert transactions to display format
                let display_data: Vec<TransactionDisplay> = transactions
                    .into_iter()
                    .map(|tx| TransactionDisplay {
                        date: tx.date.to_string(),
                        amount: tx.amount.to_string(),
                        category: tx.category_name,
                        account: tx.account_name,
                        memo: tx.memo.unwrap_or_default(),
                    })
                    .collect();

                display_table(display_data);
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ Failed to retrieve transactions: {}", e);
            Err(e.into())
        }
    }
}

/// Display a table of transactions
fn display_table(display_data: Vec<TransactionDisplay>) {
    let mut table = Table::new(display_data);
    table.with(Style::rounded());
    println!("{}", table);
}

/// Handle the Transform command
async fn handle_transform_command(
    processor: &Processor,
    program: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match processor.transform_transactions(program).await {
        Ok(result) => {
            // The result is always in grouped format
            let grouped_tables = kakei_processor::value_to_grouped_tables(&result)?;

            for group in grouped_tables {
                println!("\n=== {} ===", group.group_name);

                if group.rows.is_empty() {
                    println!("No transactions in this group.");
                } else {
                    let display_data: Vec<TransactionDisplay> = group
                        .rows
                        .into_iter()
                        .map(TransactionDisplay::from_display_row)
                        .collect();

                    display_table(display_data);
                }
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ Failed to transform transactions: {}", e);
            Err(e.into())
        }
    }
}

// Use tokio for async runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CLIArgs = CLIArgs::parse();

    // Initialize tracing for internal logging only
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn")),
        )
        .init();

    let config: Configuration = confy::load_path(args.config_file()).unwrap_or_default();

    // 1. Determine the database file path
    // Uses XDG directory standard (e.g. ~/.local/share/kakei/kakei.db on Linux)
    let project_dirs: ProjectDirs = ProjectDirs::from("dev", "haruki7049", "kakei")
        .ok_or("Could not determine project directories")?;
    let data_dir: &Path = project_dirs.data_dir();
    debug!("Data directory: {:?}", data_dir);

    // Create the data directory if it doesn't exist
    if !data_dir.exists() {
        std::fs::create_dir_all(data_dir)?;
        debug!("Creating data directory: {:?}", data_dir);
    }

    let db_path: PathBuf = data_dir.join("kakei.db");
    let db_path_str: &str = db_path.to_str().ok_or("Invalid database path")?;
    debug!("Database path: {}", db_path_str);

    // 2. Initialize the Processor
    // This establishes the DB connection and runs migrations if needed.
    let processor: Processor = Processor::new(db_path_str).await?;

    // 3. Dispatch commands
    match args.command() {
        Commands::Add {
            date,
            amount,
            currency,
            category,
            account,
            memo,
        } => {
            handle_add_command(&processor, date, amount, currency, category, account, memo).await?
        }
        Commands::Init => handle_init_command(&processor, &config, db_path_str).await?,
        Commands::List => handle_list_command(&processor).await?,
        Commands::Transform { program } => handle_transform_command(&processor, program).await?,
    }

    Ok(())
}
