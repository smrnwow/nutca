use crate::model::calculation::Solver;
use crate::model::chemistry::{NutrientAmount, Nutrients, Volume};
use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::{Profile, ProfileBuilder};
use crate::model::solutions::{FertilizersSet, Solution};
use crate::model::Error;
use uuid::Uuid;

pub struct SolutionBuilder {
    id: String,
    name: String,
    profile_builder: ProfileBuilder,
    fertilizers: Vec<Fertilizer>,
    volume: Volume,
}

impl SolutionBuilder {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            profile_builder: ProfileBuilder::new(),
            fertilizers: Vec::new(),
            volume: Volume::default(),
        }
    }

    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = name;

        self
    }

    pub fn volume(&mut self, volume: Volume) -> &mut Self {
        self.volume = volume;

        self
    }

    pub fn profile(&mut self, profile: Option<Profile>) -> &mut Self {
        match profile {
            Some(profile) => self.profile_builder = ProfileBuilder::from(profile),
            None => self.profile_builder = ProfileBuilder::new(),
        };

        self
    }

    pub fn nutrient_requirement(&mut self, nutrient_amount: NutrientAmount) -> &mut Self {
        if self.profile_builder.is_saved() {
            self.profile_builder = ProfileBuilder::from(self.profile_builder.build());
        }

        self.profile_builder.nutrient_requirement(nutrient_amount);

        self
    }

    pub fn add_fertilizer(&mut self, fertilizer: Fertilizer) -> &mut Self {
        self.fertilizers.push(fertilizer);

        self
    }

    pub fn remove_fertilizer(&mut self, fertilizer_id: String) -> &mut Self {
        self.fertilizers = self
            .fertilizers
            .iter()
            .filter(|fertilizer| fertilizer.id() != fertilizer_id)
            .map(|fertilizer| fertilizer.clone())
            .collect();

        self
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
        let profile = self.profile_builder.build();

        let amounts = Solver::new(&profile, self.fertilizers.clone()).solve();

        let fertilizers_set = FertilizersSet::from(amounts);

        let mut nutrients = Nutrients::new();

        fertilizers_set.list().iter().for_each(|fertilizer_weight| {
            fertilizer_weight
                .nutrients
                .list()
                .iter()
                .for_each(|nutrient_amount| {
                    nutrients.add(*nutrient_amount);
                });
        });

        Solution {
            id: self.id.clone(),
            name: self.name.clone(),
            profile,
            volume: self.volume,
            fertilizers_set,
            nutrients,
        }
    }
}

impl From<Solution> for SolutionBuilder {
    fn from(solution: Solution) -> Self {
        Self {
            id: solution.id(),
            name: solution.name(),
            profile_builder: ProfileBuilder::from(solution.profile()),
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
            profile_builder: ProfileBuilder::from(profile),
            fertilizers: Vec::new(),
            volume: Volume::default(),
        }
    }
}
