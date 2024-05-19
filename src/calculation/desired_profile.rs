use super::requirement::Requirement;
use crate::chemistry::Nutrient;
use ellp::problem::VariableId;
use std::collections::HashMap;

pub struct DesiredProfile {
    requirements: HashMap<Nutrient, Requirement>,
}

impl DesiredProfile {
    pub fn new(source: Vec<Requirement>) -> Self {
        let mut requirements: HashMap<Nutrient, Requirement> = HashMap::new();

        source.iter().for_each(|requirement| {
            requirements.insert(requirement.symbol(), requirement.clone());
        });

        Self { requirements }
    }

    pub fn add_coefficient(&mut self, symbol: &Nutrient, coefficient: (VariableId, f64)) {
        if let Some(requirement) = self.requirements.get_mut(symbol) {
            requirement.add_coefficient(coefficient);
        }
    }

    pub fn requirements(&self) -> Vec<&Requirement> {
        self.requirements.values().collect()
    }
}
