use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum VolumeUnits {
    Litres,
    Gallons,
}

impl VolumeUnits {
    pub fn label(&self) -> String {
        match self {
            Self::Litres => String::from("литры"),
            Self::Gallons => String::from("галлоны"),
        }
    }
}

impl Into<String> for VolumeUnits {
    fn into(self) -> String {
        match self {
            Self::Litres => String::from("l"),
            Self::Gallons => String::from("gl"),
        }
    }
}

impl From<String> for VolumeUnits {
    fn from(value: String) -> Self {
        match value.as_str() {
            "gl" => Self::Gallons,
            _ => Self::Litres,
        }
    }
}
