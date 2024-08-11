use crate::model::fertilizers::{Formula, Label, SourceType};
use serde::{Deserialize, Serialize};

/// An enumeration representing possible sources of fertilizer composition.
///
/// - **Label**: a commercial fertilizer label, containing information about composition.
/// - **Formula**: a chemical formula, specifying a compound that can be used as a fertilizer.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum SourceComposition {
    Label(Label),
    Formula(Formula),
}

impl SourceComposition {
    pub fn source_type(&self) -> SourceType {
        match self {
            Self::Label(_) => SourceType::Label,
            Self::Formula(_) => SourceType::Formula,
        }
    }
}

impl Default for SourceComposition {
    fn default() -> Self {
        Self::Label(Label::default())
    }
}
