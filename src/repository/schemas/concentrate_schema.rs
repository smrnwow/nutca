use super::{ConcentrateCompositionSchema, ConcentratePartSchema};
use crate::model::concentrates::{Concentrate, ConcentrateSummary};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ConcentrateSchema {
    pub id: String,
    pub name: String,
    pub composition: ConcentrateCompositionSchema,
    pub parts: Vec<ConcentratePartSchema>,
}

impl From<Concentrate> for ConcentrateSchema {
    fn from(concentrate: Concentrate) -> Self {
        Self {
            id: concentrate.id().clone(),
            name: concentrate.name().clone(),
            composition: ConcentrateCompositionSchema::from(concentrate.composition().clone()),
            parts: concentrate
                .parts()
                .into_iter()
                .map(|part| ConcentratePartSchema::from(part.clone()))
                .collect(),
        }
    }
}

impl Into<ConcentrateSummary> for ConcentrateSchema {
    fn into(self) -> ConcentrateSummary {
        ConcentrateSummary {
            id: self.id,
            name: self.name,
        }
    }
}

impl Default for ConcentrateSchema {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            composition: ConcentrateCompositionSchema::default(),
            parts: Vec::new(),
        }
    }
}
