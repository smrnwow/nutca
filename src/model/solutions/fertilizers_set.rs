use crate::model::fertilizers::Fertilizer;
use crate::model::solutions::FertilizerWeight;
use crate::model::{calculation::Amount, chemistry::Volume};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct FertilizersSet {
    fertilizers: Vec<FertilizerWeight>,
}

impl FertilizersSet {
    pub fn new() -> Self {
        Self {
            fertilizers: Vec::new(),
        }
    }

    pub fn add_fertilizer_weight(&mut self, fertilizer_weight: FertilizerWeight) {
        self.fertilizers.push(fertilizer_weight);
    }

    pub fn is_empty(&self) -> bool {
        self.fertilizers.len() == 0
    }

    pub fn fertilizers(&self) -> Vec<Fertilizer> {
        self.fertilizers
            .iter()
            .map(|fertilizer_weight| fertilizer_weight.clone().into())
            .collect()
    }

    pub fn list(&self) -> Vec<FertilizerWeight> {
        self.fertilizers.clone()
    }

    pub fn weight(&self, volume: Volume) -> Vec<FertilizerWeight> {
        self.fertilizers
            .iter()
            .map(|fertilizer_weight| fertilizer_weight.multiply(volume.to_litres()))
            .collect()
    }
}

impl From<Vec<Fertilizer>> for FertilizersSet {
    fn from(fertilizers: Vec<Fertilizer>) -> Self {
        let mut fertilizers_set = Self::new();

        for fertilizer in fertilizers {
            fertilizers_set.add_fertilizer_weight(FertilizerWeight::from(fertilizer));
        }

        fertilizers_set
    }
}

impl From<Vec<Amount>> for FertilizersSet {
    fn from(amounts: Vec<Amount>) -> Self {
        let mut fertilizers_set = Self::new();

        for amount in amounts {
            fertilizers_set.add_fertilizer_weight(FertilizerWeight::from(amount));
        }

        fertilizers_set
    }
}
