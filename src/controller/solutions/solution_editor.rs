use super::{EditMode, FertilizersPicker, NutritionProgramBrowser};
use crate::controller::{Error, Validation};
use crate::model::chemistry::{NutrientAmount, Volume};
use crate::model::profiles::Profile;
use crate::model::solutions::{NutrientComposition, Solution, SolutionBuilder, Solver};
use crate::model::Error as ModelError;
use crate::repository::Storage;
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub struct SolutionEditor {
    is_draft: Signal<bool>,
    edit_mode: Signal<EditMode>,
    storage: Signal<Storage>,
    validation: Validation,
    id: Option<String>,
    name: String,
    profile: Profile,
    volume: Volume,
    composition: NutrientComposition,
    nutrition_program_browser: NutritionProgramBrowser,
    fertilizers_picker: FertilizersPicker,
}

impl SolutionEditor {
    pub fn new(storage: Signal<Storage>, profile_id: String) -> Self {
        let profile = match storage.read().profiles().get(&profile_id) {
            Ok(profile) => profile,
            Err(_) => Profile::default(),
        };

        let is_draft = Signal::new(true);

        Self {
            is_draft,
            edit_mode: Signal::new(EditMode::default()),
            storage,
            validation: Validation::default(),
            id: None,
            name: String::new(),
            volume: Volume::default(),
            composition: NutrientComposition::new(),
            profile,
            fertilizers_picker: FertilizersPicker::new(storage),
            nutrition_program_browser: NutritionProgramBrowser::new(storage),
        }
    }

    pub fn edit(storage: Signal<Storage>, solution_id: String) -> Self {
        let solution = match storage.read().solutions().get(&solution_id) {
            Ok(solution) => solution,
            Err(_) => Solution::default(),
        };

        let is_draft = Signal::new(true);

        let mut fertilizers_picker = FertilizersPicker::new(storage);

        fertilizers_picker.with_picked_fertilizers(solution.fertilizers());

        Self {
            is_draft,
            edit_mode: Signal::new(EditMode::default()),
            storage,
            validation: Validation::default(),
            id: Some(solution.id()),
            name: solution.name(),
            volume: solution.volume(),
            profile: solution.profile(),
            composition: NutrientComposition::from(solution.composition()),
            fertilizers_picker,
            nutrition_program_browser: NutritionProgramBrowser::new(storage),
        }
    }

    pub fn edit_mode(&self) -> Signal<EditMode> {
        self.edit_mode
    }

    pub fn solution(&self) -> Solution {
        SolutionBuilder::new()
            .with_id(self.id.clone())
            .with_name(self.name.clone())
            .with_profile(self.profile.clone())
            .with_fertilizers(
                self.fertilizers_picker
                    .selected_set
                    .list_fertilizers_amounts(),
            )
            .with_composition(self.composition.nutrients().clone())
            .with_volume(self.volume)
            .build()
    }

    pub fn validation(&self) -> Validation {
        self.validation.clone()
    }

    pub fn fertilizers_picker(&self) -> &FertilizersPicker {
        &self.fertilizers_picker
    }

    pub fn nutrition_program_browser(&self) -> &NutritionProgramBrowser {
        &self.nutrition_program_browser
    }

    pub fn search_nutrient_program(&mut self, search_query: String) {
        self.nutrition_program_browser.search(search_query);
    }

    pub fn search_fertilizer(&mut self, search_query: String) {
        self.fertilizers_picker.browser.search(search_query);
    }

    pub fn paginate_fertilizers_browser(&mut self, page_index: usize) {
        self.fertilizers_picker.browser.paginate(page_index);
    }

    pub fn paginate_selected_set(&mut self, page_index: usize) {
        self.fertilizers_picker.selected_set.paginate(page_index);
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;

        if self.name.len() == 0 {
            self.validation
                .add_validation_error(ModelError::SolutionNameEmpty);
        }

        if self.name.len() > 100 {
            self.validation
                .add_validation_error(ModelError::SolutionNameTooLong);
        }
    }

    pub fn update_volume(&mut self, volume: Volume) {
        self.volume = volume;
    }

    pub fn change_profile(&mut self, profile_id: String) {
        match self.nutrition_program_browser.find(profile_id) {
            Some(profile) => {
                self.profile = profile;

                self.switch_edit_mode(EditMode::Automatic);

                self.calculate_composition();
            }
            None => {
                self.profile = Profile::default();

                self.switch_edit_mode(EditMode::Manual);

                self.calculate_composition();
            }
        }
    }

    pub fn select_fertilizer(&mut self, fertilizer_id: String) {
        self.fertilizers_picker.pick_fertilizer(&fertilizer_id);

        self.calculate_composition();
    }

    pub fn exclude_fertilizer(&mut self, fertilizer_id: String) {
        self.fertilizers_picker.exclude_fertilizer(&fertilizer_id);

        self.calculate_composition();
    }

    pub fn update_fertilizer_amount(&mut self, fertilizer_id: String, amount: f64) {
        self.fertilizers_picker
            .update_fertilizer_amount(&fertilizer_id, amount);

        self.switch_edit_mode(EditMode::Manual);

        self.profile = Profile::default();

        self.calculate_composition();
    }

    pub fn update_nutrient_requirement(&mut self, nutrient_requirement: NutrientAmount) {
        if self.profile.is_saved() {
            self.profile = Profile::extend(&self.profile);
        }

        self.profile
            .update_nutrient_requirement(nutrient_requirement);

        self.switch_edit_mode(EditMode::Automatic);

        self.calculate_composition();
    }

    pub fn create(&mut self) {
        self.is_draft.set(false);

        if self.validation.is_empty() {
            let result = self.storage.read().solutions().add(self.solution());

            match result {
                Ok(_) => {
                    navigator().push(Route::SolutionsMainPage {});
                }
                Err(error) => {
                    self.validation
                        .add_storage_error(Error::RepositoryError(error.to_string()));
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.is_draft.set(false);

        if self.validation.is_empty() {
            let result = self.storage.read().solutions().update(self.solution());

            match result {
                Ok(_) => {
                    navigator().push(Route::SolutionsMainPage {});
                }
                Err(error) => {
                    self.validation
                        .add_storage_error(Error::RepositoryError(error.to_string()));
                }
            }
        }
    }

    fn switch_edit_mode(&mut self, edit_mode: EditMode) {
        self.edit_mode.set(edit_mode);
    }

    fn calculate_composition(&mut self) {
        if let EditMode::Automatic = *self.edit_mode.read() {
            let calculation_results = Solver::new(
                &self.profile,
                self.fertilizers_picker
                    .selected_set
                    .list_selected_fertilizers(),
            )
            .solve();

            self.fertilizers_picker
                .with_picked_fertilizers(&calculation_results);
        }

        self.composition = NutrientComposition::from(
            self.fertilizers_picker
                .selected_set
                .list_fertilizers_amounts(),
        );
    }
}
