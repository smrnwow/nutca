use crate::model::calculation::Solver;
use crate::model::chemistry::Nutrient;
use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::Profile;
use crate::model::solutions::Solution;
use uuid::Uuid;

pub struct SolutionBuilder {
    id: String,
    name: String,
    profile: Profile,
    fertilizers: Vec<Fertilizer>,
    water_amount: usize,
}

impl SolutionBuilder {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            profile: Profile::new(),
            fertilizers: Vec::new(),
            water_amount: 1,
        }
    }

    pub fn base_on(profile: Profile) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            profile,
            fertilizers: Vec::new(),
            water_amount: 1,
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
            water_amount: 1,
        }
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

    pub fn update_water_amount(&mut self, water_amount: usize) {
        self.water_amount = water_amount;
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

    pub fn build(&self) -> Solution {
        if self.fertilizers.len() > 0 {
            let mut solution = Solver::new(self.profile.clone(), self.fertilizers.clone())
                .solve()
                .unwrap();

            solution.set_id(self.id.clone());

            solution.set_name(self.name.clone());

            solution.set_water_amount(self.water_amount);

            return solution;
        } else {
            let mut solution = Solution::empty(self.fertilizers.clone());

            solution.set_id(self.id.clone());

            solution.set_name(self.name.clone());

            solution.set_water_amount(self.water_amount);

            solution
        }
    }
}
