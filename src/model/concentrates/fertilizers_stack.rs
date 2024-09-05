use crate::model::concentrates::FertilizerAmount;
use crate::model::solutions::FertilizerWeight;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct FertilizersStack {
    stack: HashMap<String, FertilizerAmount>,
}

impl FertilizersStack {
    pub fn new(fertilizers_weights: Vec<FertilizerWeight>) -> Self {
        let mut fertilizers_stack = Self {
            stack: HashMap::new(),
        };

        fertilizers_weights.iter().for_each(|fertilizer_weight| {
            fertilizers_stack.init(fertilizer_weight);
        });

        fertilizers_stack
    }

    pub fn increment(&mut self, fertilizer_id: &String, percent: usize) {
        if let Some(fertilizer_amount) = self.stack.get_mut(fertilizer_id) {
            fertilizer_amount.increment_percent(percent);
        }
    }

    pub fn decrement(&mut self, fertilizer_id: &String, percent: usize) {
        if let Some(fertilizer_amount) = self.stack.get_mut(fertilizer_id) {
            fertilizer_amount.decrement_percent(percent);
        }
    }

    pub fn list(&self) -> Vec<&FertilizerAmount> {
        self.stack
            .values()
            .filter(|fertilizer_amount| fertilizer_amount.percent() > 0)
            .collect()
    }

    fn init(&mut self, fertilizer_weight: &FertilizerWeight) {
        let fertilizer_amount = FertilizerAmount::new(
            fertilizer_weight.fertilizer().clone(),
            fertilizer_weight.weight(),
            100,
        );

        self.stack.insert(fertilizer_amount.id(), fertilizer_amount);
    }
}
