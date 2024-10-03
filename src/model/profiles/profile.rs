use super::ProfileSummary;
use crate::model::chemistry::{Nutrient, NutrientAmount, Nutrients};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Profile {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) nutrients: Nutrients,
}

impl Profile {
    pub fn extend(profile: &Self) -> Self {
        Profile {
            id: String::new(),
            name: profile.name(),
            nutrients: profile.nutrients(),
        }
    }

    pub fn is_saved(&self) -> bool {
        !self.id().is_empty()
    }

    pub fn update_nutrient_requirement(&mut self, nutrient_requirement: NutrientAmount) {
        self.nutrients[nutrient_requirement.nutrient()] = nutrient_requirement;
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn nutrients(&self) -> Nutrients {
        self.nutrients
    }

    pub fn nutrient_requirement(&self, nutrient: Nutrient) -> NutrientAmount {
        self.nutrients[nutrient]
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            nutrients: Nutrients::new(),
        }
    }
}

impl From<Nutrients> for Profile {
    fn from(nutrients: Nutrients) -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            nutrients,
        }
    }
}

impl Into<ProfileSummary> for Profile {
    fn into(self) -> ProfileSummary {
        ProfileSummary {
            id: self.id,
            name: self.name,
        }
    }
}
