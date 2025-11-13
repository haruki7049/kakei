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
    println!("📋 Recent Transactions:");
    println!("--------------------------------------------------------------------------------");

    match processor.get_recent_transactions().await {
        Ok(transactions) => {
            if transactions.is_empty() {
                println!("No transactions found.");
            } else {
                for tx in transactions {
                    // Simple formatting
                    println!(
                        "{: <12} | {: >15} | {: <10} | {: <10} | {}",
                        tx.date,
                        tx.amount, // Money implements Display (e.g. ¥-1000)
                        tx.category_name,
                        tx.account_name,
                        tx.memo.unwrap_or_default()
                    );
                }
            }
            println!(
                "--------------------------------------------------------------------------------"
            );
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ Failed to retrieve transactions: {}", e);
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
            handle_add_command(
                &processor, &date, &amount, &currency, &category, &account, &memo,
            )
            .await?
        }
        Commands::Init => handle_init_command(&processor, &config, db_path_str).await?,
        Commands::List => handle_list_command(&processor).await?,
    }

    Ok(())
}
