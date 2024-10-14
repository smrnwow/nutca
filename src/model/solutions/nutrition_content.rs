use crate::model::chemistry::{Nutrient, NutrientAmount, Nutrients};
use crate::model::fertilizers::FertilizerAmount;

#[derive(Clone, Debug, PartialEq)]
pub struct NutritionContent {
    nutrients: Nutrients,
}

impl NutritionContent {
    pub fn new() -> Self {
        Self {
            nutrients: Nutrients::new(),
        }
    }

    pub fn calculate(&mut self, fertilizers_amounts: Vec<&FertilizerAmount>) {
        let mut nutrients = Nutrients::new();

        fertilizers_amounts.iter().for_each(|fertilizer_amount| {
            fertilizer_amount
                .nutrients()
                .list()
                .iter()
                .for_each(|nutrient_amount| {
                    nutrients.add(nutrient_amount.new(nutrient_amount.factor()));
                });
        });

        self.nutrients = nutrients;
    }

    pub fn nutrients(&self) -> &Nutrients {
        &self.nutrients
    }

    pub fn value_of(&self, nutrient: Nutrient) -> NutrientAmount {
        self.nutrients.value_of(nutrient)
    }
}

impl From<Vec<&FertilizerAmount>> for NutritionContent {
    fn from(fertilizers_amounts: Vec<&FertilizerAmount>) -> Self {
        let mut nutrition_content = NutritionContent::new();

        nutrition_content.calculate(fertilizers_amounts);

        nutrition_content
    }
}
