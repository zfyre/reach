use super::{
    ReachdbError,
    ReachApiError,
    fmt 
};

#[derive(Debug)]
pub enum RsearchError {
    ReachdbError(ReachdbError),
    ReachApiError(ReachApiError),
    SerdeError(serde_json::Error),
    IoError(std::io::Error),
}

impl fmt::Display for RsearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RsearchError::ReachdbError(err) => write!(f, "ReachdbError: {}", err),
            RsearchError::ReachApiError(err) => write!(f, "ReachApiError: {}", err),
            RsearchError::SerdeError(err) => write!(f, "SerdeError: {}", err),
            RsearchError::IoError(err) => write!(f, "IoError: {}", err),
        }
    }
}

impl std::error::Error for RsearchError {}

impl From<ReachdbError> for RsearchError {
    fn from(err: ReachdbError) -> Self {
        RsearchError::ReachdbError(err)
    }
}

impl From<ReachApiError> for RsearchError {
    fn from(err: ReachApiError) -> Self {
        RsearchError::ReachApiError(err)
    }
}

impl From<serde_json::Error> for RsearchError {
    fn from(err: serde_json::Error) -> Self {
        RsearchError::SerdeError(err)
    }
}

impl From<std::io::Error> for RsearchError {
    fn from(err: std::io::Error) -> Self {
        RsearchError::IoError(err)
    }
}