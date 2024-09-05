use crate::model::chemistry::Nutrients;
use crate::model::fertilizers::Fertilizer;
use serde::{Deserialize, Serialize};

/// Amount (in grams or milliliters) of fertilizer for the solution.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct FertilizerWeight {
    id: String,
    name: String,
    liquid: bool,
    weight: f64,
    nutrients: Nutrients,
    fertilizer: Fertilizer,
}

impl FertilizerWeight {
    pub fn new(fertilizer: Fertilizer, weight: f64) -> Self {
        Self {
            id: fertilizer.id(),
            name: fertilizer.name(),
            liquid: fertilizer.liquid(),
            weight,
            nutrients: fertilizer.nutrients().multiply(weight),
            fertilizer,
        }
    }

    pub fn volume(&self, litres: f64) -> Self {
        Self {
            id: self.id.clone(),
            name: self.name.clone(),
            liquid: self.liquid,
            weight: self.weight * litres,
            nutrients: self.nutrients,
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
        self.weight / 10.
    }

    pub fn nutrients(&self) -> Nutrients {
        self.nutrients
    }

    pub fn update_amount(&mut self, amount: f64) {
        self.weight = amount;

        self.nutrients = self.fertilizer.nutrients().multiply(amount);
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

    pub fn fertilizer(&self) -> &Fertilizer {
        &self.fertilizer
    }
}

impl From<Fertilizer> for FertilizerWeight {
    fn from(fertilizer: Fertilizer) -> Self {
        Self::new(fertilizer, 0.0)
    }
}

impl From<&Fertilizer> for FertilizerWeight {
    fn from(fertilizer: &Fertilizer) -> Self {
        Self::new(fertilizer.clone(), 0.0)
    }
}

impl Into<Fertilizer> for FertilizerWeight {
    fn into(self) -> Fertilizer {
        Fertilizer::from(self.fertilizer)
    }
}
