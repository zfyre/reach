
//########################################## EMITTING FOLLOWING MODULES ##########################################//

mod api_config;
pub use api_config::*;
mod arxiv_config;
pub use arxiv_config::*;

//############################################### EXTERNAL IMPORTS ###############################################//

use std::{env, fs, path::PathBuf, str::FromStr};
use clap::Parser;
use std::io::{self};

//############################################### INTERNAL IMPORTS ###############################################//

use crate::errors::ReachApiError;

//################################################ MEMBER IMPORTS ################################################//


//############################ COMMON FUNCTIONS/TRAITS/ENUMS (MAY/MAY-NOT BE EMITTED) ############################//

const CONFIG_FILE: &str = ".reach_config"; //FIXME: Change this to .reach_config

pub trait ReachConfigKeys {
    fn as_str(&self) -> String;
}

pub trait ReachConfig {
    type Repr;
    
    fn read_config() -> Result<Self::Repr, ReachApiError>;

    fn get_config_from_user() -> Result<(), ReachApiError>;

    fn prefix() -> String;

    fn get_config_path() -> PathBuf {
        let home = env::var("HOME")
            .or_else(|_| env::var("USERPROFILE"))
            .unwrap();
        PathBuf::from(home).join(CONFIG_FILE)
    }

    fn save_config(key: &str, value: &str) -> Result<(), ReachApiError> {
        let config_path = Self::get_config_path();
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
}
