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
}
