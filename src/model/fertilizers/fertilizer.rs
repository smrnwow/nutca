use crate::model::chemistry::Nutrients;
use crate::model::fertilizers::SourceComposition;
use serde::{Deserialize, Serialize};

/// A plant supplement containing essential nutrients.
/// Added to the solution to meet desired nutrient requirements.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Fertilizer {
    pub(super) id: String,
    pub(super) name: String,
    pub(super) vendor: String,
    pub(super) liquid: bool,
    pub(super) source_composition: SourceComposition,
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

    pub fn liquid(&self) -> bool {
        self.liquid
    }

    /// Returns the source composition from which the nutrient content was derived
    pub fn source_composition(&self) -> &SourceComposition {
        &self.source_composition
    }

    /// Returns the set of nutrients contained within fertilizer
    pub fn nutrients(&self) -> &Nutrients {
        &self.nutrients
    }

    /// Compares the total nutrient amount to the given `fertilizer`.
    /// Returns `true` if this fertilizer is richer, and `false` otherwise.
    pub fn compare(&self, fertilizer: &Fertilizer) -> bool {
        self.nutrients().total_amount() >= fertilizer.nutrients().total_amount()
    }
}

impl Default for Fertilizer {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            vendor: String::new(),
            liquid: false,
            source_composition: SourceComposition::default(),
            nutrients: Nutrients::new(),
        }
    }
}
