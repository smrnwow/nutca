use super::Part;
use crate::model::solutions::FertilizerWeight;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct CompositionFromFertilizers {
    distribution: HashMap<String, HashMap<String, FertilizerWeight>>,
}

impl CompositionFromFertilizers {
    pub fn new(distribution: HashMap<String, HashMap<String, FertilizerWeight>>) -> Self {
        Self { distribution }
    }

    pub fn update_fertilizer_amount(&mut self, part_id: &String, fertilizer: FertilizerWeight) {
        match self.distribution.get_mut(part_id) {
            Some(part_distribution) => match part_distribution.get_mut(&fertilizer.id()) {
                Some(fertilizer_amount) => {
                    let amount = fertilizer_amount.weight() + fertilizer.weight();

                    fertilizer_amount.update_amount(amount);
                }

                None => {
                    part_distribution.insert(fertilizer.id(), fertilizer);
                }
            },

            None => {
                let mut part_distribution = HashMap::new();

                part_distribution.insert(fertilizer.id(), fertilizer);

                self.distribution.insert(part_id.clone(), part_distribution);
            }
        }
    }

    pub fn remove_fertilizer(&mut self, part_id: &String, fertilizer_id: &String) {
        if let Some(part_distribution) = self.distribution.get_mut(part_id) {
            part_distribution.remove(fertilizer_id);
        }
    }

    pub fn remove_part(&mut self, part_id: &String) {
        self.distribution.remove(part_id);
    }

    pub fn fertilizers_by_part(&self, part: &Part) -> Vec<FertilizerWeight> {
        match self.distribution.get(part.id()) {
            Some(part_distribution) => part_distribution
                .values()
                .map(|fertilizer| fertilizer.volume(part.volume().to_litres()))
                .collect(),

            None => Vec::new(),
        }
    }

    pub fn distribution(&self) -> &HashMap<String, HashMap<String, FertilizerWeight>> {
        &self.distribution
    }
}

impl Default for CompositionFromFertilizers {
    fn default() -> Self {
        Self {
            distribution: HashMap::new(),
        }
    }
}
