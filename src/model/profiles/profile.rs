use crate::model::chemistry::Nutrients;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Profile {
    pub nutrients: Nutrients,
    id: String,
    name: String,
}

impl Profile {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            nutrients: Nutrients::new(),
        }
    }

    pub fn from_another(another_profile: Profile) -> Self {
        let mut profile = Self::new();

        profile.set_name(another_profile.name());

        profile.set_nutrients(another_profile.nutrients());

        profile
    }

    pub fn from(name: &str, nutrients: Nutrients) -> Self {
        let mut profile = Self::new();

        profile.create_id();

        profile.set_name(name.to_string());

        profile.set_nutrients(nutrients);

        profile
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn nutrients(&self) -> Nutrients {
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

    pub fn set_nutrients(&mut self, nutrients: Nutrients) {
        self.nutrients = nutrients;
    }
}
