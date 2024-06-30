use crate::model::chemistry::Nutrient;
use crate::model::fertilizers::Fertilizer;
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

    pub fn nutrients(&self) -> Vec<Nutrient> {
        self.fertilizer
            .nutrients()
            .iter()
            .map(|nutrient| nutrient.new(nutrient.value() * self.weight * 10.))
            .collect()
    }

    pub fn display_amount(&self) -> String {
        let units = match self.fertilizer.liquid() {
            true => String::from("мл"),
            false => String::from("г"),
        };

        format!("{:.3} {}", self.weight, units)
    }

    pub fn is_redurant(&self) -> bool {
        self.weight == 0.
    }

    pub fn macro_nutrients(&self) -> Vec<Nutrient> {
        self.fertilizer
            .macro_nutrients()
            .iter()
            .map(|nutrient| nutrient.new(nutrient.value() * self.weight * 10.))
            .collect()
    }

    pub fn nitrogen_forms(&self) -> Vec<Nutrient> {
        self.fertilizer
            .nitrogen_forms()
            .iter()
            .map(|nutrient| nutrient.new(nutrient.value() * self.weight * 10.))
            .collect()
    }

    pub fn micro_nutrients(&self) -> Vec<Nutrient> {
        self.fertilizer
            .micro_nutrients()
            .iter()
            .map(|nutrient| nutrient.new(nutrient.value() * self.weight * 10.))
            .collect()
    }
}
