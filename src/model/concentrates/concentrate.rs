use super::fillers::ManualFiller;
use crate::model::concentrates::fillers::Filler;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Concentrate {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) filler: Filler,
}

impl Concentrate {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn filler(&self) -> &Filler {
        &self.filler
    }
}

impl Default for Concentrate {
    fn default() -> Self {
        Concentrate {
            id: String::new(),
            name: String::new(),
            filler: Filler::Manual(ManualFiller::new()),
        }
    }
}
