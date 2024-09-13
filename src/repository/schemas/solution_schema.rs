use super::NutritionProgramSchema;
use crate::model::chemistry::Volume;
use crate::model::solutions::{Solution, SolutionSummary};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SolutionSchema {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) nutrition_program: NutritionProgramSchema,
    pub(crate) fertilizers: HashMap<String, f64>,
    pub(crate) volume: Volume,
}

impl From<Solution> for SolutionSchema {
    fn from(solution: Solution) -> Self {
        Self {
            id: solution.id().clone(),
            name: solution.name().clone(),
            nutrition_program: NutritionProgramSchema::from(solution.composition().clone()),
            fertilizers: solution.fertilizers().values().fold(
                HashMap::new(),
                |mut fertilizers, fertilizer_amount| {
                    fertilizers.insert(fertilizer_amount.id(), fertilizer_amount.weight());

                    fertilizers
                },
            ),
            volume: solution.volume().clone(),
        }
    }
}

impl Into<SolutionSummary> for SolutionSchema {
    fn into(self) -> SolutionSummary {
        SolutionSummary::new(self.id, self.name)
    }
}

impl Default for SolutionSchema {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            nutrition_program: NutritionProgramSchema::default(),
            fertilizers: HashMap::new(),
            volume: Volume::default(),
        }
    }
}
