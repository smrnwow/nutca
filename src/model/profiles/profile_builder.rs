use crate::model::chemistry::{NutrientAmount, Nutrients};
use crate::model::profiles::Profile;
use crate::model::Error;
use uuid::Uuid;

pub struct ProfileBuilder {
    id: String,
    name: String,
    nutrients: Nutrients,
}

impl ProfileBuilder {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            nutrients: Nutrients::new(),
        }
    }

    pub fn new_id(&mut self) -> &mut Self {
        self.id = Uuid::new_v4().to_string();
        self
    }

    pub fn name(&mut self, name: impl ToString) -> &mut Self {
        self.name = name.to_string();
        self
    }

    pub fn nutrients(&mut self, nutrients: Nutrients) -> &mut Self {
        self.nutrients = nutrients;
        self
    }

    pub fn nutrient_requirement(&mut self, nutrient_amount: NutrientAmount) -> &mut Self {
        self.nutrients.set(nutrient_amount);
        self
    }

    pub fn is_saved(&self) -> bool {
        self.id.len() > 0
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
        Profile {
            id: self.id.clone(),
            name: self.name.clone(),
            nutrients: self.nutrients.clone(),
        }
    }
}

impl From<Profile> for ProfileBuilder {
    fn from(profile: Profile) -> Self {
        Self {
            id: profile.id(),
            name: profile.name(),
            nutrients: profile.nutrients(),
        }
    }
}
