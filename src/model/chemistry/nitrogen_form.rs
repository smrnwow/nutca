use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum NitrogenForm {
    Nitrate(f64),
    Ammonium(f64),
}

impl NitrogenForm {
    pub fn symbol(&self) -> String {
        match self {
            Self::Nitrate(_) => String::from("NO3"),
            Self::Ammonium(_) => String::from("NH4"),
        }
    }

    pub fn new(&self, value: f64) -> Self {
        match self {
            Self::Nitrate(_) => Self::Nitrate(value),
            Self::Ammonium(_) => Self::Ammonium(value),
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Self::Nitrate(_) => 0,
            Self::Ammonium(_) => 1,
        }
    }

    pub fn add(&self, amount: f64) -> Self {
        match self {
            Self::Nitrate(value) => Self::Nitrate(value + amount),
            Self::Ammonium(value) => Self::Ammonium(value + amount),
        }
    }

    pub fn value(&self) -> f64 {
        match self {
            Self::Nitrate(value) => *value,
            Self::Ammonium(value) => *value,
        }
    }
}
