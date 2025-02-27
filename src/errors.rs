use std::{fmt, io};
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;

#[derive(Debug)]
pub enum ReachError {
    IoError(io::Error),
    NetworkError(ReqwestError),
    SerializationError(SerdeError),
    ConfigError(String),
    DisplayError(String),
    ApiError(String),
    ParsingError(String),
    CrawlerError(String),
}

impl fmt::Display for ReachError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReachError::IoError(e) => write!(f, "IO Error: {}", e),
            ReachError::NetworkError(e) => write!(f, "Network Error: {}", e),
            ReachError::SerializationError(e) => write!(f, "Serialization Error: {}", e),
            ReachError::ConfigError(e) => write!(f, "Configuration Error: {}", e),
            ReachError::DisplayError(e) => write!(f, "Display Error: {}", e),
            ReachError::ApiError(e) => write!(f, "API Error: {}", e),
            ReachError::ParsingError(e) => write!(f, "Parsing Error: {}", e),
            ReachError::CrawlerError(e) => write!(f, "Web Crawler Error: {}", e),
        }
    }
}

impl std::error::Error for ReachError {}

impl From<io::Error> for ReachError {
    fn from(err: io::Error) -> Self {
        ReachError::IoError(err)
    }
}

impl From<ReqwestError> for ReachError {
    fn from(err: ReqwestError) -> Self {
        ReachError::NetworkError(err)
    }
}

impl From<SerdeError> for ReachError {
    fn from(err: SerdeError) -> Self {
        ReachError::SerializationError(err)
    }
}

impl From<String> for ReachError {
    fn from(err: String) -> Self {
        ReachError::ConfigError(err)
    }
}