use crate::model::chemistry::Volume;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Part {
    id: String,
    name: String,
    concentration: usize,
    volume: Volume,
}

impl Part {
    pub fn new(name: impl ToString) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            concentration: 100,
            volume: Volume::default(),
        }
    }

    pub fn from(id: String, name: String, concentration: usize, volume: Volume) -> Self {
        Self {
            id,
            name,
            concentration,
            volume,
        }
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

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn update_concentration(&mut self, concentration: usize) {
        self.concentration = concentration;
    }

    pub fn update_volume(&mut self, volume: Volume) {
        self.volume = volume;
    }
}
