use super::UsedFertilizer;
use crate::model::chemistry::{NutrientAmount, Nutrients};

pub struct UsedFertilizerBuilder {
    id: Option<isize>,
    nutrients: Nutrients,
}

impl UsedFertilizerBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            nutrients: Nutrients::new(),
        }
    }

    pub fn id(&mut self, id: isize) -> &mut Self {
        self.id = Some(id);

        self
    }

    pub fn nutrient(&mut self, nutrient_amount: NutrientAmount) -> &mut Self {
        self.nutrients.set(nutrient_amount);

        self
    }

    pub fn build(&self) -> UsedFertilizer {
        UsedFertilizer {
            id: self.id.unwrap_or(0),
            nutrients: self.nutrients,
        }
    }
}
