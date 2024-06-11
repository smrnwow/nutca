use crate::model::chemistry::{NitrogenForm, NutrientAmount};
use crate::model::profiles::{Component, Profile};
use uuid::Uuid;

pub struct ProfileBuilder {
    id: String,
    name: String,
    nutrients: [NutrientAmount; 12],
    nitrogen_forms: [NitrogenForm; 2],
}

impl ProfileBuilder {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
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

    pub fn from(profile: Profile) -> Self {
        Self {
            id: profile.id(),
            name: profile.name(),
            nutrients: profile.nutrients_array(),
            nitrogen_forms: profile.nitrogen_forms_array(),
        }
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn update_component(&mut self, component: Component) {
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

    pub fn build(&self) -> Profile {
        let mut components: Vec<Component> = vec![];

        self.nutrients
            .iter()
            .for_each(|nutrient_amount| components.push(Component::Nutrient(*nutrient_amount)));

        self.nitrogen_forms
            .iter()
            .for_each(|nitrogen_form| components.push(Component::NitrogenForm(*nitrogen_form)));

        let mut profile = Profile::from(self.name.as_str(), components);

        profile.set_id(self.id.clone());

        profile
    }
}
