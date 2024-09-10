use super::CalculationResult;
use crate::model::chemistry::{Nutrients, Volume};
use crate::model::profiles::Profile;
use crate::model::solutions::{FertilizerWeight, Solution};
use std::collections::HashMap;
use uuid::Uuid;

pub struct SolutionBuilder {
    id: String,
    name: String,
    profile: Profile,
    fertilizers: HashMap<String, CalculationResult>,
    volume: Volume,
    composition: Nutrients,
}

impl SolutionBuilder {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            profile: Profile::default(),
            fertilizers: HashMap::new(),
            volume: Volume::default(),
            composition: Nutrients::new(),
        }
    }

    pub fn with_id(&mut self, id: Option<String>) -> &mut Self {
        if let Some(id) = id {
            self.id = id;
        }

        self
    }

    pub fn with_name(&mut self, name: String) -> &mut Self {
        self.name = name;

        self
    }

    pub fn with_volume(&mut self, volume: Volume) -> &mut Self {
        self.volume = volume;

        self
    }

    pub fn with_profile(&mut self, profile: Profile) -> &mut Self {
        self.profile = profile;

        self
    }

    pub fn with_fertilizers(&mut self, fertilizers: Vec<&FertilizerWeight>) -> &mut Self {
        fertilizers.iter().for_each(|fertilizer_weight| {
            let calculation_result = match fertilizer_weight.weight() {
                0.0 => CalculationResult::RedurantFertilizer,
                amount => CalculationResult::Calculated(amount),
            };

            self.fertilizers
                .insert(fertilizer_weight.id(), calculation_result);
        });

        self
    }

    pub fn with_fertilizer(
        &mut self,
        fertilizer_id: String,
        calculation_result: CalculationResult,
    ) -> &mut Self {
        self.fertilizers.insert(fertilizer_id, calculation_result);

        self
    }

    pub fn with_composition(&mut self, composition: Nutrients) -> &mut Self {
        self.composition = composition;

        self
    }

    pub fn build(&self) -> Solution {
        Solution {
            id: self.id.clone(),
            name: self.name.clone(),
            profile: self.profile.clone(),
            volume: self.volume,
            fertilizers: self.fertilizers.clone(),
            nutrients: self.composition,
        }
    }
}
