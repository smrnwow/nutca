use crate::controller::fertilizers::FertilizersListing;
use crate::controller::profiles::ProfilesListing;
use crate::controller::{Error, Validation};
use crate::model::profiles::Profile;
use crate::model::solutions::{Solution, SolutionBuilder};
use crate::repository::Storage;
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub struct SolutionEditor {
    is_draft: Signal<bool>,
    storage: Signal<Storage>,
    storage_error: Signal<Option<Error>>,
    solution: Memo<Solution>,
    validation: Memo<Validation>,
    profile: Memo<Profile>,
    builder: Signal<SolutionBuilder>,
    fertilizers_listing: Signal<FertilizersListing>,
    profiles_listing: Signal<ProfilesListing>,
}

impl SolutionEditor {
    pub fn new(storage: Signal<Storage>, profile_id: String) -> Self {
        let builder = match storage.read().profiles().get(&profile_id) {
            Ok(profile) => Signal::new(SolutionBuilder::from(profile)),
            Err(_) => Signal::new(SolutionBuilder::new()),
        };

        let is_draft = Signal::new(true);

        let storage_error = Signal::new(None);

        let solution = Memo::new(move || builder.read().build());

        let validation = Memo::new(move || {
            return Validation::new(
                !*is_draft.read(),
                builder.read().validate(),
                storage_error.read().clone(),
            );
        });

        let mut fertilizers_listing = FertilizersListing::new(storage);

        fertilizers_listing.update_limit(8);

        Self {
            is_draft,
            storage,
            storage_error,
            builder,
            validation,
            solution,
            profile: Memo::new(move || solution.read().profile()),
            fertilizers_listing: Signal::new(fertilizers_listing),
            profiles_listing: Signal::new(ProfilesListing::new(storage)),
        }
    }

    pub fn edit(storage: Signal<Storage>, solution_id: String) -> Self {
        let builder = match storage.read().solutions().get(&solution_id) {
            Ok(solution) => Signal::new(SolutionBuilder::from(solution)),
            Err(_) => Signal::new(SolutionBuilder::new()),
        };

        let is_draft = Signal::new(true);

        let storage_error = Signal::new(None);

        let solution = Memo::new(move || builder.read().build());

        let validation = Memo::new(move || {
            return Validation::new(
                !*is_draft.read(),
                builder.read().validate(),
                storage_error.read().clone(),
            );
        });

        let mut fertilizers_listing = FertilizersListing::new(storage);

        fertilizers_listing.update_limit(8);

        fertilizers_listing.exclude_many(
            solution
                .read()
                .fertilizers()
                .iter()
                .map(|fertilizer| fertilizer.id())
                .collect(),
        );

        Self {
            is_draft,
            storage,
            storage_error,
            builder,
            validation,
            solution,
            profile: Memo::new(move || solution.read().profile()),
            fertilizers_listing: Signal::new(fertilizers_listing),
            profiles_listing: Signal::new(ProfilesListing::new(storage)),
        }
    }

    pub fn solution(&self) -> Memo<Solution> {
        self.solution
    }

    pub fn validation(&self) -> Memo<Validation> {
        self.validation
    }

    pub fn profile(&self) -> Memo<Profile> {
        self.profile
    }

    pub fn builder(&self) -> Signal<SolutionBuilder> {
        self.builder
    }

    pub fn fertilizers_listing(&self) -> Signal<FertilizersListing> {
        self.fertilizers_listing
    }

    pub fn profiles_listing(&self) -> Signal<ProfilesListing> {
        self.profiles_listing
    }

    pub fn change_profile(&mut self, profile_id: String) {
        let profile = self.profiles_listing.read().find(profile_id);
        self.builder.write().profile(profile);
    }

    pub fn select_fertilizer(&mut self, fertilizer_id: String) {
        let fertilizer = self.fertilizers_listing.write().exclude(fertilizer_id);

        if let Some(fertilizer) = fertilizer {
            self.builder.write().add_fertilizer(fertilizer);
        }
    }

    pub fn exclude_fertilizer(&mut self, fertilizer_id: String) {
        let fertilizer = self.fertilizers_listing.write().include(fertilizer_id);

        if let Some(fertilizer) = fertilizer {
            self.builder.write().remove_fertilizer(fertilizer.id());
        }
    }

    pub fn create(&mut self) {
        self.is_draft.set(false);

        if self.validation.read().is_empty() {
            let result = self
                .storage
                .read()
                .solutions()
                .add(self.solution.read().clone());

            match result {
                Ok(_) => {
                    navigator().push(Route::SolutionsMainPage {});
                }
                Err(error) => {
                    self.storage_error
                        .set(Some(Error::RepositoryError(error.to_string())));
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.is_draft.set(false);

        if self.validation.read().is_empty() {
            let result = self
                .storage
                .read()
                .solutions()
                .update(self.solution.read().clone());

            match result {
                Ok(_) => {
                    navigator().push(Route::SolutionsMainPage {});
                }
                Err(error) => {
                    self.storage_error
                        .set(Some(Error::RepositoryError(error.to_string())));
                }
            }
        }
    }
}
