use crate::model::calculation::Calculation;
use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::{Component, Profile};
use crate::model::solutions::Solution;
use uuid::Uuid;

pub struct SolutionBuilder {
    id: String,
    name: String,
    profile: Profile,
    fertilizers: Vec<Fertilizer>,
}

impl SolutionBuilder {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            profile: Profile::new(),
            fertilizers: Vec::new(),
        }
    }

    pub fn base_on(profile: Profile) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            profile,
            fertilizers: Vec::new(),
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
        }
    }

    pub fn update_profile(&mut self, profile: Option<Profile>) {
        match profile {
            Some(profile) => self.profile = profile,
            None => self.profile = Profile::new(),
        }
    }

    pub fn update_profile_component(&mut self, component: Component) {
        self.profile.set_component(component);
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

    pub fn contains_fertilizer(&self, fertilizer_id: String) -> bool {
        let fertilizers_ids: Vec<String> = self
            .fertilizers
            .iter()
            .map(|fertilizer| fertilizer.id())
            .collect();

        fertilizers_ids.contains(&fertilizer_id)
    }

    pub fn build(&self) -> Solution {
        if self.fertilizers.len() > 0 {
            if let Ok(mut solution) =
                Calculation::new(self.profile.clone(), self.fertilizers.clone())
                    .unwrap()
                    .solve(1)
            {
                solution.set_id(self.id.clone());

                solution.set_name(self.name.clone());

                return solution;
            } else {
                let mut solution = Solution::empty(self.fertilizers.clone());

                solution.set_id(self.id.clone());

                solution.set_name(self.name.clone());

                solution
            }
        } else {
            let mut solution = Solution::empty(self.fertilizers.clone());

            solution.set_id(self.id.clone());

            solution.set_name(self.name.clone());

            solution
        }
    }
}
