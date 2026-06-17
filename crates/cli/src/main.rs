use clap::Parser;
use kakei::cli::CLIArgs;

fn main() -> anyhow::Result<()> {
    let _ = CLIArgs::parse();
    println!("Hello, world!");

    Ok(())
}
