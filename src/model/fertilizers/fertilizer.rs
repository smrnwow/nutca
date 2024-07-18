use crate::model::chemistry::{Nutrient, NutrientAmount, Nutrients};
use crate::model::fertilizers::labels::Label;
use crate::model::fertilizers::{Source, SourceType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Fertilizer {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) vendor: String,
    pub(super) source: Source,
    pub(super) liquid: bool,
    pub(super) nutrients: Nutrients,
}

impl Fertilizer {
    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn vendor(&self) -> String {
        self.vendor.clone()
    }

    pub fn source(&self) -> Source {
        self.source.clone()
    }

    pub fn source_type(&self) -> SourceType {
        match self.source {
            Source::Label(_) => SourceType::Label,
            Source::Formula(_) => SourceType::Formula,
        }
    }

    pub fn liquid(&self) -> bool {
        self.liquid
    }

    pub fn nutrients(&self) -> Nutrients {
        self.nutrients
    }

    pub fn nutrient_amount(&self, nutrient: Nutrient) -> NutrientAmount {
        self.nutrients[nutrient]
    }
}

impl Default for Fertilizer {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            vendor: String::new(),
            source: Source::Label(Label::default()),
            liquid: false,
            nutrients: Nutrients::new(),
        }
    }
}
