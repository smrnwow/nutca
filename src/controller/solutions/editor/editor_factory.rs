use super::{Editor, FertilizersPicker, ProfilesBrowser, SolutionValidator};
use crate::controller::Toaster;
use crate::model::solutions::{ProfileRequirement, Solution};
use crate::repository::{FertilizersRepository, ProfilesRepository, SolutionsRepository, Storage};
use dioxus::prelude::*;

pub struct EditorFactory {
    storage: Signal<Storage>,
    toaster: Signal<Toaster>,
}

impl EditorFactory {
    pub fn new(storage: Signal<Storage>, toaster: Signal<Toaster>) -> Self {
        Self { storage, toaster }
    }

    pub fn create(&self, profile_id: String, _concentrate_id: String) -> Editor {
        self.from_profile(&profile_id)
    }

    pub fn edit(&self, solution_id: String) -> Editor {
        let solutions_repository = SolutionsRepository::new(self.storage);
        let solution = solutions_repository.get(&solution_id);

        let profiles_repository = ProfilesRepository::new(self.storage);
        let profiles_browser = ProfilesBrowser::new(profiles_repository);

        let fertilizers_repository = FertilizersRepository::new(self.storage);
        let mut fertilizers_picker = FertilizersPicker::new(fertilizers_repository);
        fertilizers_picker.set_projection(solution.fertilizers().keys().collect());

        let mut solution_validator = SolutionValidator::new(self.toaster);
        solution_validator.init(&solution);

        Editor::new(
            solutions_repository,
            solution,
            solution_validator,
            profiles_browser,
            fertilizers_picker,
        )
    }

    fn from_profile(&self, profile_id: &String) -> Editor {
        let solutions_repository = SolutionsRepository::new(self.storage);

        let profiles_repository = ProfilesRepository::new(self.storage);
        let profiles_browser = ProfilesBrowser::new(profiles_repository);
        let profile = profiles_browser.find(&profile_id);

        let fertilizers_repository = FertilizersRepository::new(self.storage);
        let fertilizers_picker = FertilizersPicker::new(fertilizers_repository);

        let mut profile_requirement = ProfileRequirement::new();
        if let Some(profile) = profile {
            profile_requirement.set_profile(profile);
        }

        let solution = Solution::from(profile_requirement);

        let mut solution_validator = SolutionValidator::new(self.toaster);
        solution_validator.init(&solution);

        Editor::new(
            solutions_repository,
            solution,
            solution_validator,
            profiles_browser,
            fertilizers_picker,
        )
    }
}
