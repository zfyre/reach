//! Configuration for the API keys

use super::{Parser, ReachApiError, fs, FromStr, io, io::Write, ReachConfig, ReachConfigKeys};

//########################################## KEYS FOR GOOGLE & SEARCH APIS ##########################################//

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
} //TODO: Maybe use this as a field of ApiConfig??!!

impl ReachConfigKeys for ApiKeys {
    /// Get the key as a string
    fn as_str(&self) -> String {
        match self {
            &Self::Google => format!("{}.REACH_GOOGLE_SEARCH_API_KEY", ApiConfig::prefix()),
            &Self::SearchEngine => format!("{}.REACH_GOOGLE_SEARCH_ENGINE_ID", ApiConfig::prefix()),
            &Self::Gemini => format!("{}.REACH_GEMINI_API_KEY", ApiConfig::prefix()),
        }
    }
}

//########################################## SUBCOMMAND FOR GOOGLE & SEARCH APIS ##########################################//

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
    pub show: bool,
}

impl ReachConfig for ApiConfig {
    
    type Repr = Vec<(String, String)>;
    /// Read the configuration from the file
    ///     
    /// # Returns
    ///
    /// * `Result<Vec<(String, String)>, Error>` - The result of the operation
    ///
    /// # Errors
    ///
    /// * If the file cannot be read
    fn read_config() -> Result<Self::Repr, ReachApiError> {
        let config_path = Self::get_config_path();
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
    fn get_config_from_user() -> Result<(), ReachApiError> {
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
            Self::save_config(&ApiKeys::Google.as_str(), &google_api_key.trim())?;
        }
        if !search_engine_id.trim().is_empty() {
            Self::save_config(&ApiKeys::SearchEngine.as_str(), &search_engine_id.trim())?;
        }
        if !gemini_api_key.trim().is_empty() {
            Self::save_config(&ApiKeys::Gemini.as_str(), &gemini_api_key.trim())?;
        }
        Ok(())
    }

    /// Get the prefix for the configuration
    fn prefix() -> String {
        String::from_str("ApiConfig").unwrap()
    }
}
