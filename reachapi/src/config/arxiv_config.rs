//! Configuration for Arxiv

use super::{get_config_path, Parser, ReachApiError, fs, FromStr, io, io::Write};

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
    Categories,
} //TODO: Maybe use this as a field of ArxivConfig??!!

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
    pub show: bool,
}

impl ArxivConfig {
    /// Read the configuration from the file
    ///
    /// # Returns
    ///
    /// * `Result<Vec<(String, Vec<String>)>, Error>` - The result of the operation
    pub fn read_config() -> Result<Vec<(String, Vec<String>)>, ReachApiError> {
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
    pub fn get_config_from_user() -> Result<(), ReachApiError> {
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
