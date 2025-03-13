use std::{fmt, io};
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;

#[derive(Debug)]
pub enum ReachApiError {
    IoError(io::Error),
    NetworkError(ReqwestError),
    SerializationError(SerdeError),
}

impl fmt::Display for ReachApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReachApiError::IoError(e) => write!(f, "IO Error: {}", e),
            ReachApiError::NetworkError(e) => write!(f, "Network Error: {}", e),
            ReachApiError::SerializationError(e) => write!(f, "Serialization Error: {}", e),
        }
    }
}

impl std::error::Error for ReachApiError {}

impl From<io::Error> for ReachApiError {
    fn from(err: io::Error) -> Self {
        ReachApiError::IoError(err)
    }
}

impl From<ReqwestError> for ReachApiError {
    fn from(err: ReqwestError) -> Self {
        ReachApiError::NetworkError(err)
    }
}

impl From<SerdeError> for ReachApiError {
    fn from(err: SerdeError) -> Self {
        ReachApiError::SerializationError(err)
    }
}
