use crate::model::chemistry::{NitrogenForm, NutrientAmount};
use std::ops::Index;

#[derive(Clone, Copy, Debug)]
pub struct Profile {
    nutrients: [NutrientAmount; 12],
    nitrogen_forms: [NitrogenForm; 2],
}

impl Profile {
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
                NutrientAmount::Manganese(0.0),
                NutrientAmount::Copper(0.0),
                NutrientAmount::Zinc(0.0),
                NutrientAmount::Boron(0.0),
                NutrientAmount::Molybdenum(0.0),
            ],
            nitrogen_forms: [NitrogenForm::Nitrate(0.0), NitrogenForm::Ammonium(0.0)],
        }
    }

    pub fn nutrients(&self) -> Vec<NutrientAmount> {
        Vec::from(self.nutrients)
    }

    pub fn nitrogen_forms(&self) -> Vec<NitrogenForm> {
        Vec::from(self.nitrogen_forms)
    }

    pub fn set_nutrient(&mut self, nutrient_requirement: NutrientAmount) {
        self.nutrients[nutrient_requirement.index()] = nutrient_requirement;

        if let NutrientAmount::Nitrogen(value) = nutrient_requirement {
            let nitrate_value =
                value - self.nitrogen_forms[NitrogenForm::Ammonium(0.0).index()].value();

            self.nitrogen_forms[NitrogenForm::Nitrate(0.0).index()] =
                NitrogenForm::Nitrate(nitrate_value);
        }
    }

    pub fn set_nitrogen_form(&mut self, nitrogen_form: NitrogenForm) {
        self.nitrogen_forms[nitrogen_form.index()] = nitrogen_form;

        match nitrogen_form {
            NitrogenForm::Nitrate(value) => {
                let ammonium_value =
                    self.nutrients[NutrientAmount::Nitrogen(0.0).index()].value() - value;

                self.nitrogen_forms[NitrogenForm::Ammonium(0.0).index()] =
                    NitrogenForm::Ammonium(ammonium_value);
            }

            NitrogenForm::Ammonium(value) => {
                let nitrate_value =
                    self.nutrients[NutrientAmount::Nitrogen(0.0).index()].value() - value;

                self.nitrogen_forms[NitrogenForm::Nitrate(0.0).index()] =
                    NitrogenForm::Nitrate(nitrate_value);
            }
        }
    }

    pub fn update_nutrient(&mut self, nutrient_requirement: NutrientAmount) {
        let nutrient = self.nutrients[nutrient_requirement.index()];

        self.nutrients[nutrient_requirement.index()] = nutrient.add(nutrient_requirement.value());
    }
}

impl Index<NutrientAmount> for Profile {
    type Output = NutrientAmount;

    fn index(&self, nutrient_requirement: NutrientAmount) -> &Self::Output {
        &self.nutrients[nutrient_requirement.index()]
    }
}

impl Index<NitrogenForm> for Profile {
    type Output = NitrogenForm;

    fn index(&self, nitrogen_form: NitrogenForm) -> &Self::Output {
        &self.nitrogen_forms[nitrogen_form.index()]
    }
}
