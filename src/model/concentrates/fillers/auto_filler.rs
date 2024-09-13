use crate::model::chemistry::Volume;
use crate::model::concentrates::parts::{AutoPart, FertilizerPercent};
use crate::model::concentrates::DefaultConcentrate;
use crate::model::solutions::FertilizerWeight;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct AutoFiller {
    solution_id: String,
    stack: HashMap<String, usize>,
    parts: Vec<AutoPart>,
}

impl AutoFiller {
    pub fn new(solution_id: String, fertilizers: Vec<&FertilizerWeight>) -> Self {
        let mut stack: HashMap<String, usize> = HashMap::new();

        fertilizers.iter().for_each(|fertilizer_weight| {
            stack.insert(fertilizer_weight.id(), 100);
        });

        let parts: Vec<AutoPart> = DefaultConcentrate::new(fertilizers)
            .parts()
            .iter()
            .map(|part| (*part).clone())
            .collect();

        parts.iter().for_each(|part| {
            part.fertilizers().iter().for_each(|fertilizer| {
                if let Some(usage_percent) = stack.get_mut(&fertilizer.id()) {
                    *usage_percent -= fertilizer.percent();
                }
            });
        });

        Self {
            solution_id,
            parts,
            stack,
        }
    }

    pub fn solution_id(&self) -> &String {
        &self.solution_id
    }

    pub fn parts(&self) -> &Vec<AutoPart> {
        &self.parts
    }

    pub fn stack(&self) -> &HashMap<String, usize> {
        &self.stack
    }

    pub fn add_part(&mut self) {
        if self.parts.len() < 5 {
            self.parts.push(AutoPart::new(""));
        }
    }

    pub fn delete_part(&mut self, part_id: String) {
        let position = self.parts.iter().position(|part| *part.id() == part_id);

        if let Some(index) = position {
            let part = self.parts.remove(index);

            part.fertilizers().iter().for_each(|fertilizer| {
                self.increment_percent(&fertilizer.id(), fertilizer.percent());
            });
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

    pub fn add_fertilizer(&mut self, part_id: String, fertilizer_id: String, percent: usize) {
        if let Some(part) = self.find_part(&part_id) {
            part.add_fertilizer(FertilizerPercent::new(fertilizer_id.clone(), percent));

            self.decrement_percent(&fertilizer_id, percent);
        }
    }

    pub fn delete_fertilizer(&mut self, part_id: String, fertilizer_id: String) {
        if let Some(part) = self.find_part(&part_id) {
            if let Some(fertilizer_amount) = part.delete_fertilizer(fertilizer_id) {
                self.increment_percent(&fertilizer_amount.id(), fertilizer_amount.percent());
            }
        }
    }

    pub fn get_part(&mut self, part_id: &String) -> Option<&mut AutoPart> {
        let position = self.parts.iter().position(|part| *part.id() == *part_id);

        match position {
            Some(index) => self.parts.get_mut(index),
            None => None,
        }
    }

    fn find_part(&mut self, part_id: &String) -> Option<&mut AutoPart> {
        let position = self.parts.iter().position(|part| *part.id() == *part_id);

        match position {
            Some(index) => self.parts.get_mut(index),
            None => None,
        }
    }

    fn decrement_percent(&mut self, fertilizer_id: &String, percent: usize) {
        if let Some(usage_percent) = self.stack.get_mut(fertilizer_id) {
            *usage_percent -= percent;
        }
    }

    fn increment_percent(&mut self, fertilizer_id: &String, percent: usize) {
        if let Some(usage_percent) = self.stack.get_mut(fertilizer_id) {
            *usage_percent += percent;
        }
    }
}
