use crate::model::chemistry::Nutrient;
use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::Profile;
use crate::model::solutions::{FertilizerWeight, NutrientResult};
use serde::{Deserialize, Serialize};
use std::ops::Index;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Solution {
    id: String,
    name: String,
    profile: Profile,
    value: Profile,
    fertilizers_weights: Vec<FertilizerWeight>,
    water_amount: usize,
}

impl Solution {
    pub fn from(profile: Profile) -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            profile,
            value: Profile::new(),
            fertilizers_weights: Vec::new(),
            water_amount: 1,
        }
    }

    pub fn new() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            profile: Profile::new(),
            value: Profile::new(),
            fertilizers_weights: Vec::new(),
            water_amount: 1,
        }
    }

    pub fn empty(fertilizers: Vec<Fertilizer>) -> Self {
        let mut result_profile = Self::new();

        for fertilizer in fertilizers {
            result_profile.add_fertilizer_weight(fertilizer, 0.0);
        }

        result_profile
    }

    pub fn add_fertilizer_weight(&mut self, fertilizer: Fertilizer, amount: f64) {
        let fertilizer_weight = FertilizerWeight::new(fertilizer, amount);

        fertilizer_weight.nutrients().iter().for_each(|nutrient| {
            self.value.add_nutrient(*nutrient);
        });

        self.fertilizers_weights.push(fertilizer_weight);
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

    pub fn set_water_amount(&mut self, water_amount: usize) {
        self.water_amount = water_amount;
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

    pub fn fertilizers(&self) -> Vec<FertilizerWeight> {
        let mut fertilizers = self
            .fertilizers_weights
            .iter()
            .map(|fertilizer_weight| {
                FertilizerWeight::new(
                    fertilizer_weight.fertilizer.clone(),
                    fertilizer_weight.weight * self.water_amount as f64,
                )
            })
            .collect::<Vec<FertilizerWeight>>();

        fertilizers.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap());

        fertilizers
    }

    pub fn water_amount(&self) -> usize {
        self.water_amount
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
