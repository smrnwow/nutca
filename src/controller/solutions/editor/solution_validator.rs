use crate::controller::Toaster;
use crate::model::solutions::Solution;
use crate::model::Error;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct SolutionValidator {
    toaster: Signal<Toaster>,
    name: Option<Error>,
    is_rendering_enabled: bool,
}

impl SolutionValidator {
    pub fn new(toaster: Signal<Toaster>) -> Self {
        Self {
            toaster,
            is_rendering_enabled: false,
            name: None,
        }
    }

    pub fn init(&mut self, solution: &Solution) {
        self.validate_name(solution.name());
    }

    pub fn enable_rendering(&mut self) {
        self.is_rendering_enabled = true;
    }

    pub fn validate_name(&mut self, name: &String) {
        if name.len() == 0 {
            self.name = Some(Error::SolutionNameEmpty);
            return;
        }

        if name.len() > 100 {
            self.name = Some(Error::SolutionNameTooLong);
            return;
        }

        self.name = None;
    }

    pub fn validate(&mut self) -> bool {
        self.enable_rendering();

        if let Some(error) = self.name {
            self.toaster.write().add(error.message());
            return false;
        }

        true
    }

    pub fn name(&self) -> Option<String> {
        if self.is_rendering_enabled {
            if let Some(error) = self.name {
                return Some(error.message());
            }
        }

        None
    }
}
