use crate::model::chemistry::Volume;
use crate::model::fertilizers::Fertilizer;
use crate::model::solutions::{CalculationResult, FertilizerWeight};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct SelectedSet {
    fertilizers_amounts: HashMap<String, FertilizerWeight>,
    limit: usize,
    page_index: usize,
}

impl SelectedSet {
    pub fn new() -> Self {
        Self {
            fertilizers_amounts: HashMap::new(),
            limit: 8,
            page_index: 1,
        }
    }

    pub fn set_selected_fertilizers(
        &mut self,
        fertilizers_amounts: &HashMap<String, CalculationResult>,
        fertilizers: Vec<Fertilizer>,
    ) {
        fertilizers.into_iter().for_each(|fertilizer| {
            if let Some(calculation_result) = fertilizers_amounts.get(&fertilizer.id()) {
                self.fertilizers_amounts.insert(
                    fertilizer.id(),
                    FertilizerWeight::new(fertilizer, calculation_result.amount()),
                );
            }
        });
    }

    pub fn add_selected_fertilizer(&mut self, fertilizer: Fertilizer) {
        self.fertilizers_amounts
            .insert(fertilizer.id(), FertilizerWeight::new(fertilizer, 0.0));
    }

    pub fn update_selected_fertilizer(&mut self, fertilizer_id: &String, amount: f64) {
        if let Some(fertilizer) = self.fertilizers_amounts.get_mut(fertilizer_id) {
            fertilizer.update_amount(amount);
        }
    }

    pub fn exclude_fertilizer(&mut self, fertilizer_id: &String) {
        self.fertilizers_amounts.remove(fertilizer_id);
    }

    pub fn selected_fertilizers_ids(&self) -> Vec<&String> {
        self.fertilizers_amounts
            .keys()
            .map(|fertilizer_id| fertilizer_id)
            .collect()
    }

    pub fn list_fertilizers_amounts(&self) -> Vec<&FertilizerWeight> {
        self.fertilizers_amounts.values().collect()
    }

    pub fn list_selected_fertilizers(&self) -> Vec<&Fertilizer> {
        self.fertilizers_amounts
            .values()
            .map(|fertilizer_amount| fertilizer_amount.fertilizer())
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.fertilizers_amounts.is_empty()
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

    pub fn items(&self, volume: Volume) -> Vec<FertilizerWeight> {
        let start = (self.page_index - 1) * self.limit;

        let end = (self.page_index * self.limit) - 1;

        let fertilizers_amounts = self.list_fertilizers_amounts();

        if end < self.fertilizers_amounts.len() {
            return fertilizers_amounts[start..=end]
                .iter()
                .map(|fertilizer_amount| fertilizer_amount.volume(volume.to_litres()))
                .collect();
        } else {
            return fertilizers_amounts[start..]
                .iter()
                .map(|fertilizer_amount| fertilizer_amount.volume(volume.to_litres()))
                .collect();
        }
    }
}
