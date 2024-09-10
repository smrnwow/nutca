use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum CalculationResult {
    Calculated(f64),
    DuplicatedNutrientSource,
    NullishNutrientRequirement,
    NegativeAmount,
    RedurantFertilizer,
}

impl CalculationResult {
    pub fn amount(&self) -> f64 {
        match self {
            Self::Calculated(amount) => *amount,
            Self::DuplicatedNutrientSource => 0.0,
            Self::NullishNutrientRequirement => 0.0,
            Self::NegativeAmount => 0.0,
            Self::RedurantFertilizer => 0.0,
        }
    }
}
