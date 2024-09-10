use super::CalculationResult;
use crate::model::chemistry::{Nutrient, NutrientAmount, Nutrients, Volume};
use crate::model::profiles::{Profile, ProfileBuilder};
use crate::model::solutions::{Conductivity, NutrientResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Solution {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) profile: Profile,
    pub(super) volume: Volume,
    pub(super) fertilizers: HashMap<String, CalculationResult>,
    pub(super) nutrients: Nutrients,
}

impl Solution {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn profile(&self) -> Profile {
        self.profile.clone()
    }

    pub fn fertilizers(&self) -> &HashMap<String, CalculationResult> {
        &self.fertilizers
    }

    pub fn fertilizer(&self, fertilizer_id: &String) -> Option<CalculationResult> {
        match self.fertilizers.get(fertilizer_id) {
            Some(calculation_result) => Some(*calculation_result),
            None => None,
        }
    }

    pub fn volume(&self) -> Volume {
        self.volume
    }

    pub fn composition(&self) -> Nutrients {
        self.nutrients
    }

    pub fn nutrient_value(&self, nutrient: Nutrient) -> NutrientAmount {
        self.nutrients.value_of(nutrient)
    }

    pub fn nutrient_diff(&self, nutrient: Nutrient) -> NutrientResult {
        let nutrient_value = self.nutrients.value_of(nutrient).value();

        NutrientResult::new(
            self.profile.nutrient_requirement(nutrient).value(),
            format!("{:.3}", nutrient_value).parse().unwrap(),
        )
    }

    pub fn ec(&self) -> f64 {
        Conductivity::new(self.nutrients).conductivity()
    }

    pub fn is_empty(&self) -> bool {
        self.fertilizers.is_empty()
    }
}

impl From<Profile> for Solution {
    fn from(profile: Profile) -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            profile,
            nutrients: Nutrients::new(),
            volume: Volume::default(),
            fertilizers: HashMap::new(),
        }
    }
}

impl Default for Solution {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            profile: ProfileBuilder::new().build(),
            nutrients: Nutrients::new(),
            volume: Volume::default(),
            fertilizers: HashMap::new(),
        }
    }
}
