use rusqlite::Error as RusqliteError;
use serde_json::Error as SerdeJsonError;

#[derive(Debug)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

impl From<RusqliteError> for Error {
    fn from(error: RusqliteError) -> Self {
        match error {
            _ => Self::new("database error"),
        }
    }
}

impl From<SerdeJsonError> for Error {
    fn from(error: SerdeJsonError) -> Self {
        Self::new("serialization error")
    }
}
