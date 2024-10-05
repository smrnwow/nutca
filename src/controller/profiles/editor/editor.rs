use super::ProfileValidator;
use crate::model::chemistry::NutrientAmount;
use crate::model::profiles::{Profile, Stage};
use crate::repository::ProfilesRepository;
use crate::ui::router::Route;
use dioxus_router::prelude::*;

pub struct Editor {
    profiles_repository: ProfilesRepository,
    profile: Profile,
    profile_validator: ProfileValidator,
}

impl Editor {
    pub fn new(
        profiles_repository: ProfilesRepository,
        profile: Profile,
        profile_validator: ProfileValidator,
    ) -> Self {
        Self {
            profiles_repository,
            profile,
            profile_validator,
        }
    }

    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    pub fn profile_validator(&self) -> &ProfileValidator {
        &self.profile_validator
    }

    pub fn update_name(&mut self, name: String) {
        self.profile_validator.validate_name(&name);

        self.profile.update_name(name);
    }

    pub fn add_stage(&mut self) {
        self.profile.add_stage(Stage::new());
    }

    pub fn update_stage_name(&mut self, stage_id: String, name: String) {
        self.profile.update_stage_name(&stage_id, name);
    }

    pub fn update_nutrient(&mut self, stage_id: String, nutrient_amount: NutrientAmount) {
        self.profile.update_nutrient(&stage_id, nutrient_amount);
    }

    pub fn remove_stage(&mut self, stage_id: String) {
        self.profile.remove_stage(&stage_id);
    }

    pub fn create(&mut self) {
        if self.profile_validator.validate() {
            if let Ok(_) = self.profiles_repository.create(self.profile.clone()) {
                navigator().push(Route::ProfilesMainPage {});
            }
        }
    }

    pub fn update(&mut self) {
        if self.profile_validator.validate() {
            if let Ok(_) = self.profiles_repository.update(self.profile.clone()) {
                navigator().push(Route::ProfilesMainPage {});
            }
        }
    }

    pub fn back(&self) {
        navigator().push(Route::ProfilesMainPage {});
    }
}
