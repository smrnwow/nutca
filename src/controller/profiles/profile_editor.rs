use crate::controller::{Error, Validation};
use crate::repository::Storage;
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::model::profiles::{Profile, ProfileBuilder};

pub struct ProfileEditor {
    is_draft: Signal<bool>,
    storage: Signal<Storage>,
    storage_error: Signal<Option<Error>>,
    builder: Signal<ProfileBuilder>,
    validation: Memo<Validation>,
    profile: Memo<Profile>,
}

impl ProfileEditor {
    pub fn new(storage: Signal<Storage>) -> Self {
        let builder = Signal::new(ProfileBuilder::new());

        let storage_error = Signal::new(None);

        let is_draft = Signal::new(true);

        Self {
            is_draft,
            storage,
            storage_error,
            builder,
            validation: Memo::new(move || {
                return Validation::new(
                    !*is_draft.read(),
                    builder.read().validate(),
                    storage_error.read().clone(),
                );
            }),
            profile: Memo::new(move || builder.read().build()),
        }
    }

    pub fn edit(storage: Signal<Storage>, profile_id: String) -> Self {
        let profile = storage.read().profiles().get(profile_id);

        let builder = match profile {
            Ok(profile) => Signal::new(ProfileBuilder::from(profile)),
            Err(_) => Signal::new(ProfileBuilder::new()),
        };

        let storage_error = Signal::new(None);

        let is_draft = Signal::new(true);

        Self {
            is_draft,
            storage,
            storage_error,
            builder,
            validation: Memo::new(move || {
                return Validation::new(
                    !*is_draft.read(),
                    builder.read().validate(),
                    storage_error.read().clone(),
                );
            }),
            profile: Memo::new(move || builder.read().build()),
        }
    }

    pub fn builder(&self) -> Signal<ProfileBuilder> {
        self.builder
    }

    pub fn profile(&self) -> Memo<Profile> {
        self.profile
    }

    pub fn validation(&self) -> Memo<Validation> {
        self.validation
    }

    pub fn create(&mut self) {
        self.is_draft.set(false);

        if self.validation.read().is_empty() {
            let result = self
                .storage
                .read()
                .profiles()
                .add(self.profile.read().clone());

            match result {
                Ok(_) => {
                    navigator().push(Route::ProfilesMainPage {});
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
                .profiles()
                .update(self.profile.read().clone());

            match result {
                Ok(_) => {
                    navigator().push(Route::ProfilesMainPage {});
                }
                Err(error) => {
                    self.storage_error
                        .set(Some(Error::RepositoryError(error.to_string())));
                }
            }
        }
    }

    pub fn back(&self) {
        navigator().push(Route::ProfilesMainPage {});
    }
}
