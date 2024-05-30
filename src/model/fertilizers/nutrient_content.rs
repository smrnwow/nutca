use super::Nutrient;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum NutrientContent {
    Nitrogen(f64),
    NitrogenNitrate(f64),
    NitrogenAmmonium(f64),
    Phosphor(f64),
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

impl NutrientContent {
    pub fn symbol(&self) -> &str {
        match self {
            NutrientContent::Nitrogen(_) => "N",
            NutrientContent::NitrogenNitrate(_) => "NO3",
            NutrientContent::NitrogenAmmonium(_) => "NH4",
            NutrientContent::Phosphor(_) => "P",
            NutrientContent::Potassium(_) => "K",
            NutrientContent::Calcium(_) => "Ca",
            NutrientContent::Magnesium(_) => "Mg",
            NutrientContent::Sulfur(_) => "S",
            NutrientContent::Iron(_) => "Fe",
            NutrientContent::Manganese(_) => "Mn",
            NutrientContent::Copper(_) => "Cu",
            NutrientContent::Zinc(_) => "Zn",
            NutrientContent::Boron(_) => "B",
            NutrientContent::Molybdenum(_) => "Mo",
        }
    }

    pub fn value(&self) -> f64 {
        match self {
            NutrientContent::Nitrogen(value) => *value,
            NutrientContent::NitrogenNitrate(value) => *value,
            NutrientContent::NitrogenAmmonium(value) => *value,
            NutrientContent::Phosphor(value) => *value,
            NutrientContent::Potassium(value) => *value,
            NutrientContent::Calcium(value) => *value,
            NutrientContent::Magnesium(value) => *value,
            NutrientContent::Sulfur(value) => *value,
            NutrientContent::Iron(value) => *value,
            NutrientContent::Manganese(value) => *value,
            NutrientContent::Copper(value) => *value,
            NutrientContent::Zinc(value) => *value,
            NutrientContent::Boron(value) => *value,
            NutrientContent::Molybdenum(value) => *value,
        }
    }

    pub fn nutrient(&self) -> Nutrient {
        match self {
            NutrientContent::Nitrogen(_) => Nutrient::Nitrogen,
            NutrientContent::NitrogenNitrate(_) => Nutrient::NitrogenNitrate,
            NutrientContent::NitrogenAmmonium(_) => Nutrient::NitrogenAmmonium,
            NutrientContent::Phosphor(_) => Nutrient::Phosphor,
            NutrientContent::Potassium(_) => Nutrient::Potassium,
            NutrientContent::Calcium(_) => Nutrient::Calcium,
            NutrientContent::Magnesium(_) => Nutrient::Magnesium,
            NutrientContent::Sulfur(_) => Nutrient::Sulfur,
            NutrientContent::Iron(_) => Nutrient::Iron,
            NutrientContent::Manganese(_) => Nutrient::Manganese,
            NutrientContent::Copper(_) => Nutrient::Copper,
            NutrientContent::Zinc(_) => Nutrient::Zinc,
            NutrientContent::Boron(_) => Nutrient::Boron,
            NutrientContent::Molybdenum(_) => Nutrient::Molybdenum,
        }
    }
}
