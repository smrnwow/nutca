use crate::model::chemistry::Nutrients;
use crate::model::solutions::NutrientComposition;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum NutritionProgramSchema {
    Saved(String),
    ByValue(Nutrients),
}

impl From<NutrientComposition> for NutritionProgramSchema {
    fn from(nutrient_composition: NutrientComposition) -> Self {
        let nutrition_program = nutrient_composition.nutrition_program();

        match nutrition_program.is_saved() {
            true => NutritionProgramSchema::Saved(nutrition_program.id()),
            false => NutritionProgramSchema::ByValue(nutrition_program.nutrients()),
        }
    }
}

impl Default for NutritionProgramSchema {
    fn default() -> Self {
        Self::ByValue(Nutrients::new())
    }
}
