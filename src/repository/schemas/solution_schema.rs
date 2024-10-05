use super::ProfileRequirementSchema;
use crate::model::chemistry::Volume;
use crate::model::solutions::{Solution, SolutionSummary};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SolutionSchema {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) profile_requirement: ProfileRequirementSchema,
    pub(crate) fertilizers: HashMap<String, f64>,
    pub(crate) volume: Volume,
}

impl From<Solution> for SolutionSchema {
    fn from(solution: Solution) -> Self {
        Self {
            id: solution.id().clone(),
            name: solution.name().clone(),
            profile_requirement: ProfileRequirementSchema::from(
                solution.profile_requirement().clone(),
            ),
            fertilizers: solution.fertilizers().values().fold(
                HashMap::new(),
                |mut fertilizers, fertilizer_amount| {
                    fertilizers.insert(
                        fertilizer_amount.fertilizer().id(),
                        fertilizer_amount.amount_base(),
                    );

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
            profile_requirement: ProfileRequirementSchema::default(),
            fertilizers: HashMap::new(),
            volume: Volume::default(),
        }
    }
}
