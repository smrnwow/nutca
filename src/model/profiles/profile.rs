use crate::model::chemistry::{Nutrient, NutrientAmount, Nutrients};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Profile {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) nutrients: Nutrients,
}

impl Profile {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn nutrients(&self) -> Nutrients {
        self.nutrients
    }

    pub fn nutrient_requirement(&self, nutrient: Nutrient) -> NutrientAmount {
        self.nutrients[nutrient]
    }

    pub fn is_empty(&self) -> bool {
        self.nutrients.total_amount() == 0.0
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            nutrients: Nutrients::new(),
        }
    }
}
