use clap::Parser;
use directories::ProjectDirs;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();

    if args.initialize_configuration_file {
        init_config()?;
    }

    if args.initialize_default_kakeibo {
        init_default_kakeibo()?;
    }

    Ok(())
}

/// Initialize default kakeibo file.
/// To: ~/.local/share/kakei/default.csv
fn init_default_kakeibo() -> anyhow::Result<()> {
    // Gets project_dirs, contains data_dir
    let project_dirs = ProjectDirs::from("dev", "haruki7049", "kakei")
        .expect("ERROR: Cannot read project_dirs for dev.haruki7049.kakei");

    // Gets /home/haruki/.local/share/kakei/default.csv
    let kakeibo_path = project_dirs.data_dir().join("default.csv");

    // Save default.csv to the path with default context
    let mut file = File::create(kakeibo_path)?;
    file.write_all(b"Name,Price\nSushi,-1000\n")?;

    Ok(())
}

/// Initialize configuration file.
/// To: ~/.config/kakei/config.toml
fn init_config() -> anyhow::Result<()> {
    let app_name = env!("CARGO_PKG_NAME");
    let config_name = "config";

    // Initialize config
    let mut cfg = KakeiConfig::default();

    // Write kakei binary's version
    cfg.version = env!("CARGO_PKG_VERSION").to_string();

    // Records configuration to filesystem
    confy::store(app_name, config_name, cfg)?;

    Ok(())
}

#[derive(Debug, Parser)]
#[command(arg_required_else_help = true)]
struct Args {
    /// Initialize configuration file (In Linux: $XDG_CONFIG_HOME/kakei)
    #[arg(long, default_value_t = false)]
    initialize_configuration_file: bool,

    /// Initialize default kakeibo file (In Linux: $XDG_DATA_HOME/kakei)
    #[arg(long, default_value_t = false)]
    initialize_default_kakeibo: bool,
}

#[derive(Default, Serialize, Deserialize)]
/// This struct is used by ~/.config/kakei/config.toml
/// And contains kakei's configurations
struct KakeiConfig {
    /// Kakei's software version
    version: String,

    /// The sheets' names
    sheets: Vec<String>,
}
