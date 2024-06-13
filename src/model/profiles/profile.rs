use crate::model::chemistry::Nutrient;
use serde::{Deserialize, Serialize};
use std::ops::Index;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Profile {
    id: String,
    name: String,
    nutrients: [Nutrient; 14],
}

impl Profile {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            nutrients: [
                Nutrient::Nitrogen(0.0),
                Nutrient::NitrogenNitrate(0.0),
                Nutrient::NitrogenAmmonium(0.0),
                Nutrient::Phosphorus(0.0),
                Nutrient::Potassium(0.0),
                Nutrient::Calcium(0.0),
                Nutrient::Magnesium(0.0),
                Nutrient::Sulfur(0.0),
                Nutrient::Iron(0.0),
                Nutrient::Manganese(0.0),
                Nutrient::Copper(0.0),
                Nutrient::Zinc(0.0),
                Nutrient::Boron(0.0),
                Nutrient::Molybdenum(0.0),
            ],
        }
    }

    pub fn from_another(another_profile: Profile) -> Self {
        let mut profile = Self::new();

        profile.set_name(another_profile.name());

        for nutrient_amount in another_profile.nutrients() {
            profile.add_nutrient(nutrient_amount);
        }

        profile
    }

    pub fn from(name: &str, nutrients: Vec<Nutrient>) -> Self {
        let mut profile = Self::new();

        profile.set_name(name.to_string());

        for nutrient in nutrients {
            profile.set_nutrient(nutrient);
        }

        profile.create_id();

        profile
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn nutrients(&self) -> Vec<Nutrient> {
        Vec::from(self.nutrients)
    }

    pub fn nutrients_array(&self) -> [Nutrient; 14] {
        self.nutrients
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

    pub fn set_nutrient(&mut self, nutrient: Nutrient) {
        self.nutrients[nutrient.index()] = nutrient;

        match nutrient {
            Nutrient::Nitrogen(value) => {
                let nitrate_value =
                    value - self.nutrients[Nutrient::NitrogenAmmonium(0.0).index()].value();

                self.nutrients[Nutrient::NitrogenNitrate(0.0).index()] =
                    Nutrient::NitrogenNitrate(nitrate_value);
            }

            Nutrient::NitrogenNitrate(value) => {
                let ammonium_value =
                    self.nutrients[Nutrient::Nitrogen(0.0).index()].value() - value;

                self.nutrients[Nutrient::NitrogenAmmonium(0.0).index()] =
                    Nutrient::NitrogenAmmonium(ammonium_value);
            }

            Nutrient::NitrogenAmmonium(value) => {
                let nitrate_value = self.nutrients[Nutrient::Nitrogen(0.0).index()].value() - value;

                self.nutrients[Nutrient::NitrogenNitrate(0.0).index()] =
                    Nutrient::NitrogenNitrate(nitrate_value);
            }

            _ => {}
        }
    }

    pub fn add_nutrient(&mut self, nutrient: Nutrient) {
        self.nutrients[nutrient.index()] = self.nutrients[nutrient.index()].add(nutrient.value());
    }
}

impl Index<Nutrient> for Profile {
    type Output = Nutrient;

    fn index(&self, nutrient: Nutrient) -> &Self::Output {
        &self.nutrients[nutrient.index()]
    }
}
