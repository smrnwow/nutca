use super::Component;
use crate::model::chemistry::{NitrogenForm, NutrientAmount};
use serde::{Deserialize, Serialize};
use std::ops::Index;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Profile {
    id: String,
    name: String,
    nutrients: [NutrientAmount; 12],
    nitrogen_forms: [NitrogenForm; 2],
}

impl Profile {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
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

    pub fn from(name: &str, components: Vec<Component>) -> Self {
        let mut profile = Self::new();

        profile.set_name(name.to_string());

        for component in components {
            profile.set_component(component);
        }

        profile.create_id();

        profile
    }

    pub fn nutrients(&self) -> Vec<NutrientAmount> {
        Vec::from(self.nutrients)
    }

    pub fn nitrogen_forms(&self) -> Vec<NitrogenForm> {
        Vec::from(self.nitrogen_forms)
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn create_id(&mut self) {
        self.id = Uuid::new_v4().to_string();
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_component(&mut self, component: Component) {
        match component {
            Component::Nutrient(nutrient_amount) => {
                self.nutrients[nutrient_amount.index()] = nutrient_amount;

                if let NutrientAmount::Nitrogen(value) = nutrient_amount {
                    let nitrate_value =
                        value - self.nitrogen_forms[NitrogenForm::Ammonium(0.0).index()].value();

                    self.nitrogen_forms[NitrogenForm::Nitrate(0.0).index()] =
                        NitrogenForm::Nitrate(nitrate_value);
                }
            }

            Component::NitrogenForm(nitrogen_form) => {
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
        }
    }
}

impl Index<NutrientAmount> for Profile {
    type Output = NutrientAmount;

    fn index(&self, nutrient_amount: NutrientAmount) -> &Self::Output {
        &self.nutrients[nutrient_amount.index()]
    }
}

impl Index<NitrogenForm> for Profile {
    type Output = NitrogenForm;

    fn index(&self, nitrogen_form: NitrogenForm) -> &Self::Output {
        &self.nitrogen_forms[nitrogen_form.index()]
    }
}
