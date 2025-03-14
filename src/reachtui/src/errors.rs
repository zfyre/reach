use std::{fmt, io};

#[derive(Debug)]
pub enum ReachTuiError {
    IoError(io::Error),
    FromUtf8Error(std::string::FromUtf8Error),
    OtherError(String),
}

impl fmt::Display for ReachTuiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReachTuiError::IoError(e) => write!(f, "IO Error: {}", e),
            ReachTuiError::FromUtf8Error(e) => write!(f, "FromUtf8 Error: {}", e),
            ReachTuiError::OtherError(e) => write!(f, "Error: {}", e),
        }
    }
}

impl std::error::Error for ReachTuiError {}

impl From<std::io::Error> for ReachTuiError {
    fn from(err: std::io::Error) -> Self {
        ReachTuiError::IoError(err)
    }

}
impl From<std::string::FromUtf8Error> for ReachTuiError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        ReachTuiError::FromUtf8Error(err)
    }
}
impl From<String> for ReachTuiError {
    fn from(err: String) -> Self {
        ReachTuiError::OtherError(err)
    }
}

// impl From<String> for ReachdbError {
//     fn from(err: String) -> Self {
//         ReachdbError::ConfigError(err)
//     }
// }