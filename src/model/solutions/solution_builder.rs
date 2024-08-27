use crate::model::chemistry::{Nutrients, Volume};
use crate::model::profiles::Profile;
use crate::model::solutions::{FertilizerWeight, FertilizersSet, Solution};
use uuid::Uuid;

pub struct SolutionBuilder {
    id: String,
    name: String,
    profile: Profile,
    fertilizer_amounts: Vec<FertilizerWeight>,
    volume: Volume,
    composition: Nutrients,
}

impl SolutionBuilder {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            profile: Profile::default(),
            fertilizer_amounts: Vec::new(),
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

    pub fn with_fertilizers_set(&mut self, fertilizers_set: FertilizersSet) -> &mut Self {
        self.fertilizer_amounts = fertilizers_set.list().clone();

        self
    }

    pub fn with_fertilizer_amount(&mut self, fertilizer_amount: FertilizerWeight) -> &mut Self {
        self.fertilizer_amounts.push(fertilizer_amount);

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
            fertilizers_set: FertilizersSet::new(self.fertilizer_amounts.clone()),
            nutrients: self.composition,
        }
    }
}
