use clap::Parser;

fn main() -> anyhow::Result<()> {
    let _ = CLIArgs::parse();
    println!("Hello, world!");

    Ok(())
}

#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub struct CLIArgs {}
