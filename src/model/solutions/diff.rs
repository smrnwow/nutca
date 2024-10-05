use super::NutrientResult;
use crate::model::chemistry::{Nutrient, Nutrients};

#[derive(Clone, Debug, PartialEq)]
pub struct Diff {
    target: Nutrients,
    calculated: Nutrients,
}

impl Diff {
    pub fn new(target: Nutrients, calculated: Nutrients) -> Self {
        Self { target, calculated }
    }

    pub fn nutrient_diff(&self, nutrient: Nutrient) -> NutrientResult {
        NutrientResult::new(
            self.target.value_of(nutrient).value(),
            format!("{:.3}", self.calculated.value_of(nutrient).value())
                .parse()
                .unwrap(),
        )
    }
}
