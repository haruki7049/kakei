use clap::Parser;
use serde_derive::{Deserialize, Serialize};

fn main() -> Result<(), confy::ConfyError> {
    let args: Args = Args::parse();

    if args.initialize_configuration_file {
        init_config()?;
    }

    Ok(())
}

fn init_config() -> Result<(), confy::ConfyError> {
    let app_name = env!("CARGO_PKG_NAME");
    let config_name = "config";

    // Initialize config
    let mut cfg = KakeiConfig::default();

    cfg.version = env!("CARGO_PKG_VERSION").to_string();

    // Records configuration to filesystem
    confy::store(app_name, config_name, cfg)?;

    Ok(())
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, default_value_t = false)]
    initialize_configuration_file: bool,
}

#[derive(Default, Serialize, Deserialize)]
struct KakeiConfig {
    version: String,
    sheets: Vec<String>,
}
