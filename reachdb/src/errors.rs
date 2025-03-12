use std::{fmt, io};

#[derive(Debug)]
pub enum ReachdbError {
    IoError(io::Error),
    BincodeError(bincode::Error),
    SerdeJsonError(serde_json::Error),
    SledError(sled::Error),
    FromUtf8Error(std::string::FromUtf8Error),
    OtherError(String),
}

impl fmt::Display for ReachdbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReachdbError::IoError(e) => write!(f, "IO Error: {}", e),
            ReachdbError::BincodeError(e) => write!(f, "Bincode Error: {}", e),
            ReachdbError::SerdeJsonError(e) => write!(f, "Serde Error: {}", e),
            ReachdbError::SledError(e) => write!(f, "Sled Error: {}", e),
            ReachdbError::FromUtf8Error(e) => write!(f, "FromUtf8 Error: {}", e),
            ReachdbError::OtherError(e) => write!(f, "Error: {}", e),
        }
    }
}

impl std::error::Error for ReachdbError {}

impl From<std::io::Error> for ReachdbError {
    fn from(err: std::io::Error) -> Self {
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
impl From<sled::Error> for ReachdbError {
    fn from(err: sled::Error) -> Self {
        ReachdbError::SledError(err)
    }
}
impl From<std::string::FromUtf8Error> for ReachdbError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        ReachdbError::FromUtf8Error(err)
    }
}
impl From<String> for ReachdbError {
    fn from(err: String) -> Self {
        ReachdbError::OtherError(err)
    }
}


// impl From<String> for ReachdbError {
//     fn from(err: String) -> Self {
//         ReachdbError::ConfigError(err)
//     }
// }