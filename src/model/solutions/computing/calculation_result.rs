use super::FertilizerPortion;
use std::collections::HashMap;

pub struct CalculationResult {
    fertilizers: HashMap<String, FertilizerPortion>,
}

impl CalculationResult {
    pub fn from(portions: Vec<(String, FertilizerPortion)>) -> Self {
        let mut fertilizers = HashMap::new();

        portions.into_iter().for_each(|(fertilizer_id, portion)| {
            fertilizers.insert(fertilizer_id, portion);
        });

        Self { fertilizers }
    }

    pub fn portions(&self) -> Vec<(&str, FertilizerPortion)> {
        self.fertilizers
            .iter()
            .map(|(fertilizer_id, portion)| (fertilizer_id.as_str(), *portion))
            .collect()
    }
}
