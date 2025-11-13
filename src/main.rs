use clap::Parser;
use kakei::cli::{CLIArgs, Commands};
// use kakei::KakeiConfig; // (Will be used later)

fn main() -> anyhow::Result<()> {
    // Parse CLI arguments
    let args: CLIArgs = CLIArgs::parse();

    // Dispatch commands
    match args.command {
        Commands::Add {
            date,
            amount,
            currency,
            category,
            account,
            memo,
        } => {
            println!("🚀 Add command received:");
            println!("  - Date: {}", date);
            println!("  - Amount: {} {}", amount, currency);
            println!("  - Category: {}", category);
            println!("  - Account: {}", account);
            println!("  - Memo: {:?}", memo);

            todo!("Add command received (Not implemented yet).");
        }
        Commands::Init => {
            println!("🔧 Init command received.");

            todo!("Init command received (Not implemented yet).");

            // TODO: Pass these values to the Processor to save in DB
            // TODO: Call init_config() and init_database() here
            // init_config()?;
            // init_default_kakeibo()?;
        }
        Commands::List => {
            todo!("📋 List command received (Not implemented yet).");
        }
    }

    Ok(())
}
