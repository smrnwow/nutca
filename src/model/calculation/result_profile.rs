use crate::model::calculation::{FertilizerWeight, NutrientRequirement, Profile};
use crate::model::fertilizers::{Fertilizer, NutrientContent};

#[derive(Clone, Debug)]
pub struct ResultProfile {
    pub profile: Profile,
    pub fertilizers_weights: Vec<FertilizerWeight>,
}

impl ResultProfile {
    pub fn new() -> Self {
        Self {
            profile: Profile::new(),
            fertilizers_weights: Vec::new(),
        }
    }

    pub fn empty(fertilizers: Vec<Fertilizer>) -> Self {
        let mut result_profile = Self::new();

        for fertilizer in fertilizers {
            result_profile.add_fertilizer_weight(FertilizerWeight::new(fertilizer, 0.0));
        }

        result_profile
    }

    pub fn add_fertilizer_weight(&mut self, fertilizer_weight: FertilizerWeight) {
        fertilizer_weight
            .nutrients()
            .iter()
            .for_each(|nutrient| match nutrient {
                NutrientContent::Nitrogen(value) => self
                    .profile
                    .update_nutrient(NutrientRequirement::Nitrogen(*value)),
                NutrientContent::NitrogenNitrate(value) => self
                    .profile
                    .update_nutrient(NutrientRequirement::NitrogenNitrate(*value)),
                NutrientContent::NitrogenAmmonium(value) => self
                    .profile
                    .update_nutrient(NutrientRequirement::NitrogenAmmonium(*value)),
                NutrientContent::Phosphor(value) => self
                    .profile
                    .update_nutrient(NutrientRequirement::Phosphor(*value)),
                NutrientContent::Potassium(value) => self
                    .profile
                    .update_nutrient(NutrientRequirement::Potassium(*value)),
                NutrientContent::Calcium(value) => self
                    .profile
                    .update_nutrient(NutrientRequirement::Calcium(*value)),
                NutrientContent::Magnesium(value) => self
                    .profile
                    .update_nutrient(NutrientRequirement::Magnesium(*value)),
                NutrientContent::Sulfur(value) => self
                    .profile
                    .update_nutrient(NutrientRequirement::Sulfur(*value)),
                NutrientContent::Iron(value) => self
                    .profile
                    .update_nutrient(NutrientRequirement::Iron(*value)),
                NutrientContent::Manganese(value) => self
                    .profile
                    .update_nutrient(NutrientRequirement::Manganese(*value)),
                NutrientContent::Copper(value) => self
                    .profile
                    .update_nutrient(NutrientRequirement::Copper(*value)),
                NutrientContent::Zinc(value) => self
                    .profile
                    .update_nutrient(NutrientRequirement::Zinc(*value)),
                NutrientContent::Boron(value) => self
                    .profile
                    .update_nutrient(NutrientRequirement::Boron(*value)),
                NutrientContent::Molybdenum(value) => self
                    .profile
                    .update_nutrient(NutrientRequirement::Molybdenum(*value)),
            });

        self.fertilizers_weights.push(fertilizer_weight);
    }
}
