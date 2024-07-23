use crate::model::chemistry::{Nutrients, Volume};
use crate::model::fertilizers::Fertilizer;
use crate::model::solutions::FertilizerWeight;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct FertilizersSet {
    fertilizers: Vec<FertilizerWeight>,
}

impl FertilizersSet {
    pub fn new(fertilizers_weights: Vec<FertilizerWeight>) -> Self {
        Self {
            fertilizers: fertilizers_weights,
        }
    }

    pub fn empty() -> Self {
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

    pub fn nutrients(&self) -> Nutrients {
        let mut nutrients = Nutrients::new();

        self.list().iter().for_each(|fertilizer_weight| {
            fertilizer_weight
                .nutrients
                .list()
                .iter()
                .for_each(|nutrient_amount| {
                    nutrients.add(*nutrient_amount);
                });
        });

        nutrients
    }

    pub fn list(&self) -> Vec<FertilizerWeight> {
        let mut fertilizers_weights = Vec::new();

        self.fertilizers
            .iter()
            .filter(|fertilizer_weight| fertilizer_weight.weight() == 0.0)
            .for_each(|fertilizer_weight| fertilizers_weights.push(fertilizer_weight.clone()));

        let mut non_nullish: Vec<&FertilizerWeight> = self
            .fertilizers
            .iter()
            .filter(|fertilizer_weight| fertilizer_weight.weight() > 0.0)
            .collect();

        non_nullish.sort_by(|a, b| b.weight().partial_cmp(&a.weight()).unwrap());

        non_nullish
            .iter()
            .map(|fertilizer_weight| *fertilizer_weight)
            .for_each(|fertilizer_weight| fertilizers_weights.push(fertilizer_weight.clone()));

        fertilizers_weights
    }

    pub fn weight(&self, volume: Volume) -> Vec<FertilizerWeight> {
        let mut fertilizers_weights = Vec::new();

        self.fertilizers
            .iter()
            .filter(|fertilizer_weight| fertilizer_weight.weight() == 0.0)
            .for_each(|fertilizer_weight| fertilizers_weights.push(fertilizer_weight.clone()));

        let mut non_nullish: Vec<&FertilizerWeight> = self
            .fertilizers
            .iter()
            .filter(|fertilizer_weight| fertilizer_weight.weight() > 0.0)
            .collect();

        non_nullish.sort_by(|a, b| b.weight().partial_cmp(&a.weight()).unwrap());

        non_nullish
            .iter()
            .map(|fertilizer_weight| fertilizer_weight.volume(volume.to_litres()))
            .for_each(|fertilizer_weight| fertilizers_weights.push(fertilizer_weight));

        fertilizers_weights
    }
}

impl From<Vec<Fertilizer>> for FertilizersSet {
    fn from(fertilizers: Vec<Fertilizer>) -> Self {
        let mut fertilizers_set = Self::empty();

        for fertilizer in fertilizers {
            fertilizers_set.add_fertilizer_weight(FertilizerWeight::from(fertilizer));
        }

        fertilizers_set
    }
}
