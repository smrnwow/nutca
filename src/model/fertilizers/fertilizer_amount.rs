use crate::model::chemistry::{Nutrients, Volume};
use crate::model::fertilizers::Fertilizer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct FertilizerAmount {
    fertilizer: Fertilizer,
    amount: f64,
    volume: Volume,
    concentration: usize,
}

impl FertilizerAmount {
    pub fn new(fertilizer: Fertilizer, amount: f64) -> Self {
        Self {
            fertilizer,
            amount,
            volume: Volume::default(),
            concentration: 1,
        }
    }

    pub fn volume(&mut self, volume: Volume) {
        self.volume = volume;
    }

    pub fn concentration(&mut self, concentration: usize) {
        self.concentration = concentration;
    }

    pub fn update_amount(&mut self, amount: f64) {
        self.amount = amount;
    }

    pub fn add(&mut self, fertilizer_amount: &FertilizerAmount) {
        self.amount += fertilizer_amount.amount_base();
    }

    pub fn amount_base(&self) -> f64 {
        self.amount
    }

    pub fn amount(&self) -> f64 {
        self.amount * self.volume.to_litres() * self.concentration as f64
    }

    pub fn units(&self) -> &str {
        match self.fertilizer.liquid {
            true => "мл",
            false => "г",
        }
    }

    pub fn fertilizer(&self) -> &Fertilizer {
        &self.fertilizer
    }

    pub fn nutrients(&self) -> Nutrients {
        self.fertilizer.nutrients().multiply(self.amount)
    }

    pub fn is_redurant(&self) -> bool {
        self.amount == 0.
    }
}

impl From<Fertilizer> for FertilizerAmount {
    fn from(fertilizer: Fertilizer) -> Self {
        Self::new(fertilizer, 0.0)
    }
}

impl Into<Fertilizer> for FertilizerAmount {
    fn into(self) -> Fertilizer {
        Fertilizer::from(self.fertilizer)
    }
}
