use crate::model::chemistry::Nutrient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq)]
pub enum NutrientAmount {
    Nitrogen(f64),
    NitrogenNitrate(f64),
    NitrogenAmmonium(f64),
    Phosphorus(f64),
    Potassium(f64),
    Calcium(f64),
    Magnesium(f64),
    Sulfur(f64),
    Iron(f64),
    Manganese(f64),
    Copper(f64),
    Zinc(f64),
    Boron(f64),
    Molybdenum(f64),
}

impl NutrientAmount {
    pub fn nutrient(&self) -> Nutrient {
        match self {
            Self::Nitrogen(_) => Nutrient::Nitrogen,
            Self::NitrogenNitrate(_) => Nutrient::NitrogenNitrate,
            Self::NitrogenAmmonium(_) => Nutrient::NitrogenAmmonium,
            Self::Phosphorus(_) => Nutrient::Phosphorus,
            Self::Potassium(_) => Nutrient::Potassium,
            Self::Calcium(_) => Nutrient::Calcium,
            Self::Magnesium(_) => Nutrient::Magnesium,
            Self::Sulfur(_) => Nutrient::Sulfur,
            Self::Iron(_) => Nutrient::Iron,
            Self::Manganese(_) => Nutrient::Manganese,
            Self::Copper(_) => Nutrient::Copper,
            Self::Zinc(_) => Nutrient::Zinc,
            Self::Boron(_) => Nutrient::Boron,
            Self::Molybdenum(_) => Nutrient::Molybdenum,
        }
    }

    pub fn value(&self) -> f64 {
        match self {
            Self::Nitrogen(value) => *value,
            Self::NitrogenNitrate(value) => *value,
            Self::NitrogenAmmonium(value) => *value,
            Self::Phosphorus(value) => *value,
            Self::Potassium(value) => *value,
            Self::Calcium(value) => *value,
            Self::Magnesium(value) => *value,
            Self::Sulfur(value) => *value,
            Self::Iron(value) => *value,
            Self::Manganese(value) => *value,
            Self::Copper(value) => *value,
            Self::Zinc(value) => *value,
            Self::Boron(value) => *value,
            Self::Molybdenum(value) => *value,
        }
    }

    pub fn factor(&self) -> f64 {
        self.value() * 10.
    }

    pub fn new(&self, value: f64) -> Self {
        match self {
            Self::Nitrogen(_) => Self::Nitrogen(value),
            Self::NitrogenNitrate(_) => Self::NitrogenNitrate(value),
            Self::NitrogenAmmonium(_) => Self::NitrogenAmmonium(value),
            Self::Phosphorus(_) => Self::Phosphorus(value),
            Self::Potassium(_) => Self::Potassium(value),
            Self::Calcium(_) => Self::Calcium(value),
            Self::Magnesium(_) => Self::Magnesium(value),
            Self::Sulfur(_) => Self::Sulfur(value),
            Self::Iron(_) => Self::Iron(value),
            Self::Manganese(_) => Self::Manganese(value),
            Self::Copper(_) => Self::Copper(value),
            Self::Zinc(_) => Self::Zinc(value),
            Self::Boron(_) => Self::Boron(value),
            Self::Molybdenum(_) => Self::Molybdenum(value),
        }
    }

    pub fn add(&self, amount: f64) -> Self {
        match self {
            Self::Nitrogen(value) => Self::Nitrogen(value + amount),
            Self::NitrogenNitrate(value) => Self::NitrogenNitrate(value + amount),
            Self::NitrogenAmmonium(value) => Self::NitrogenAmmonium(value + amount),
            Self::Phosphorus(value) => Self::Phosphorus(value + amount),
            Self::Potassium(value) => Self::Potassium(value + amount),
            Self::Calcium(value) => Self::Calcium(value + amount),
            Self::Magnesium(value) => Self::Magnesium(value + amount),
            Self::Sulfur(value) => Self::Sulfur(value + amount),
            Self::Iron(value) => Self::Iron(value + amount),
            Self::Manganese(value) => Self::Manganese(value + amount),
            Self::Copper(value) => Self::Copper(value + amount),
            Self::Zinc(value) => Self::Zinc(value + amount),
            Self::Boron(value) => Self::Boron(value + amount),
            Self::Molybdenum(value) => Self::Molybdenum(value + amount),
        }
    }

    pub fn plus(&self, amount: f64) -> Self {
        match self {
            Self::Nitrogen(value) => Self::Nitrogen(value + amount),
            Self::NitrogenNitrate(value) => Self::NitrogenNitrate(value + amount),
            Self::NitrogenAmmonium(value) => Self::NitrogenAmmonium(value + amount),
            Self::Phosphorus(value) => Self::Phosphorus(value + amount),
            Self::Potassium(value) => Self::Potassium(value + amount),
            Self::Calcium(value) => Self::Calcium(value + amount),
            Self::Magnesium(value) => Self::Magnesium(value + amount),
            Self::Sulfur(value) => Self::Sulfur(value + amount),
            Self::Iron(value) => Self::Iron(value + amount),
            Self::Manganese(value) => Self::Manganese(value + amount),
            Self::Copper(value) => Self::Copper(value + amount),
            Self::Zinc(value) => Self::Zinc(value + amount),
            Self::Boron(value) => Self::Boron(value + amount),
            Self::Molybdenum(value) => Self::Molybdenum(value + amount),
        }
    }

    pub fn minus(&self, amount: f64) -> Self {
        match self {
            Self::Nitrogen(value) => Self::Nitrogen(value - amount),
            Self::NitrogenNitrate(value) => Self::NitrogenNitrate(value - amount),
            Self::NitrogenAmmonium(value) => Self::NitrogenAmmonium(value - amount),
            Self::Phosphorus(value) => Self::Phosphorus(value - amount),
            Self::Potassium(value) => Self::Potassium(value - amount),
            Self::Calcium(value) => Self::Calcium(value - amount),
            Self::Magnesium(value) => Self::Magnesium(value - amount),
            Self::Sulfur(value) => Self::Sulfur(value - amount),
            Self::Iron(value) => Self::Iron(value - amount),
            Self::Manganese(value) => Self::Manganese(value - amount),
            Self::Copper(value) => Self::Copper(value - amount),
            Self::Zinc(value) => Self::Zinc(value - amount),
            Self::Boron(value) => Self::Boron(value - amount),
            Self::Molybdenum(value) => Self::Molybdenum(value - amount),
        }
    }

    pub fn multiple(&self, factor: f64) -> Self {
        match self {
            Self::Nitrogen(value) => Self::Nitrogen(value * factor),
            Self::NitrogenNitrate(value) => Self::NitrogenNitrate(value * factor),
            Self::NitrogenAmmonium(value) => Self::NitrogenAmmonium(value * factor),
            Self::Phosphorus(value) => Self::Phosphorus(value * factor),
            Self::Potassium(value) => Self::Potassium(value * factor),
            Self::Calcium(value) => Self::Calcium(value * factor),
            Self::Magnesium(value) => Self::Magnesium(value * factor),
            Self::Sulfur(value) => Self::Sulfur(value * factor),
            Self::Iron(value) => Self::Iron(value * factor),
            Self::Manganese(value) => Self::Manganese(value * factor),
            Self::Copper(value) => Self::Copper(value * factor),
            Self::Zinc(value) => Self::Zinc(value * factor),
            Self::Boron(value) => Self::Boron(value * factor),
            Self::Molybdenum(value) => Self::Molybdenum(value * factor),
        }
    }
}
