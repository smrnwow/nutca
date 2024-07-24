use crate::model::chemistry::{Nutrient, NutrientAmount};
use crate::model::solutions::FertilizersSet;
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Nutrients {
    nutrients: [NutrientAmount; 14],
}

impl Nutrients {
    pub fn new() -> Self {
        Self {
            nutrients: [
                NutrientAmount::Nitrogen(0.0),
                NutrientAmount::NitrogenNitrate(0.0),
                NutrientAmount::NitrogenAmmonium(0.0),
                NutrientAmount::Phosphorus(0.0),
                NutrientAmount::Potassium(0.0),
                NutrientAmount::Calcium(0.0),
                NutrientAmount::Magnesium(0.0),
                NutrientAmount::Sulfur(0.0),
                NutrientAmount::Iron(0.0),
                NutrientAmount::Manganese(0.0),
                NutrientAmount::Copper(0.0),
                NutrientAmount::Zinc(0.0),
                NutrientAmount::Boron(0.0),
                NutrientAmount::Molybdenum(0.0),
            ],
        }
    }

    pub fn multiply(&self, factor: f64) -> Self {
        let mut nutrients = Self::new();

        for nutrient_amount in self.nutrients {
            nutrients.set(nutrient_amount.new(nutrient_amount.value() * factor));
        }

        nutrients
    }

    pub fn list(&self) -> Vec<NutrientAmount> {
        self.nutrients
            .iter()
            .filter(|nutrient| nutrient.value() > 0.)
            .map(|nutrient| *nutrient)
            .collect()
    }

    pub fn total_nutrients(&self) -> f64 {
        self.list()
            .iter()
            .map(|nutrient_amount| nutrient_amount.value())
            .sum()
    }

    pub fn stringify(&self) -> String {
        let mut string = String::new();

        self.list()
            .iter()
            .for_each(|nutrient_amount| string.push_str(nutrient_amount.nutrient().symbol()));

        string
    }

    pub fn value_of(&self, nutrient: Nutrient) -> NutrientAmount {
        self.nutrients[nutrient.index()]
    }

    pub fn macros(&self) -> Vec<NutrientAmount> {
        self.nutrients
            .iter()
            .filter(|nutrient_amount| match nutrient_amount.nutrient() {
                Nutrient::Nitrogen
                | Nutrient::Phosphorus
                | Nutrient::Potassium
                | Nutrient::Calcium
                | Nutrient::Magnesium
                | Nutrient::Sulfur => nutrient_amount.value() > 0.,
                _ => false,
            })
            .map(|nutrient| *nutrient)
            .collect()
    }

    pub fn nitrogen_forms(&self) -> Vec<NutrientAmount> {
        self.nutrients
            .iter()
            .filter(|nutrient_amount| match nutrient_amount.nutrient() {
                Nutrient::NitrogenNitrate | Nutrient::NitrogenAmmonium => {
                    nutrient_amount.value() > 0.
                }
                _ => false,
            })
            .map(|nutrient| *nutrient)
            .collect()
    }

    pub fn micros(&self) -> Vec<NutrientAmount> {
        self.nutrients
            .iter()
            .filter(|nutrient_amount| match nutrient_amount.nutrient() {
                Nutrient::Iron
                | Nutrient::Manganese
                | Nutrient::Copper
                | Nutrient::Zinc
                | Nutrient::Boron
                | Nutrient::Molybdenum => nutrient_amount.value() > 0.,
                _ => false,
            })
            .map(|nutrient| *nutrient)
            .collect()
    }

    pub fn add(&mut self, nutrient_amount: NutrientAmount) {
        let nutrient = nutrient_amount.nutrient();
        self[nutrient] = self[nutrient].add(nutrient_amount.value());
    }

    pub fn set(&mut self, nutrient_amount: NutrientAmount) {
        self[nutrient_amount.nutrient()] = nutrient_amount;

        match nutrient_amount {
            NutrientAmount::Nitrogen(value) => {
                let nitrate_value = value - self[Nutrient::NitrogenAmmonium].value();
                self[Nutrient::NitrogenNitrate] = NutrientAmount::NitrogenNitrate(nitrate_value);
            }
            NutrientAmount::NitrogenNitrate(value) => {
                let ammonium_value = self[Nutrient::Nitrogen].value() - value;
                self[Nutrient::NitrogenAmmonium] = NutrientAmount::NitrogenAmmonium(ammonium_value);
            }
            NutrientAmount::NitrogenAmmonium(value) => {
                let nitrate_value = self[Nutrient::Nitrogen].value() - value;
                self[Nutrient::NitrogenNitrate] = NutrientAmount::NitrogenNitrate(nitrate_value);
            }
            _ => {}
        }
    }
}

impl Index<Nutrient> for Nutrients {
    type Output = NutrientAmount;

    fn index(&self, nutrient: Nutrient) -> &Self::Output {
        &self.nutrients[nutrient.index()]
    }
}

impl IndexMut<Nutrient> for Nutrients {
    fn index_mut(&mut self, nutrient: Nutrient) -> &mut Self::Output {
        &mut self.nutrients[nutrient.index()]
    }
}

impl From<Vec<NutrientAmount>> for Nutrients {
    fn from(nutrients_amounts: Vec<NutrientAmount>) -> Self {
        let mut nutrients = Self::new();

        for nutrient_amount in nutrients_amounts {
            nutrients.set(nutrient_amount);
        }

        nutrients
    }
}

impl From<&FertilizersSet> for Nutrients {
    fn from(fertilizers_set: &FertilizersSet) -> Nutrients {
        let mut nutrients = Nutrients::new();

        fertilizers_set.list().iter().for_each(|fertilizer_weight| {
            fertilizer_weight
                .nutrients
                .list()
                .iter()
                .for_each(|nutrient_amount| {
                    nutrients.add(*nutrient_amount);
                });
        });

        nutrients
    }
}
