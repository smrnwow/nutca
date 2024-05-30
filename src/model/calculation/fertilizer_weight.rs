use crate::model::fertilizers::{Fertilizer, NutrientContent};

#[derive(Debug, Clone)]
pub struct FertilizerWeight {
    pub fertilizer: Fertilizer,
    pub weight: f64,
}

impl FertilizerWeight {
    pub fn new(fertilizer: Fertilizer, weight: f64) -> Self {
        Self { fertilizer, weight }
    }

    pub fn nutrients(&self) -> Vec<NutrientContent> {
        self.fertilizer
            .nutrient_contents()
            .nutrients()
            .iter()
            .map(|nutrient| match nutrient {
                NutrientContent::Nitrogen(value) => NutrientContent::Nitrogen(value * self.weight),
                NutrientContent::NitrogenNitrate(value) => {
                    NutrientContent::NitrogenNitrate(value * self.weight)
                }
                NutrientContent::NitrogenAmmonium(value) => {
                    NutrientContent::NitrogenAmmonium(value * self.weight)
                }
                NutrientContent::Phosphor(value) => NutrientContent::Phosphor(value * self.weight),
                NutrientContent::Potassium(value) => {
                    NutrientContent::Potassium(value * self.weight)
                }
                NutrientContent::Calcium(value) => NutrientContent::Calcium(value * self.weight),
                NutrientContent::Magnesium(value) => {
                    NutrientContent::Magnesium(value * self.weight)
                }
                NutrientContent::Sulfur(value) => NutrientContent::Sulfur(value * self.weight),
                NutrientContent::Iron(value) => NutrientContent::Iron(value * self.weight),
                NutrientContent::Manganese(value) => {
                    NutrientContent::Manganese(value * self.weight)
                }
                NutrientContent::Copper(value) => NutrientContent::Copper(value * self.weight),
                NutrientContent::Zinc(value) => NutrientContent::Zinc(value * self.weight),
                NutrientContent::Boron(value) => NutrientContent::Boron(value * self.weight),
                NutrientContent::Molybdenum(value) => {
                    NutrientContent::Molybdenum(value * self.weight)
                }
            })
            .collect()
    }
}
