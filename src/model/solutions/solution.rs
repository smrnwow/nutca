use crate::model::chemistry::{Nutrient, Nutrients, Volume};
use crate::model::profiles::Profile;
use crate::model::solutions::{FertilizersSet, NutrientResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Solution {
    pub nutrients: Nutrients,
    pub fertilizers_set: FertilizersSet,
    id: String,
    name: String,
    profile: Profile,
    volume: Volume,
}

impl Solution {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            profile: Profile::new(),
            nutrients: Nutrients::new(),
            volume: Volume::default(),
            fertilizers_set: FertilizersSet::new(),
        }
    }

    pub fn with_id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_volume(mut self, volume: Volume) -> Self {
        self.volume = volume;
        self
    }

    pub fn with_fertilizers_set(mut self, fertilizers_set: FertilizersSet) -> Self {
        self.fertilizers_set = fertilizers_set;

        self.fertilizers_set
            .list()
            .iter()
            .for_each(|fertilizer_weight| {
                fertilizer_weight
                    .nutrients
                    .list()
                    .iter()
                    .for_each(|nutrient_amount| {
                        self.nutrients.add(*nutrient_amount);
                    });
            });

        self
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn profile(&self) -> Profile {
        self.profile.clone()
    }

    pub fn is_empty(&self) -> bool {
        self.fertilizers_set.is_empty()
    }

    pub fn volume(&self) -> Volume {
        self.volume
    }

    pub fn nutrient_result(&self, nutrient: Nutrient) -> NutrientResult {
        let required_amount = self.profile.nutrients[nutrient].value();

        let result_amount = self.nutrients[nutrient].value();

        NutrientResult::new(
            required_amount,
            format!("{:.3}", result_amount).parse().unwrap(),
        )
    }

    pub fn ec(&self) -> f64 {
        let total_ppms: f64 = self
            .nutrients
            .list()
            .iter()
            .map(|nutrient| nutrient.value())
            .sum();

        (total_ppms / 1000.) * 0.7
    }
}

impl From<Profile> for Solution {
    fn from(profile: Profile) -> Self {
        Self {
            nutrients: Nutrients::new(),
            fertilizers_set: FertilizersSet::new(),
            id: String::new(),
            name: String::new(),
            profile,
            volume: Volume::default(),
        }
    }
}
