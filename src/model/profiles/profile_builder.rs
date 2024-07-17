use crate::model::chemistry::{NutrientAmount, Nutrients};
use crate::model::profiles::Profile;
use crate::model::Error;
use uuid::Uuid;

pub struct ProfileBuilder {
    pub nutrients: Nutrients,
    id: String,
    name: String,
}

impl ProfileBuilder {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            nutrients: Nutrients::new(),
        }
    }

    pub fn from(profile: Profile) -> Self {
        Self {
            id: profile.id(),
            name: profile.name(),
            nutrients: profile.nutrients(),
        }
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn update_nutrient(&mut self, nutrient_amount: NutrientAmount) {
        self.nutrients.set(nutrient_amount);
    }

    pub fn validate(&self) -> Vec<Error> {
        let mut errors = Vec::new();

        if self.name.len() == 0 {
            errors.push(Error::ProfileNameEmpty);
        }

        if self.name.len() > 100 {
            errors.push(Error::ProfileNameTooLong);
        }

        errors
    }

    pub fn build(&self) -> Profile {
        let mut profile = Profile::from(self.name.as_str(), self.nutrients);

        profile.set_id(self.id.clone());

        profile
    }
}
