mod api_config;
mod arxiv_config;

pub use api_config::*;
pub use arxiv_config::*;

use crate::errors::ReachError;

use clap::Parser;
use std::io::{self, Write};
use std::{env, fs, path::PathBuf, str::FromStr};

/// The name of the configuration file
const CONFIG_FILE: &str = ".reach-config";

/// Get the path to the configuration file
///
/// # Returns
///
/// * `PathBuf` - The path to the configuration file
fn get_config_path() -> PathBuf {
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap();
    PathBuf::from(home).join(CONFIG_FILE)
}

/// Save the configuration to the file
///
/// # Arguments
///
/// * `key` - The key to be saved
///
/// * `value` - The value to be saved
///
/// # Returns
///
/// * `Result<(), Error>` - The result of the operation
///
/// # Errors
///
/// * If the file cannot be read or written to
fn save_config(key: &str, value: &str) -> Result<(), ReachError> {
    let config_path = get_config_path();
    let content = if config_path.exists() {
        let existing = fs::read_to_string(&config_path)?;
        let mut lines: Vec<String> = existing
            .lines()
            .filter(|line| !line.starts_with(key))
            .map(|line| line.to_string())
            .collect();
        lines.push(format!("{}={}", key, value));
        lines.join("\n")
    } else {
        format!("{}={}", key, value)
    };
    fs::write(config_path, content)?;
    Ok(())
}
