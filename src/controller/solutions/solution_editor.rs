use super::EditMode;
use crate::controller::fertilizers::FertilizersListing;
use crate::controller::profiles::ProfilesListing;
use crate::controller::{Error, Validation};
use crate::model::chemistry::{NutrientAmount, Nutrients, Volume};
use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::Profile;
use crate::model::solutions::{
    FertilizerWeight, FertilizersSet, Solution, SolutionBuilder, Solver,
};
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
    fertilizers_set: FertilizersSet,
    volume: Volume,
    composition: Nutrients,
    fertilizers_listing: Signal<FertilizersListing>,
    profiles_listing: Signal<ProfilesListing>,
}

impl SolutionEditor {
    pub fn new(storage: Signal<Storage>, profile_id: String) -> Self {
        let profile = match storage.read().profiles().get(&profile_id) {
            Ok(profile) => profile,
            Err(_) => Profile::default(),
        };

        let is_draft = Signal::new(true);

        let mut fertilizers_listing = FertilizersListing::new(storage);

        fertilizers_listing.update_limit(8);

        Self {
            is_draft,
            edit_mode: Signal::new(EditMode::default()),
            storage,
            validation: Validation::default(),
            id: None,
            name: String::new(),
            volume: Volume::default(),
            composition: Nutrients::new(),
            profile,
            fertilizers_set: FertilizersSet::new(Vec::new()),
            fertilizers_listing: Signal::new(fertilizers_listing),
            profiles_listing: Signal::new(ProfilesListing::new(storage)),
        }
    }

    pub fn edit(storage: Signal<Storage>, solution_id: String) -> Self {
        let solution = match storage.read().solutions().get(&solution_id) {
            Ok(solution) => solution,
            Err(_) => Solution::default(),
        };

        let is_draft = Signal::new(true);

        let mut fertilizers_listing = FertilizersListing::new(storage);

        fertilizers_listing.update_limit(8);

        fertilizers_listing.exclude_many(
            solution
                .fertilizers()
                .iter()
                .map(|fertilizer| fertilizer.id())
                .collect(),
        );

        let fertilizers_set = FertilizersSet::new(solution.fertilizers());

        Self {
            is_draft,
            edit_mode: Signal::new(EditMode::default()),
            storage,
            validation: Validation::default(),
            id: Some(solution.id()),
            name: solution.name(),
            volume: solution.volume(),
            profile: solution.profile(),
            fertilizers_set,
            composition: solution.composition(),
            fertilizers_listing: Signal::new(fertilizers_listing),
            profiles_listing: Signal::new(ProfilesListing::new(storage)),
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
            .with_fertilizers_set(self.fertilizers_set.clone())
            .with_composition(self.composition)
            .with_volume(self.volume)
            .build()
    }

    pub fn validation(&self) -> Validation {
        self.validation.clone()
    }

    pub fn fertilizers_listing(&self) -> Signal<FertilizersListing> {
        self.fertilizers_listing
    }

    pub fn profiles_listing(&self) -> Signal<ProfilesListing> {
        self.profiles_listing
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
        let profile = self.profiles_listing.read().find(profile_id);

        match profile {
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
        let fertilizer = self.fertilizers_listing.write().exclude(fertilizer_id);

        if let Some(fertilizer) = fertilizer {
            self.fertilizers_set
                .add_fertilizer_weight(FertilizerWeight::new(fertilizer, 0.0));

            self.calculate_composition();
        }
    }

    pub fn exclude_fertilizer(&mut self, fertilizer_id: String) {
        let fertilizer = self.fertilizers_listing.write().include(fertilizer_id);

        if let Some(fertilizer) = fertilizer {
            self.fertilizers_set.remove_fertilizer(fertilizer.id());

            self.calculate_composition();
        }
    }

    pub fn update_fertilizer_amount(&mut self, fertilizer_id: String, amount: f64) {
        self.fertilizers_set
            .update_fertilizer_amount(fertilizer_id, amount);

        self.profile = Profile::default();

        self.switch_edit_mode(EditMode::Manual);

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
        match *self.edit_mode.read() {
            EditMode::Manual => {
                self.composition = self.fertilizers_set.nutrients();
            }

            EditMode::Automatic => {
                let fertilizers = self
                    .fertilizers_set
                    .list()
                    .iter()
                    .map(|fertilizer_amount| fertilizer_amount.clone().into())
                    .collect::<Vec<Fertilizer>>();

                self.fertilizers_set =
                    Solver::new(&self.profile, fertilizers.iter().collect()).solve();

                self.composition = self.fertilizers_set.nutrients();
            }
        }
    }
}
