use crate::model::chemistry::Volume;
use crate::model::concentrates::parts::{FertilizerAmount, ManualPart};
use crate::model::fertilizers::Fertilizer;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ManualFiller {
    parts: Vec<ManualPart>,
}

impl ManualFiller {
    pub fn new() -> Self {
        Self {
            parts: vec![ManualPart::new("")],
        }
    }

    pub fn parts(&self) -> &Vec<ManualPart> {
        &self.parts
    }

    pub fn add_part(&mut self) {
        if self.parts.len() < 5 {
            self.parts.push(ManualPart::new(""));
        }
    }

    pub fn delete_part(&mut self, part_id: String) {
        let position = self.parts.iter().position(|part| *part.id() == part_id);

        if let Some(index) = position {
            self.parts.remove(index);
        }
    }

    pub fn update_part_name(&mut self, part_id: String, name: String) {
        if let Some(part) = self.find_part(&part_id) {
            part.update_name(name);
        }
    }

    pub fn update_part_concentration(&mut self, part_id: String, concentration: usize) {
        if let Some(part) = self.find_part(&part_id) {
            part.update_concentration(concentration);
        }
    }

    pub fn update_part_volume(&mut self, part_id: String, volume: Volume) {
        if let Some(part) = self.find_part(&part_id) {
            part.update_volume(volume);
        }
    }

    pub fn add_part_fertilizer(&mut self, part_id: String, fertilizer: Fertilizer, amount: f64) {
        if let Some(part) = self.find_part(&part_id) {
            part.add_fertilizer(FertilizerAmount::new(fertilizer.id(), amount));
        }
    }

    pub fn delete_part_fertilizer(&mut self, part_id: String, fertilizer_id: String) {
        if let Some(part) = self.find_part(&part_id) {
            part.delete_fertilizer(fertilizer_id);
        }
    }

    pub fn get_part(&mut self, part_id: &String) -> Option<&mut ManualPart> {
        let position = self.parts.iter().position(|part| *part.id() == *part_id);

        match position {
            Some(index) => self.parts.get_mut(index),
            None => None,
        }
    }

    fn find_part(&mut self, part_id: &String) -> Option<&mut ManualPart> {
        let position = self.parts.iter().position(|part| *part.id() == *part_id);

        match position {
            Some(index) => self.parts.get_mut(index),
            None => None,
        }
    }
}
