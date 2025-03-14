

use super::{
    io, ReachdbError, ReqwestError, SerdeError, fmt, RsearchError, ReachApiError, ReachTuiError
};

#[derive(Debug)]
pub enum ReachError {
    IoError(io::Error),
    NetworkError(ReqwestError),
    SerializationError(SerdeError),
    ReachdbError(ReachdbError),
    RsearchError(RsearchError),
    ReachApiError(ReachApiError),
    ReachTuiError(ReachTuiError),
}

impl fmt::Display for ReachError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReachError::IoError(e) => write!(f, "IO Error: {}", e),
            ReachError::NetworkError(e) => write!(f, "Network Error: {}", e),
            ReachError::SerializationError(e) => write!(f, "Serialization Error: {}", e),
            ReachError::ReachdbError(e) => write!(f, "Reachdb Error: {}", e),
            ReachError::RsearchError(e) => write!(f, "Rsearch Error: {}", e),
            ReachError::ReachApiError(e) => write!(f, "ReachApi Error: {}", e),
            ReachError::ReachTuiError(e) => write!(f, "ReachTui Error: {}", e),
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
impl From<ReachdbError> for ReachError {
    fn from(err: ReachdbError) -> Self {
        ReachError::ReachdbError(err)
    }
}

impl From<RsearchError> for ReachError {
    fn from(err: RsearchError) -> Self {
        ReachError::RsearchError(err)
    }
}

impl From<ReachApiError> for ReachError {
    fn from(err: ReachApiError) -> Self {
        ReachError::ReachApiError(err)
    }
}

impl From<ReachTuiError> for ReachError {
    fn from(err: ReachTuiError) -> Self {
        ReachError::ReachTuiError(err)
    }
}