use crate::model::calculation::Solver;
use crate::model::chemistry::Nutrient;
use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::Profile;
use crate::model::solutions::{Solution, SolutionError};
use uuid::Uuid;

pub struct SolutionBuilder {
    id: String,
    name: String,
    profile: Profile,
    fertilizers: Vec<Fertilizer>,
    volume: usize,
    saved: bool,
}

impl SolutionBuilder {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            profile: Profile::new(),
            fertilizers: Vec::new(),
            volume: 1,
            saved: false,
        }
    }

    pub fn base_on(profile: Profile) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            profile,
            fertilizers: Vec::new(),
            volume: 1,
            saved: false,
        }
    }

    pub fn from(solution: Solution) -> Self {
        Self {
            id: solution.id(),
            name: solution.name(),
            profile: solution.profile(),
            fertilizers: solution
                .fertilizers()
                .iter()
                .map(|fertilizer_weight| fertilizer_weight.fertilizer.clone())
                .collect(),
            volume: 1,
            saved: false,
        }
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn update_volume(&mut self, volume: usize) {
        self.volume = volume;
    }

    pub fn update_profile(&mut self, profile: Option<Profile>) {
        match profile {
            Some(profile) => self.profile = profile,
            None => self.profile = Profile::new(),
        }
    }

    pub fn update_profile_nutrient(&mut self, nutrient: Nutrient) {
        if self.profile.id().len() > 0 {
            self.profile = Profile::from_another(self.profile.clone());
        }

        self.profile.set_nutrient(nutrient);
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

    pub fn fertilizers(&self) -> Vec<String> {
        self.fertilizers
            .iter()
            .map(|fertilizer| fertilizer.id())
            .collect()
    }

    pub fn validate(&self) -> SolutionError {
        let mut solution_error = SolutionError::new();

        if self.saved {
            if self.name.len() == 0 {
                solution_error.set_name("не заполнено");
            }
        }

        solution_error
    }

    pub fn save(&mut self) {
        self.saved = true;
    }

    pub fn build(&self) -> Solution {
        let mut solution = {
            if self.fertilizers.len() > 0 {
                Solver::new(self.profile.clone(), self.fertilizers.clone())
                    .solve()
                    .unwrap()
            } else {
                Solution::empty(self.fertilizers.clone())
            }
        };

        solution.set_id(self.id.clone());

        solution.set_name(self.name.clone());

        solution.set_water_amount(self.volume);

        return solution;
    }
}
