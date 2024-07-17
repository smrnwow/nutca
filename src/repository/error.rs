use rusqlite::Error as RusqliteError;
use serde_json::Error as SerdeJsonError;

pub type Error = Box<dyn std::error::Error>;

#[derive(Clone, Debug)]
pub struct RepositoryError {
    pub message: String,
}

impl RepositoryError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for RepositoryError {}

impl From<RusqliteError> for RepositoryError {
    fn from(error: RusqliteError) -> Self {
        match error {
            _ => Self::new("database error"),
        }
    }
}

impl From<SerdeJsonError> for RepositoryError {
    fn from(_: SerdeJsonError) -> Self {
        Self::new("serialization error")
    }
}
