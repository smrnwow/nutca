use crate::model::chemistry::{Nutrient, NutrientAmount, Nutrients};
use crate::model::profiles::Profile;

#[derive(Clone, Debug, PartialEq)]
pub struct ProfileRequirement {
    profile: Option<(Profile, String)>,
    nutrients: Nutrients,
}

impl ProfileRequirement {
    pub fn new() -> Self {
        Self {
            profile: None,
            nutrients: Nutrients::new(),
        }
    }

    pub fn set_profile(&mut self, profile: Profile) {
        self.nutrients = *profile.first_stage().nutrients();

        let stage_id = profile.first_stage().id().to_string();

        self.profile = Some((profile, stage_id));
    }

    pub fn change_profile_stage(&mut self, stage_id: String) {
        if let Some(profile) = &mut self.profile {
            if let Some(stage) = profile.0.stage(&stage_id) {
                self.nutrients = *stage.nutrients();

                profile.1 = stage.id().to_string();
            }
        }
    }

    pub fn drop_profile(&mut self) {
        self.profile = None;
    }

    pub fn update_nutrient(&mut self, nutrient_amount: NutrientAmount) {
        if let Some(_) = &mut self.profile {
            self.profile = None;
        }

        self.nutrients.set(nutrient_amount);
    }

    pub fn profile(&self) -> Option<&(Profile, String)> {
        self.profile.as_ref()
    }

    pub fn nutrients(&self) -> &Nutrients {
        &self.nutrients
    }

    pub fn value_of(&self, nutrient: Nutrient) -> f64 {
        self.nutrients.value_of(nutrient).value()
    }
}

impl From<(Profile, String)> for ProfileRequirement {
    fn from(requirement: (Profile, String)) -> Self {
        let (profile, stage_id) = requirement;

        let mut profile_requirement = Self::new();

        profile_requirement.set_profile(profile);

        profile_requirement.change_profile_stage(stage_id);

        profile_requirement
    }
}

impl From<Nutrients> for ProfileRequirement {
    fn from(nutrients: Nutrients) -> Self {
        Self {
            profile: None,
            nutrients,
        }
    }
}
