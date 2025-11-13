use clap::Parser;
use directories::ProjectDirs;
use kakei::{
    cli::{CLIArgs, Commands},
    configs::Configuration,
};
use kakei_processor::Processor;
use std::path::{Path, PathBuf};
use tracing::{Level, info, debug};
use tracing_subscriber::filter::EnvFilter;

// Use tokio for async runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CLIArgs = CLIArgs::parse();

    // Initialize tracing by tracing-subscriber
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(Level::INFO)
        .init();

    let config: Configuration = confy::load_path(args.config_file()).unwrap_or_else(|_| {
        debug!("Running kakei with default Configuration...");
        Configuration::default()
    });

    // 1. Determine the database file path
    // Uses XDG directory standard (e.g. ~/.local/share/kakei/kakei.db on Linux)
    let project_dirs: ProjectDirs = ProjectDirs::from("dev", "haruki7049", "kakei")
        .ok_or("Could not determine project directories")?;
    let data_dir: &Path = project_dirs.data_dir();

    // Create the data directory if it doesn't exist
    if !data_dir.exists() {
        std::fs::create_dir_all(data_dir)?;
    }

    let db_path: PathBuf = data_dir.join("kakei.db");
    let db_path_str: &str = db_path.to_str().ok_or("Invalid database path")?;

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
            info!("🚀 Adding transaction...");

            // Call the business logic in processor crate
            let tx_id = processor
                .add_transaction(&date, &amount, &currency, &category, &account, memo)
                .await?;

            info!("Transaction added successfully! (ID: {:?})", tx_id);
        }
        Commands::Init => {
            info!("🔧 Initializing database with default data...");
            processor
                .init_master_data(&config.default_categories, &config.default_accounts)
                .await?;

            info!(
                "✅ Initialization complete. Database ready at: {}",
                db_path_str
            );
        }
        Commands::List => {
            info!("📋 Recent Transactions:");
            info!(
                "--------------------------------------------------------------------------------"
            );

            let transactions = processor.get_recent_transactions().await?;

            if transactions.is_empty() {
                info!("No transactions found.");
            } else {
                for tx in transactions {
                    // Simple formatting
                    info!(
                        "{: <12} | {: >15} | {: <10} | {: <10} | {}",
                        tx.date,
                        tx.amount, // Money implements Display (e.g. ¥-1000)
                        tx.category_name,
                        tx.account_name,
                        tx.memo.unwrap_or_default()
                    );
                }
            }
            info!(
                "--------------------------------------------------------------------------------"
            );
        }
    }

    Ok(())
}
