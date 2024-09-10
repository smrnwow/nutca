use super::FertilizerWeight;
use crate::model::chemistry::Nutrients;

pub struct NutrientComposition {
    nutrients: Nutrients,
}

impl NutrientComposition {
    pub fn new() -> Self {
        Self {
            nutrients: Nutrients::new(),
        }
    }

    pub fn nutrients(&self) -> &Nutrients {
        &self.nutrients
    }
}

impl From<Nutrients> for NutrientComposition {
    fn from(nutrients: Nutrients) -> Self {
        Self { nutrients }
    }
}

impl From<Vec<&FertilizerWeight>> for NutrientComposition {
    fn from(fertilizers_amounts: Vec<&FertilizerWeight>) -> Self {
        let mut nutrients = Nutrients::new();

        fertilizers_amounts.iter().for_each(|fertilizer_amount| {
            fertilizer_amount
                .nutrients()
                .list()
                .iter()
                .for_each(|nutrient_amount| {
                    nutrients.add(*nutrient_amount);
                });
        });

        Self { nutrients }
    }
}
