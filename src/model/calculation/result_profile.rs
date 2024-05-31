use crate::model::calculation::FertilizerWeight;
use crate::model::chemistry::{NitrogenForm, NutrientAmount};
use crate::model::fertilizers::Fertilizer;
use std::ops::Index;

#[derive(Clone, Debug)]
pub struct ResultProfile {
    pub fertilizers_weights: Vec<FertilizerWeight>,
    nutrients: [NutrientAmount; 12],
    nitrogen_forms: [NitrogenForm; 2],
}

impl ResultProfile {
    pub fn new() -> Self {
        Self {
            nutrients: [
                NutrientAmount::Nitrogen(0.0),
                NutrientAmount::Phosphorus(0.0),
                NutrientAmount::Potassium(0.0),
                NutrientAmount::Calcium(0.0),
                NutrientAmount::Magnesium(0.0),
                NutrientAmount::Sulfur(0.0),
                NutrientAmount::Iron(0.0),
                NutrientAmount::Zinc(0.0),
                NutrientAmount::Manganese(0.0),
                NutrientAmount::Boron(0.0),
                NutrientAmount::Copper(0.0),
                NutrientAmount::Molybdenum(0.0),
            ],
            nitrogen_forms: [NitrogenForm::Nitrate(0.0), NitrogenForm::Ammonium(0.0)],
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
        fertilizer_weight.nutrients().iter().for_each(|nutrient| {
            self.nutrients[nutrient.index()] =
                self.nutrients[nutrient.index()].add(nutrient.value());
        });

        fertilizer_weight
            .nitrogen_forms()
            .iter()
            .for_each(|nitrogen_form| {
                self.nitrogen_forms[nitrogen_form.index()] =
                    self.nitrogen_forms[nitrogen_form.index()].add(nitrogen_form.value());
            });

        self.fertilizers_weights.push(fertilizer_weight);
    }
}

impl Index<NutrientAmount> for ResultProfile {
    type Output = NutrientAmount;

    fn index(&self, nutrient_amount: NutrientAmount) -> &Self::Output {
        &self.nutrients[nutrient_amount.index()]
    }
}

impl Index<NitrogenForm> for ResultProfile {
    type Output = NitrogenForm;

    fn index(&self, nitrogen_form: NitrogenForm) -> &Self::Output {
        &self.nitrogen_forms[nitrogen_form.index()]
    }
}
