use super::{NutrientComposition, Solver};
use crate::model::chemistry::{NutrientAmount, Volume};
use crate::model::fertilizers::FertilizerAmount;
use crate::model::profiles::Profile;
use crate::model::solutions::Conductivity;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Solution {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) composition: NutrientComposition,
    pub(super) fertilizers: HashMap<String, FertilizerAmount>,
    pub(super) volume: Volume,
}

impl Solution {
    pub fn new(
        id: String,
        name: String,
        composition: NutrientComposition,
        mut fertilizers: HashMap<String, FertilizerAmount>,
        volume: Volume,
    ) -> Self {
        fertilizers.values_mut().for_each(|fertilizer_amount| {
            fertilizer_amount.volume(volume);
        });

        Self {
            id,
            name,
            composition,
            fertilizers,
            volume,
        }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn composition(&self) -> &NutrientComposition {
        &self.composition
    }

    pub fn fertilizers(&self) -> &HashMap<String, FertilizerAmount> {
        &self.fertilizers
    }

    pub fn fertilizer(&self, fertilizer_id: &String) -> Option<&FertilizerAmount> {
        self.fertilizers.get(fertilizer_id)
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

    pub fn change_nutrition_program(&mut self, nutrition_program: Option<Profile>) {
        self.composition.with_nutrition_program(nutrition_program);

        self.calculate_fertilizers_weights();
    }

    pub fn update_nutrient_requirement(&mut self, nutrient_requirement: NutrientAmount) {
        self.composition
            .with_nutrient_requirement(nutrient_requirement);

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
        Conductivity::new(self.composition.nutrients()).conductivity()
    }

    pub fn is_empty(&self) -> bool {
        self.fertilizers.is_empty()
    }

    fn calculate_fertilizers_weights(&mut self) {
        let calculation_results = Solver::new(
            self.composition.nutrition_program(),
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
        self.composition
            .with_fertilizers_amounts(self.fertilizers.values().collect())
    }
}

impl Default for Solution {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            composition: NutrientComposition::default(),
            fertilizers: HashMap::new(),
            volume: Volume::default(),
        }
    }
}

impl From<NutrientComposition> for Solution {
    fn from(composition: NutrientComposition) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            composition,
            fertilizers: HashMap::new(),
            volume: Volume::default(),
        }
    }
}
