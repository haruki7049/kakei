use clap::Parser;
use directories::ProjectDirs;
use kakei::{
    cli::{CLIArgs, Commands},
    configs::Configuration,
};
use kakei_processor::Processor;
use std::path::{Path, PathBuf};
use tracing::{debug, error, info};
use tracing_subscriber::filter::EnvFilter;

// Use tokio for async runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CLIArgs = CLIArgs::parse();

    // Initialize tracing by tracing-subscriber
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    info!("Starting kakei application");
    debug!("Using config file: {:?}", args.config_file());

    let config: Configuration = confy::load_path(args.config_file()).unwrap_or_else(|e| {
        debug!(
            "Failed to load configuration: {}. Using default Configuration",
            e
        );
        Configuration::default()
    });

    // 1. Determine the database file path
    // Uses XDG directory standard (e.g. ~/.local/share/kakei/kakei.db on Linux)
    let project_dirs: ProjectDirs = ProjectDirs::from("dev", "haruki7049", "kakei")
        .ok_or("Could not determine project directories")?;
    let data_dir: &Path = project_dirs.data_dir();
    debug!("Data directory: {:?}", data_dir);

    // Create the data directory if it doesn't exist
    if !data_dir.exists() {
        info!("Creating data directory: {:?}", data_dir);
        std::fs::create_dir_all(data_dir)?;
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
            info!("🚀 Adding transaction...");

            // Call the business logic in processor crate
            match processor
                .add_transaction(&date, &amount, &currency, &category, &account, memo)
                .await
            {
                Ok(tx_id) => {
                    info!("Transaction added successfully! (ID: {:?})", tx_id);
                }
                Err(e) => {
                    error!("Failed to add transaction: {}", e);
                    return Err(e.into());
                }
            }
        }
        Commands::Init => {
            info!("🔧 Initializing database with default data...");
            match processor
                .init_master_data(&config.default_categories, &config.default_accounts)
                .await
            {
                Ok(_) => {
                    info!(
                        "✅ Initialization complete. Database ready at: {}",
                        db_path_str
                    );
                }
                Err(e) => {
                    error!("Failed to initialize database: {}", e);
                    return Err(e.into());
                }
            }
        }
        Commands::List => {
            info!("📋 Recent Transactions:");
            info!(
                "--------------------------------------------------------------------------------"
            );

            match processor.get_recent_transactions().await {
                Ok(transactions) => {
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
                Err(e) => {
                    error!("Failed to retrieve transactions: {}", e);
                    return Err(e.into());
                }
            }
        }
    }

    info!("Application completed successfully");
    Ok(())
}
