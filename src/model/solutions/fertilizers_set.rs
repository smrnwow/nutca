use crate::model::fertilizers::Fertilizer;
use crate::model::solutions::FertilizerWeight;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct FertilizersSet {
    page_index: usize,
    limit: usize,
    items: Vec<FertilizerWeight>,
}

impl FertilizersSet {
    pub fn new(
        water_amount: usize,
        fertilizers_weights: Vec<FertilizerWeight>,
        redurant_fertilizers: Vec<Fertilizer>,
    ) -> Self {
        let mut items: Vec<FertilizerWeight> = redurant_fertilizers
            .iter()
            .map(|fertilizer| FertilizerWeight::new(fertilizer.clone(), 0.0))
            .collect();

        let mut fertilizers_weights: Vec<FertilizerWeight> = fertilizers_weights
            .iter()
            .map(|fertilizer_weight| {
                FertilizerWeight::new(
                    fertilizer_weight.fertilizer.clone(),
                    fertilizer_weight.weight * water_amount as f64,
                )
            })
            .collect();

        fertilizers_weights.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap());

        items.extend(fertilizers_weights.into_iter());

        Self {
            page_index: 1,
            limit: 8,
            items,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.len() == 0
    }

    pub fn total(&self) -> usize {
        self.items.len()
    }

    pub fn page_index(&self) -> usize {
        self.page_index
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn paginate(&mut self, page_index: usize) {
        self.page_index = page_index;
    }

    pub fn items(&self) -> Vec<FertilizerWeight> {
        let start = (self.page_index - 1) * self.limit;

        let end = (self.page_index * self.limit) - 1;

        if end < self.items.len() {
            Vec::from(&self.items[start..=end])
        } else {
            Vec::from(&self.items[start..])
        }
    }
}
