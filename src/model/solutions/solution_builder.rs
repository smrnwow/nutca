use crate::model::chemistry::{NutrientAmount, Volume};
use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::{Profile, ProfileBuilder};
use crate::model::solutions::{Solution, Solver};
use crate::model::Error;
use uuid::Uuid;

/// A helper struct that enables to incremental building a solution
pub struct SolutionBuilder {
    id: String,
    name: String,
    profile_builder: ProfileBuilder,
    fertilizers: Vec<Fertilizer>,
    volume: Volume,
}

impl SolutionBuilder {
    /// Creates a new instance of the builder with default values
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            profile_builder: ProfileBuilder::new(),
            fertilizers: Vec::new(),
            volume: Volume::default(),
        }
    }

    /// Sets the solution's name
    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = name;

        self
    }

    /// Sets volume of the solution
    pub fn volume(&mut self, volume: Volume) -> &mut Self {
        self.volume = volume;

        self
    }

    /// Sets the nutrient profile.
    /// If `profile` is `Some`, uses it; otherwise, sets a new empty one.
    pub fn profile(&mut self, profile: Option<Profile>) -> &mut Self {
        match profile {
            Some(profile) => self.profile_builder = ProfileBuilder::from(profile),
            None => self.profile_builder = ProfileBuilder::new(),
        };

        self
    }

    /// Updates the nutrient requirement for the solution.
    /// If the current nutrient profile is saved, it extends the new profile from the existing one
    /// and applies the updated requirements to the new profile.
    pub fn nutrient_requirement(&mut self, nutrient_amount: NutrientAmount) -> &mut Self {
        if self.profile_builder.is_saved() {
            let profile = self.profile_builder.build();

            self.profile_builder = ProfileBuilder::new();

            self.profile_builder
                .name(profile.name())
                .nutrients(profile.nutrients());
        }

        self.profile_builder.nutrient_requirement(nutrient_amount);

        self
    }

    /// Adds fertilizer to solution
    pub fn add_fertilizer(&mut self, fertilizer: Fertilizer) -> &mut Self {
        self.fertilizers.push(fertilizer);

        self
    }

    /// Removes fertilizer from solution
    /// `fertilizer_id`: The unique identifier for the fertilizer to be removed
    pub fn remove_fertilizer(&mut self, fertilizer_id: String) -> &mut Self {
        let position = self
            .fertilizers
            .iter()
            .position(|fertilizer| fertilizer.id() == fertilizer_id);

        if let Some(index) = position {
            self.fertilizers.remove(index);
        }

        self
    }

    /// Validates the solution
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

    /// Builds the solution
    pub fn build(&self) -> Solution {
        let profile = self.profile_builder.build();

        println!("self.fertilizers {:#?}", self.fertilizers);

        let fertilizers_set = Solver::new(&profile, self.fertilizers()).solve();

        let nutrients = fertilizers_set.nutrients();

        Solution {
            id: self.id.clone(),
            name: self.name.clone(),
            profile,
            volume: self.volume,
            fertilizers_set,
            nutrients,
        }
    }

    fn fertilizers(&self) -> Vec<&Fertilizer> {
        self.fertilizers.iter().collect()
    }
}

impl From<Solution> for SolutionBuilder {
    fn from(solution: Solution) -> Self {
        Self {
            id: solution.id(),
            name: solution.name(),
            profile_builder: ProfileBuilder::from(solution.profile()),
            fertilizers: solution.fertilizers_set.into(),
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

#[cfg(test)]
mod tests {
    use super::SolutionBuilder;
    use crate::model::fertilizers::{FertilizerBuilder, LabelComponent};

    #[test]
    fn fertilizers_management() {
        let fertilizer_1 = FertilizerBuilder::new()
            .label_component(LabelComponent::Calcium(10.0))
            .build();

        let fertilizer_2 = FertilizerBuilder::new()
            .label_component(LabelComponent::Copper(0.01))
            .build();

        let mut solution_builder = SolutionBuilder::new();

        solution_builder
            .add_fertilizer(fertilizer_1.clone())
            .add_fertilizer(fertilizer_2.clone());

        assert!(solution_builder
            .fertilizers
            .iter()
            .find(|fertilizer| fertilizer.id() == fertilizer_1.id())
            .is_some());

        solution_builder.remove_fertilizer(fertilizer_1.id());

        assert!(solution_builder
            .fertilizers
            .iter()
            .find(|fertilizer| fertilizer.id() == fertilizer_1.id())
            .is_none());

        assert!(solution_builder
            .fertilizers
            .iter()
            .find(|fertilizer| fertilizer.id() == fertilizer_2.id())
            .is_some());
    }
}
