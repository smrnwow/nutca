use super::{Diff, NutritionContent, ProfileRequirement, Solver};
use crate::model::chemistry::{Nutrient, NutrientAmount, Volume};
use crate::model::fertilizers::FertilizerAmount;
use crate::model::profiles::Profile;
use crate::model::solutions::Conductivity;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Solution {
    id: String,
    name: String,
    profile_requirement: ProfileRequirement,
    fertilizers: HashMap<String, FertilizerAmount>,
    volume: Volume,
    nutrition_content: NutritionContent,
}

impl Solution {
    pub fn new(
        id: String,
        name: String,
        profile_requirement: ProfileRequirement,
        mut fertilizers: HashMap<String, FertilizerAmount>,
        volume: Volume,
    ) -> Self {
        fertilizers.values_mut().for_each(|fertilizer_amount| {
            fertilizer_amount.volume(volume);
        });

        let nutrition_content =
            NutritionContent::from(fertilizers.values().collect::<Vec<&FertilizerAmount>>());

        Self {
            id,
            name,
            profile_requirement,
            fertilizers,
            volume,
            nutrition_content,
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn profile_requirement(&self) -> &ProfileRequirement {
        &self.profile_requirement
    }

    pub fn fertilizers(&self) -> &HashMap<String, FertilizerAmount> {
        &self.fertilizers
    }

    pub fn fertilizer(&self, fertilizer_id: &String) -> Option<&FertilizerAmount> {
        self.fertilizers.get(fertilizer_id)
    }

    pub fn nutrition_content(&self) -> &NutritionContent {
        &self.nutrition_content
    }

    pub fn diff(&self) -> Diff {
        Diff::new(
            *self.profile_requirement.nutrients(),
            *self.nutrition_content.nutrients(),
        )
    }

    pub fn nutrient_value(&self, nutrient: Nutrient) -> NutrientAmount {
        self.nutrition_content.value_of(nutrient)
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn update_volume(&mut self, volume: Volume) {
        self.volume = volume;

        self.fertilizers.values_mut().for_each(|fertilizer_amount| {
            fertilizer_amount.volume(volume);
        });
    }

    pub fn change_profile(&mut self, profile: Option<Profile>) {
        match profile {
            Some(profile) => self.profile_requirement.set_profile(profile),
            None => self.profile_requirement.drop_profile(),
        };

        self.calculate_fertilizers_weights();
    }

    pub fn update_nutrient_requirement(&mut self, nutrient_amount: NutrientAmount) {
        self.profile_requirement.update_nutrient(nutrient_amount);

        self.calculate_fertilizers_weights();
    }

    pub fn add_fertilizer(&mut self, fertilizer: impl Into<FertilizerAmount>) {
        let fertilizer_weight = fertilizer.into();

        self.fertilizers
            .insert(fertilizer_weight.fertilizer().id(), fertilizer_weight);

        self.calculate_fertilizers_weights();
    }

    pub fn remove_fertilizer(&mut self, fertilizer_id: &String) -> Option<FertilizerAmount> {
        match self.fertilizers.remove(fertilizer_id) {
            Some(fertilizer_weight) => {
                self.calculate_fertilizers_weights();

                Some(fertilizer_weight)
            }

            None => None,
        }
    }

    pub fn update_fertilizer_amount(&mut self, fertilizer_id: &String, amount: f64) {
        if let Some(fertilizer_weight) = self.fertilizers.get_mut(fertilizer_id) {
            fertilizer_weight.update_amount(amount);

            self.calculate_nutrients();
        }
    }

    pub fn volume(&self) -> Volume {
        self.volume
    }

    pub fn ec(&self) -> f64 {
        Conductivity::new(self.nutrition_content.nutrients()).conductivity()
    }

    pub fn is_empty(&self) -> bool {
        self.fertilizers.is_empty()
    }

    fn calculate_fertilizers_weights(&mut self) {
        let calculation_results = Solver::new(
            self.profile_requirement.nutrients(),
            self.fertilizers
                .values()
                .map(|fertilizer_weight| fertilizer_weight.fertilizer())
                .collect(),
        )
        .solve();

        calculation_results
            .iter()
            .for_each(|(fertilizer_id, value)| {
                if let Some(fertilizer_weight) = self.fertilizers.get_mut(fertilizer_id) {
                    fertilizer_weight.update_amount(value.amount());
                }
            });

        self.calculate_nutrients()
    }

    fn calculate_nutrients(&mut self) {
        self.nutrition_content
            .calculate(self.fertilizers.values().collect())
    }
}

impl Default for Solution {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            profile_requirement: ProfileRequirement::new(),
            fertilizers: HashMap::new(),
            volume: Volume::default(),
            nutrition_content: NutritionContent::new(),
        }
    }
}

impl From<ProfileRequirement> for Solution {
    fn from(profile_requirement: ProfileRequirement) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            profile_requirement,
            fertilizers: HashMap::new(),
            volume: Volume::default(),
            nutrition_content: NutritionContent::new(),
        }
    }
}
