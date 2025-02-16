use std::{env, fs, path::PathBuf, str::FromStr};
use clap::Parser;
use crate::apis::Error;
use std::io::{self, Write};

/// The commands that can be executed
/// 
/// * `Config` - Configure API keys
#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// Configure API keys
    ApiConfig(ApiConfig),

    /// Configure Arxiv config
    ArxivConfig(ArxivConfig)
}

/// The name of the configuration file
pub const CONFIG_FILE: &str = ".reach-config";

/// Get the path to the configuration file
/// 
/// # Returns
/// 
/// * `PathBuf` - The path to the configuration file
fn get_config_path() -> PathBuf {
    let home = env::var("HOME").or_else(|_| env::var("USERPROFILE")).unwrap();
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
}//TODO: Maybe use this as a field of ApiConfig??!!

impl ApiKeys {
    /// Get the key as a string
    pub fn as_str(&self) -> String {
        match self {
            &Self::Google => format!("{}.REACH_GOOGLE_SEARCH_API_KEY", ApiConfig::prefix()),
            &Self::SearchEngine => format!("{}.REACH_GOOGLE_SEARCH_ENGINE_ID", ApiConfig::prefix()),
            &Self::Gemini => format!("{}.REACH_GEMINI_API_KEY", ApiConfig::prefix()),
        }
    }
}

/// The configuration for the API keys
/// 
/// * `google_api_key` - Google Search API key
/// 
/// * `search_engine_id` - Search Engine ID
/// 
/// * `gemini_api_key` - Gemini API key
/// 
/// * `show` - Show the current configuration
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
                if line.starts_with(&Self::prefix()) {
                    let parts: Vec<&str> = line.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        Some((parts[0].to_string(), parts[1].to_string()))
                    } else {
                        None
                    }
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
            save_config(&ApiKeys::Google.as_str(), &google_api_key.trim())?;
        }
        if !search_engine_id.trim().is_empty() {
            save_config(&ApiKeys::SearchEngine.as_str(), &search_engine_id.trim())?;
        }
        if !gemini_api_key.trim().is_empty() {
            save_config(&ApiKeys::Gemini.as_str(), &gemini_api_key.trim())?;
        }
        Ok(())
    }

    /// Get the prefix for the configuration
    fn prefix() -> String {
        String::from_str("ApiConfig").unwrap()
    }
}


/// The keys that can be configured for Arxiv
/// 
/// * `IncludeWords` - Include keywords
/// 
/// * `ExcludeWords` - Exclude keywords
/// 
/// * `Authors` - Authors
/// 
/// * `Categories` - Categories
#[derive(Debug, Clone, Copy)]
pub enum ArxivKeys {
    IncludeWords,
    ExcludeWords,
    Authors,
    Categories
}//TODO: Maybe use this as a field of ArxivConfig??!!

impl ArxivKeys {
    
    /// Get the key as a string
    pub fn as_str(&self) -> String {
        match self {
            &Self::IncludeWords => format!("{}.REACH_INCLUDE_WORDS", ArxivConfig::prefix()),
            &Self::ExcludeWords => format!("{}.REACH_EXCLUDE_WORDS", ArxivConfig::prefix()),
            &Self::Authors => format!("{}.REACH_AUTHORS", ArxivConfig::prefix()),
            &Self::Categories => format!("{}.REACH_CATEGORIES", ArxivConfig::prefix()),
        }
    }
}

/// The configuration for Arxiv
/// 
/// * `include_keywords` - Include keywords
/// 
/// * `ignore_keywords` - Exclude keywords
/// 
/// * `authors` - Authors
/// 
/// * `categories` - Categories
/// 
/// * `show` - Show the current configuration
#[derive(Parser, Debug)]
pub struct ArxivConfig {
    
    /// Set include keywords to search
    include_keywords: Option<String>,

    /// Set exclude keywords to not search
    ignore_keywords: Option<String>,
    
    /// Set authors to search
    authors: Option<String>,

    /// Set categories to search
    categories: Option<String>,
    
    /// Show current configuration
    #[arg(long)]
    pub show: bool
}

impl ArxivConfig {
    /// Read the configuration from the file
    /// 
    /// # Returns
    /// 
    /// * `Result<Vec<(String, Vec<String>)>, Error>` - The result of the operation
    pub fn read_config() -> Result<Vec<(String, Vec<String>)>, Error> {
        let config_path = get_config_path();
        if !config_path.exists() {
            return Ok(vec![]);
        }
        let content = fs::read_to_string(config_path)?;
        Ok(content
            .lines()
            .filter_map(|line| {
                if line.starts_with(&Self::prefix()) {
                    let parts: Vec<&str> = line.splitn(2, '=').collect();
                    if parts.len() == 2 {

                        let parts_separated = parts[1]
                            .split(',')
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>();

                        Some((parts[0].to_string(), parts_separated))
                    } else {
                        None
                    }
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
    pub fn get_config_from_user() -> Result<(), Error> {
        let mut include_words = String::new();
        let mut exclude_words = String::new();
        let mut authors = String::new();
        let mut categories = String::new();

        print!("Keywords to be included (Separeted by ',') (press Enter to skip): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut include_words)?;

        print!("Keywords to be excluded (Separeted by ',') (press Enter to skip): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut exclude_words)?;

        print!("Categories to be included (Separeted by ',') (press Enter to skip): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut categories)?;

        print!("Authors to be included (Separeted by ',') (press Enter to skip): ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut authors)?;

        if !include_words.trim().is_empty() {
            save_config(&ArxivKeys::IncludeWords.as_str(), &include_words.trim())?;
        }
        if !exclude_words.trim().is_empty() {
            save_config(&ArxivKeys::ExcludeWords.as_str(), &exclude_words.trim())?;
        }
        if !categories.trim().is_empty() {
            save_config(&ArxivKeys::Categories.as_str(), &categories.trim())?;
        }
        if !authors.trim().is_empty() {
            save_config(&ArxivKeys::Authors.as_str(), &authors.trim())?;
        }

        Ok(())
    }

    /// Get the prefix for the configuration
    fn prefix() -> String {
        String::from_str("ArxivConfig").unwrap()
    }
}