use std::fmt;
use reachapi::ReachApiError;

#[derive(Debug)]
pub enum RchatError {
    ReachApiError(ReachApiError),
    IoError(std::io::Error),
}

impl fmt::Display for RchatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RchatError::ReachApiError(err) => write!(f, "ReachApiError: {}", err),
            RchatError::IoError(err) => write!(f, "IoError: {}", err),
        }
    }
}

impl std::error::Error for RchatError {}

impl From<ReachApiError> for RchatError {
    fn from(err: ReachApiError) -> Self {
        RchatError::ReachApiError(err)
    }
}

impl From<std::io::Error> for RchatError {
    fn from(err: std::io::Error) -> Self {
        RchatError::IoError(err)
    }
}