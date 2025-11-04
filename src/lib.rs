//! Kakei

pub mod cli;

use serde_derive::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
/// This struct is used by ~/.config/kakei/config.toml
/// And contains kakei's configurations
pub struct KakeiConfig {
    /// Kakei's software version
    pub version: String,

    /// The sheets' names
    pub sheets: Vec<String>,
}
