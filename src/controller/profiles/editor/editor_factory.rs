use super::{Editor, ProfileValidator};
use crate::controller::Toaster;
use crate::model::profiles::Profile;
use crate::repository::{ProfilesRepository, Storage};
use dioxus::prelude::*;

pub struct EditorFactory {
    storage: Signal<Storage>,
    toaster: Signal<Toaster>,
}

impl EditorFactory {
    pub fn new(storage: Signal<Storage>, toaster: Signal<Toaster>) -> Self {
        Self { storage, toaster }
    }

    pub fn create(&self) -> Editor {
        let profiles_repository = ProfilesRepository::new(self.storage);

        let profile = Profile::new();

        let mut profile_validator = ProfileValidator::new(self.toaster);
        profile_validator.init(&profile);

        Editor::new(profiles_repository, profile, profile_validator)
    }

    pub fn edit(&self, profile_id: &str) -> Editor {
        let profiles_repository = ProfilesRepository::new(self.storage);
        let profile = profiles_repository.find_or_default(profile_id);

        let mut profile_validator = ProfileValidator::new(self.toaster);
        profile_validator.init(&profile);

        Editor::new(profiles_repository, profile, profile_validator)
    }
}
