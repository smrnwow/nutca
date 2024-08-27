use crate::controller::Error;
use crate::model::Error as ModelError;

#[derive(Clone, Debug, PartialEq)]
pub struct Validation {
    show_errors: bool,
    errors: Vec<Error>,
}

impl Validation {
    pub fn new(
        show_errors: bool,
        validation_errors: Vec<ModelError>,
        storage_error: Option<Error>,
    ) -> Self {
        let mut errors: Vec<Error> = Vec::new();

        for error in validation_errors {
            errors.push(Error::ModelError(error));
        }

        if let Some(error) = storage_error {
            errors.push(error);
        }

        Self {
            show_errors,
            errors,
        }
    }

    pub fn add_validation_error(&mut self, error: ModelError) {
        self.errors.push(Error::ModelError(error));
    }

    pub fn add_storage_error(&mut self, error: Error) {
        self.errors.push(error);
    }

    pub fn list(&self) -> Vec<String> {
        if self.show_errors {
            self.errors.iter().map(|error| error.message()).collect()
        } else {
            Vec::new()
        }
    }

    pub fn get(&self, entity: &str) -> Option<String> {
        if self.show_errors {
            let error = self.errors.iter().find(|error| {
                if let Error::ModelError(model_error) = error {
                    return model_error.entity() == entity;
                }

                false
            });

            match error {
                Some(error) => Some(error.message()),
                None => None,
            }
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.errors.len() == 0
    }
}

impl Default for Validation {
    fn default() -> Self {
        Self {
            show_errors: true,
            errors: Vec::new(),
        }
    }
}
