use crate::model::chemistry::Nutrients;
use crate::model::fertilizers::Fertilizer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct FertilizerWeight {
    pub nutrients: Nutrients,
    id: String,
    name: String,
    liquid: bool,
    weight: f64,
    fertilizer: Fertilizer,
}

impl FertilizerWeight {
    pub fn new(fertilizer: Fertilizer, weight: f64) -> Self {
        Self {
            nutrients: fertilizer.nutrients().multiply(weight),
            id: fertilizer.id(),
            name: fertilizer.name(),
            liquid: fertilizer.liquid(),
            weight,
            fertilizer,
        }
    }

    pub fn volume(&self, litres: f64) -> Self {
        Self {
            nutrients: self.nutrients,
            id: self.id.clone(),
            name: self.name.clone(),
            liquid: self.liquid,
            weight: self.weight * litres,
            fertilizer: self.fertilizer.clone(),
        }
    }

    pub fn multiply(&self, factor: f64) -> Self {
        let weight = self.weight * factor;

        Self {
            nutrients: self.nutrients.multiply(weight),
            id: self.id.clone(),
            name: self.name.clone(),
            liquid: self.liquid,
            weight,
            fertilizer: self.fertilizer.clone(),
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }

    pub fn amount(&self) -> String {
        let units = match self.liquid {
            true => String::from("мл"),
            false => String::from("г"),
        };

        format!("{:.3} {}", self.weight / 10., units)
    }

    pub fn is_redurant(&self) -> bool {
        self.weight == 0.
    }
}

impl Into<Fertilizer> for FertilizerWeight {
    fn into(self) -> Fertilizer {
        Fertilizer::from(self.fertilizer)
    }
}

impl From<Fertilizer> for FertilizerWeight {
    fn from(fertilizer: Fertilizer) -> Self {
        Self::new(fertilizer, 0.0)
    }
}
