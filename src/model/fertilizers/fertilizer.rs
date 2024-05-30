use super::{Composition, Contents, Label, Units};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Fertilizer {
    id: String,
    name: String,
    vendor: String,
    composition: Composition,
    nutrient_contents: Contents,
}

impl Fertilizer {
    pub fn build() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            vendor: String::new(),
            composition: Composition::Label(Label::new(Units::Percent)),
            nutrient_contents: Contents::new(),
        }
    }

    pub fn set_name(self, name: String) -> Self {
        Self { name, ..self }
    }

    pub fn set_vendor(self, vendor: String) -> Self {
        Self { vendor, ..self }
    }

    pub fn set_composition(self, composition: Composition) -> Self {
        Self {
            composition: composition.clone(),
            nutrient_contents: composition.into(),
            ..self
        }
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn vendor(&self) -> String {
        self.vendor.clone()
    }

    pub fn nutrient_contents(&self) -> Contents {
        self.nutrient_contents
    }
}
