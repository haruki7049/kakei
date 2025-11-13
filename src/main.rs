use clap::Parser;
use directories::ProjectDirs;
use kakei::cli::{CLIArgs, Commands};
use kakei_processor::Processor;
use std::path::{Path, PathBuf};

// Use tokio for async runtime
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CLIArgs = CLIArgs::parse();

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
    match args.command {
        Commands::Add {
            date,
            amount,
            currency,
            category,
            account,
            memo,
        } => {
            println!("🚀 Adding transaction...");

            // Call the business logic in processor crate
            let tx_id = processor
                .add_transaction(&date, &amount, &currency, &category, &account, memo)
                .await?;

            println!("Transaction added successfully! (ID: {:?})", tx_id);
        }
        Commands::Init => {
            println!("🔧 Initializing database with default data...");
            processor.init_default_data().await?;

            println!(
                "✅ Initialization complete. Database ready at: {}",
                db_path_str
            );
        }
        Commands::List => {
            println!("📋 Recent Transactions:");
            println!(
                "--------------------------------------------------------------------------------"
            );

            let transactions = processor.get_recent_transactions().await?;

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
        }
    }

    Ok(())
}
