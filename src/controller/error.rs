use nutca::Error as ModelError;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    ModelError(ModelError),
    RepositoryError(String),
}

impl Error {
    pub fn message(&self) -> String {
        match self {
            Self::ModelError(error) => error.message(),
            Self::RepositoryError(error) => error.clone(),
        }
    }
}
