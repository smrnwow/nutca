use super::{FertilizersPicker, FertilizersUsed, ProfilesBrowser, SolutionValidator};
use crate::model::chemistry::{NutrientAmount, Volume};
use crate::model::solutions::Solution;
use crate::repository::SolutionsRepository;
use crate::ui::router::Route;
use dioxus_router::prelude::*;

pub struct Editor {
    solutions_repository: SolutionsRepository,
    solution: Solution,
    solution_validator: SolutionValidator,
    profiles_browser: ProfilesBrowser,
    fertilizers_picker: FertilizersPicker,
    fertilizers_used: FertilizersUsed,
}

impl Editor {
    pub fn new(
        solutions_repository: SolutionsRepository,
        solution: Solution,
        solution_validator: SolutionValidator,
        profiles_browser: ProfilesBrowser,
        fertilizers_picker: FertilizersPicker,
    ) -> Self {
        let fertilizers_used = FertilizersUsed::new(solution.fertilizers().values().collect());

        Self {
            solutions_repository,
            solution,
            solution_validator,
            profiles_browser,
            fertilizers_picker,
            fertilizers_used,
        }
    }

    pub fn solution(&self) -> &Solution {
        &self.solution
    }

    pub fn validator(&self) -> &SolutionValidator {
        &self.solution_validator
    }

    pub fn fertilizers_picker(&self) -> &FertilizersPicker {
        &self.fertilizers_picker
    }

    pub fn profiles_browser(&self) -> &ProfilesBrowser {
        &self.profiles_browser
    }

    pub fn fertilizers_used(&self) -> &FertilizersUsed {
        &self.fertilizers_used
    }

    pub fn search_profile(&mut self, search_query: String) {
        self.profiles_browser.search(search_query);
    }

    pub fn search_fertilizer(&mut self, search_query: String) {
        self.fertilizers_picker.search(search_query);
    }

    pub fn paginate_fertilizers_browser(&mut self, page_index: usize) {
        self.fertilizers_picker.paginate(page_index);
    }

    pub fn paginate_selected_set(&mut self, page_index: usize) {
        self.fertilizers_used.paginate(page_index);
    }

    pub fn update_name(&mut self, name: String) {
        self.solution_validator.validate_name(&name);

        self.solution.update_name(name);
    }

    pub fn update_volume(&mut self, volume: Volume) {
        self.solution.update_volume(volume);

        self.fertilizers_used
            .set_fertilizers(self.solution.fertilizers().values().collect());
    }

    pub fn change_profile(&mut self, profile_id: String) {
        let profile = self.profiles_browser.find(&profile_id);

        self.solution.change_profile(profile);

        self.fertilizers_used
            .set_fertilizers(self.solution.fertilizers().values().collect());
    }

    pub fn change_profile_stage(&mut self, stage_id: String) {
        self.solution.change_profile_stage(stage_id);

        self.fertilizers_used
            .set_fertilizers(self.solution.fertilizers().values().collect());
    }

    pub fn update_nutrient_requirement(&mut self, nutrient_requirement: NutrientAmount) {
        self.solution
            .update_nutrient_requirement(nutrient_requirement);

        self.fertilizers_used
            .set_fertilizers(self.solution.fertilizers().values().collect());
    }

    pub fn select_fertilizer(&mut self, fertilizer_id: String) {
        if let Some(fertilizer) = self.fertilizers_picker.pick(&fertilizer_id) {
            self.solution.add_fertilizer(fertilizer);

            self.fertilizers_used
                .set_fertilizers(self.solution.fertilizers().values().collect());
        }
    }

    pub fn exclude_fertilizer(&mut self, fertilizer_id: String) {
        self.solution.remove_fertilizer(&fertilizer_id);

        self.fertilizers_picker.remove_projection(&fertilizer_id);

        self.fertilizers_used
            .set_fertilizers(self.solution.fertilizers().values().collect());
    }

    pub fn update_fertilizer_amount(&mut self, fertilizer_id: String, amount: f64) {
        self.solution
            .update_fertilizer_amount(&fertilizer_id, amount);

        self.fertilizers_used
            .set_fertilizers(self.solution.fertilizers().values().collect());
    }

    pub fn create(&mut self) {
        if self.solution_validator.validate() {
            if let Ok(_) = self.solutions_repository.create(self.solution.clone()) {
                navigator().push(Route::SolutionsMainPage {});
            }
        }
    }

    pub fn update(&mut self) {
        if self.solution_validator.validate() {
            if let Ok(_) = self.solutions_repository.update(self.solution.clone()) {
                navigator().push(Route::SolutionsMainPage {});
            };
        }
    }
}
