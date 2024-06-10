use crate::model::{
    chemistry::{NitrogenForm, NutrientAmount},
    fertilizers::Fertilizer,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct FertilizerWeight {
    pub fertilizer: Fertilizer,
    pub weight: f64,
}

impl FertilizerWeight {
    pub fn new(fertilizer: Fertilizer, weight: f64) -> Self {
        Self { fertilizer, weight }
    }

    pub fn nutrients(&self) -> Vec<NutrientAmount> {
        self.fertilizer
            .nutrients()
            .iter()
            .map(|nutrient_amount| nutrient_amount.new(nutrient_amount.value() * self.weight))
            .collect()
    }

    pub fn nitrogen_forms(&self) -> Vec<NitrogenForm> {
        self.fertilizer
            .nitrogen_forms()
            .iter()
            .map(|nitrogen_form| nitrogen_form.new(nitrogen_form.value() * self.weight))
            .collect()
    }
}
