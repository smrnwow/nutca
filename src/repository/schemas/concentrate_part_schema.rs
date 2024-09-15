use crate::model::chemistry::Volume;
use crate::model::concentrates::Part;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ConcentratePartSchema {
    id: String,
    name: String,
    concentration: usize,
    volume: Volume,
}

impl From<Part> for ConcentratePartSchema {
    fn from(part: Part) -> Self {
        Self {
            id: part.id().clone(),
            name: part.name().clone(),
            concentration: part.concentration().clone(),
            volume: part.volume().clone(),
        }
    }
}

impl Into<Part> for ConcentratePartSchema {
    fn into(self) -> Part {
        Part::from(self.id, self.name, self.concentration, self.volume)
    }
}
