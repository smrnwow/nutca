use super::FertilizerPercent;
use crate::model::chemistry::Volume;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AutoPart {
    id: String,
    name: String,
    concentration: usize,
    volume: Volume,
    fertilizers: Vec<FertilizerPercent>,
}

impl AutoPart {
    pub fn new(name: impl ToString) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            concentration: 100,
            volume: Volume::default(),
            fertilizers: Vec::new(),
        }
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn update_concentration(&mut self, concentration: usize) {
        self.concentration = concentration;
    }

    pub fn update_volume(&mut self, volume: Volume) {
        self.volume = volume;
    }

    pub fn add_fertilizer(&mut self, fertilizer: FertilizerPercent) {
        self.fertilizers.push(fertilizer);
    }

    pub fn delete_fertilizer(&mut self, fertilizer_id: String) -> Option<FertilizerPercent> {
        let position = self
            .fertilizers
            .iter()
            .position(|fertilizer| *fertilizer.id() == fertilizer_id);

        if let Some(index) = position {
            return Some(self.fertilizers.remove(index));
        }

        None
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn concentration(&self) -> usize {
        self.concentration
    }

    pub fn volume(&self) -> Volume {
        self.volume
    }

    pub fn fertilizers(&self) -> Vec<FertilizerPercent> {
        let coefficient = self.concentration as f64 * self.volume.to_litres();

        self.fertilizers
            .iter()
            .map(|fertilizer| fertilizer.coefficient(coefficient))
            .collect()
    }
}
