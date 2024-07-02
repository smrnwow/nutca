use crate::model::chemistry::{Nutrient, Volume};
use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::Profile;
use crate::model::solutions::{FertilizerWeight, FertilizersSet, NutrientResult};
use serde::{Deserialize, Serialize};
use std::ops::Index;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Solution {
    id: String,
    name: String,
    profile: Profile,
    value: Profile,
    fertilizers_weights: Vec<FertilizerWeight>,
    redurant_fertilizers: Vec<Fertilizer>,
    volume: Volume,
}

impl Solution {
    pub fn from(profile: Profile) -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            profile,
            value: Profile::new(),
            fertilizers_weights: Vec::new(),
            redurant_fertilizers: Vec::new(),
            volume: Volume::default(),
        }
    }

    pub fn new() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            profile: Profile::new(),
            value: Profile::new(),
            fertilizers_weights: Vec::new(),
            redurant_fertilizers: Vec::new(),
            volume: Volume::default(),
        }
    }

    pub fn empty(fertilizers: Vec<Fertilizer>) -> Self {
        let mut solution = Self::new();

        for fertilizer in fertilizers {
            solution.add_fertilizer_weight(fertilizer, 0.0);
        }

        solution
    }

    pub fn add_fertilizer_weight(&mut self, fertilizer: Fertilizer, amount: f64) {
        if amount <= 0.0 {
            self.add_redurant_fertilizer(fertilizer);
        } else {
            let fertilizer_weight = FertilizerWeight::new(fertilizer, amount);

            fertilizer_weight.nutrients().iter().for_each(|nutrient| {
                self.value.add_nutrient(*nutrient);
            });

            self.fertilizers_weights.push(fertilizer_weight);
        }
    }

    pub fn add_redurant_fertilizer(&mut self, fertilizer: Fertilizer) {
        self.redurant_fertilizers.push(fertilizer);
    }

    pub fn nutrient_result(&self, nutrient: Nutrient) -> NutrientResult {
        let result_value = format!("{:.3}", self.value[nutrient].value())
            .parse()
            .unwrap();

        NutrientResult::new(self.profile[nutrient].value(), result_value)
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_volume(&mut self, volume: Volume) {
        self.volume = volume;
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
        self.fertilizers_weights.len() == 0
    }

    pub fn fertilizers_set(&self) -> FertilizersSet {
        FertilizersSet::new(
            self.volume,
            self.fertilizers_weights.clone(),
            self.redurant_fertilizers.clone(),
        )
    }

    pub fn fertilizers(&self) -> Vec<FertilizerWeight> {
        let mut fertilizers_weights = self
            .fertilizers_weights
            .iter()
            .map(|fertilizer_weight| {
                FertilizerWeight::new(
                    fertilizer_weight.fertilizer.clone(),
                    fertilizer_weight.weight * self.volume.value() as f64,
                )
            })
            .collect::<Vec<FertilizerWeight>>();

        fertilizers_weights.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap());

        let mut redurant_fertilizers = self
            .redurant_fertilizers
            .iter()
            .map(|fertilizer| FertilizerWeight::new(fertilizer.clone(), 0.0))
            .collect::<Vec<FertilizerWeight>>();

        redurant_fertilizers.extend(fertilizers_weights.into_iter());

        redurant_fertilizers
    }

    pub fn volume(&self) -> Volume {
        self.volume
    }

    pub fn ec(&self) -> f64 {
        let total_ppms: f64 = self
            .value
            .nutrients()
            .iter()
            .map(|nutrient| nutrient.value())
            .sum();

        (total_ppms / 1000.) * 0.7
    }
}

impl Index<Nutrient> for Solution {
    type Output = Nutrient;

    fn index(&self, nutrient: Nutrient) -> &Self::Output {
        &self.value[nutrient]
    }
}
