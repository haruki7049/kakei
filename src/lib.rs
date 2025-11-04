//! Kakei

pub mod cli;

use serde::{Deserialize, Serialize};

/// This struct is used by ~/.config/kakei/config.toml
/// And contains kakei's configurations
#[derive(Default, Serialize, Deserialize)]
pub struct KakeiConfig {
    /// Kakei's software version
    pub version: String,

    /// The sheets' names
    pub sheets: Vec<String>,
}
