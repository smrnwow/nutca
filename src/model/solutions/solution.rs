use crate::model::chemistry::{Nutrient, NutrientAmount, Nutrients, Volume};
use crate::model::profiles::{Profile, ProfileBuilder};
use crate::model::solutions::{Conductivity, FertilizerWeight, FertilizersSet, NutrientResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Solution {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) profile: Profile,
    pub(super) volume: Volume,
    pub(super) fertilizers_set: FertilizersSet,
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

    pub fn fertilizers(&self) -> Vec<FertilizerWeight> {
        self.fertilizers_set.weight(self.volume)
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
        /*
        let total_ppms: f64 = self
            .nutrients
            .list()
            .iter()
            .map(|nutrient| nutrient.value())
            .sum();

        (total_ppms / 1000.) * 0.7
        */
    }

    pub fn is_empty(&self) -> bool {
        self.fertilizers_set.is_empty()
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
            fertilizers_set: FertilizersSet::default(),
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
            fertilizers_set: FertilizersSet::default(),
        }
    }
}
