use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum FertilizerPortion {
    Calculated(f64),
    DuplicatedNutrientSource,
    NullishNutrientRequirement,
    NegativeAmount,
    RedurantFertilizer,
}

impl FertilizerPortion {
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
