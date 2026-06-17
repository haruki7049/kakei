use clap::Parser;
use directories::ProjectDirs;
use std::{
    path::PathBuf,
    sync::{LazyLock, Mutex},
};

fn main() -> anyhow::Result<()> {
    let _ = CLIArgs::parse();
    println!("Hello, world!");

    Ok(())
}

#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub struct CLIArgs {
    /// The file path to kakeibo note.
    pub kakeibo: PathBuf,

    /// kakei's config file path
    #[arg(short, long, default_value = DEFAULT_CONFIG_PATH.lock().unwrap().display().to_string())]
    pub config: PathBuf,
}

/// Default Configuration Path, using directories crate to calculate ProjectDirs (~/.config/kakei)
static DEFAULT_CONFIG_PATH: LazyLock<Mutex<PathBuf>> = LazyLock::new(|| {
    let proj_dirs = ProjectDirs::from("dev", "haruki7049", "kakei")
        .expect("Failed to search ProjectDirs for dev.haruki7049.kakei");
    let mut config_path: PathBuf = proj_dirs.config_dir().to_path_buf();
    let filename: &str = "config.toml";

    config_path.push(filename);
    Mutex::new(config_path)
});
