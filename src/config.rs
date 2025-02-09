use std::{env, fs, path::PathBuf};
use clap::Parser;
use crate::apis::Error;
use std::io::{self, Write};

/// The commands that can be executed
/// 
/// * `Config` - Configure API keys
#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Configure API keys
    Config(ApiConfig)
}

/// Get the path to the configuration file
/// 
/// # Returns
/// 
/// * `PathBuf` - The path to the configuration file
fn get_config_path() -> PathBuf {
    let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).unwrap();
    PathBuf::from(home).join(CONFIG_FILE)
}

/// The name of the configuration file
pub const CONFIG_FILE: &str = ".reach-config";

/// The API keys that can be configured
/// 
/// * `Google` - Google Search API key
/// 
/// * `SearchEngine` - Search Engine ID
/// 
/// * `Gemini` - Gemini API key
#[derive(Debug, Clone, Copy)]
pub enum ApiKeys {
    Google,
    SearchEngine,
    Gemini,
}

impl ApiKeys {
    pub fn as_str(&self) -> &'static str {
        match self {
            ApiKeys::Google => "REACH_GOOGLE_SEARCH_API_KEY",
            ApiKeys::SearchEngine => "REACH_GOOGLE_SEARCH_ENGINE_ID",
            ApiKeys::Gemini => "REACH_GEMINI_API_KEY",
        }
    }
}

#[derive(Parser, Debug)]
pub struct ApiConfig {

    /// Set Google Search API key
    gemini_api_key: Option<String>,

    /// Set Search Engine ID
    google_api_key: Option<String>,
    
    /// Set Gemini API key
    search_engine_id: Option<String>,
    
    /// Show current configuration
    #[arg(long)]
    pub show: bool
}

impl ApiConfig {
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
    pub fn save_config(key: &str, value: &str) -> Result<(), Error> {
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
    
    /// Read the configuration from the file
    ///     
    /// # Returns
    /// 
    /// * `Result<Vec<(String, String)>, Error>` - The result of the operation
    /// 
    /// # Errors
    /// 
    /// * If the file cannot be read
    pub fn read_config() -> Result<Vec<(String, String)>, Error> {
        let config_path = get_config_path();
        if !config_path.exists() {
            return Ok(vec![]);
        }
        let content = fs::read_to_string(config_path)?;
        Ok(content
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    Some((parts[0].to_string(), parts[1].to_string()))
                } else {
                    None
                }
            })
            .collect())
    }

    /// Get the configuration from the user
    /// 
    /// # Returns
    /// 
    /// * `Result<(), Error>` - The result of the operation
    /// 
    /// # Errors
    /// 
    /// * If the input/output fails
    /// 
    /// * If the configuration cannot be saved / read / written
    pub fn get_config_from_user() -> Result<(), Error> {
        let mut google_api_key = String::new();
        let mut search_engine_id = String::new();
        let mut gemini_api_key = String::new();

        print!("Enter Google API Key (press Enter to skip): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut google_api_key)?;

        print!("Enter Search Engine ID (press Enter to skip): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut search_engine_id)?;

        print!("Enter Gemini API Key (press Enter to skip): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut gemini_api_key)?;

        if !google_api_key.trim().is_empty() {
            ApiConfig::save_config(ApiKeys::Google.as_str(), google_api_key.trim())?;
        }
        if !search_engine_id.trim().is_empty() {
            ApiConfig::save_config(ApiKeys::SearchEngine.as_str(), search_engine_id.trim())?;
        }
        if !gemini_api_key.trim().is_empty() {
            ApiConfig::save_config(ApiKeys::Gemini.as_str(), gemini_api_key.trim())?;
        }
        Ok(())
    }
}



