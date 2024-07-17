use crate::model::calculation::Solver;
use crate::model::chemistry::{NutrientAmount, Volume};
use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::Profile;
use crate::model::solutions::{FertilizersSet, Solution};
use crate::model::Error;
use uuid::Uuid;

pub struct SolutionBuilder {
    id: String,
    name: String,
    profile: Profile,
    fertilizers: Vec<Fertilizer>,
    volume: Volume,
}

impl SolutionBuilder {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            profile: Profile::new(),
            fertilizers: Vec::new(),
            volume: Volume::default(),
        }
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn update_volume(&mut self, volume: Volume) {
        self.volume = volume;
    }

    pub fn update_profile(&mut self, profile: Option<Profile>) {
        match profile {
            Some(profile) => self.profile = profile,
            None => self.profile = Profile::new(),
        }
    }

    pub fn update_profile_nutrient(&mut self, nutrient: NutrientAmount) {
        if self.profile.id().len() > 0 {
            self.profile = Profile::from_another(self.profile.clone());
        }

        self.profile.nutrients.set(nutrient);
    }

    pub fn add_fertilizer(&mut self, fertilizer: Fertilizer) {
        self.fertilizers.push(fertilizer);
    }

    pub fn remove_fertilizer(&mut self, fertilizer_id: String) {
        self.fertilizers = self
            .fertilizers
            .iter()
            .filter(|fertilizer| fertilizer.id() != fertilizer_id)
            .map(|fertilizer| fertilizer.clone())
            .collect();
    }

    pub fn profile(&self) -> Profile {
        self.profile.clone()
    }

    pub fn validate(&self) -> Vec<Error> {
        let mut errors = Vec::new();

        if self.name.len() == 0 {
            errors.push(Error::SolutionNameEmpty);
        }

        if self.name.len() > 100 {
            errors.push(Error::SolutionNameTooLong);
        }

        errors
    }

    pub fn build(&self) -> Solution {
        let amounts = Solver::new(self.profile.clone(), self.fertilizers.clone()).solve();

        Solution::from(self.profile.clone())
            .with_fertilizers_set(FertilizersSet::from(amounts))
            .with_id(self.id.clone())
            .with_name(self.name.clone())
            .with_volume(self.volume)
    }
}

impl From<Solution> for SolutionBuilder {
    fn from(solution: Solution) -> Self {
        Self {
            id: solution.id(),
            name: solution.name(),
            profile: solution.profile(),
            fertilizers: solution.fertilizers_set.fertilizers(),
            volume: Volume::default(),
        }
    }
}

impl From<Profile> for SolutionBuilder {
    fn from(profile: Profile) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            profile,
            fertilizers: Vec::new(),
            volume: Volume::default(),
        }
    }
}
