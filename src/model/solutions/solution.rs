use super::FertilizerWeight;
use crate::model::chemistry::{NitrogenForm, NutrientAmount};
use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::Profile;
use serde::{Deserialize, Serialize};
use std::ops::Index;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Solution {
    id: String,
    name: String,
    profile: Profile,
    value: Profile,
    fertilizers_weights: Vec<FertilizerWeight>,
}

impl Solution {
    pub fn from(profile: Profile) -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            profile,
            value: Profile::new(),
            fertilizers_weights: Vec::new(),
        }
    }

    pub fn new() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            profile: Profile::new(),
            value: Profile::new(),
            fertilizers_weights: Vec::new(),
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

        fertilizer_weight
            .nitrogen_forms()
            .iter()
            .for_each(|nitrogen_form| {
                self.value.add_nitrogen_form(*nitrogen_form);
            });

        self.fertilizers_weights.push(fertilizer_weight);
    }

    pub fn create_id(&mut self) {
        self.id = Uuid::new_v4().to_string();
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
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

    pub fn fertilizers(&self) -> &Vec<FertilizerWeight> {
        &self.fertilizers_weights
    }
}

impl Index<NutrientAmount> for Solution {
    type Output = NutrientAmount;

    fn index(&self, nutrient_amount: NutrientAmount) -> &Self::Output {
        &self.value[nutrient_amount]
    }
}

impl Index<NitrogenForm> for Solution {
    type Output = NitrogenForm;

    fn index(&self, nitrogen_form: NitrogenForm) -> &Self::Output {
        &self.value[nitrogen_form]
    }
}
