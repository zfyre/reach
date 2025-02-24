#[derive(Debug)]
#[allow(dead_code)]
pub enum ReachError {
    ReqwestError(reqwest::Error),
    IoError(std::io::Error),
    CommandProcessError(Box<dyn std::error::Error>),
    JsonParseError(serde_json::Error),
    UnexpectedError(String),
}

impl From<reqwest::Error> for ReachError {
    fn from(err: reqwest::Error) -> ReachError {
        ReachError::ReqwestError(err)
    }
}

impl From<std::io::Error> for ReachError {
    fn from(err: std::io::Error) -> ReachError {
        ReachError::IoError(err)
    }
}

impl From<Box<dyn std::error::Error>> for ReachError {
    fn from(err: Box<dyn std::error::Error>) -> ReachError {
        ReachError::CommandProcessError(err)
    }
}

impl From<serde_json::Error> for ReachError {
    fn from(err: serde_json::Error) -> ReachError {
        ReachError::JsonParseError(err)
    }
}
