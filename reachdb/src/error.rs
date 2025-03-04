use std::{fmt, io};

#[derive(Debug)]
pub enum ReachdbError {
    IoError(io::Error),
    BincodeError(bincode::Error),
    SerdeJsonError(serde_json::Error),
}

impl fmt::Display for ReachdbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReachdbError::IoError(e) => write!(f, "IO Error: {}", e),
            ReachdbError::BincodeError(e) => write!(f, "Bincode Error: {}", e),
            ReachdbError::SerdeJsonError(e) => write!(f, "Serde Error: {}", e),
        }
    }
}

impl std::error::Error for ReachdbError {}

impl From<io::Error> for ReachdbError {
    fn from(err: io::Error) -> Self {
        ReachdbError::IoError(err)
    }
}
impl From<bincode::Error> for ReachdbError {
    fn from(err: bincode::Error) -> Self {
        ReachdbError::BincodeError(err)
    }
}
impl From<serde_json::Error> for ReachdbError {
    fn from(err: serde_json::Error) -> Self {
        ReachdbError::SerdeJsonError(err)
    }
}

// impl From<String> for ReachdbError {
//     fn from(err: String) -> Self {
//         ReachdbError::ConfigError(err)
//     }
// }