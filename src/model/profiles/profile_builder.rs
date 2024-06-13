use crate::model::chemistry::Nutrient;
use crate::model::profiles::Profile;
use uuid::Uuid;

pub struct ProfileBuilder {
    id: String,
    name: String,
    nutrients: [Nutrient; 14],
}

impl ProfileBuilder {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
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

    pub fn from(profile: Profile) -> Self {
        Self {
            id: profile.id(),
            name: profile.name(),
            nutrients: profile.nutrients_array(),
        }
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn update_nutrient(&mut self, nutrient: Nutrient) {
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

    pub fn build(&self) -> Profile {
        let mut nutrients: Vec<Nutrient> = vec![];

        self.nutrients
            .iter()
            .for_each(|nutrient| nutrients.push(*nutrient));

        let mut profile = Profile::from(self.name.as_str(), nutrients);

        profile.set_id(self.id.clone());

        profile
    }
}
