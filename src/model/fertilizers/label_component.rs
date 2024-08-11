use crate::model::chemistry::NutrientAmount;
use serde::{Deserialize, Serialize};

/// An enumeration of nutrients in different forms within fertilizer which listed on its label
#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum LabelComponent {
    Nitrogen(f64),
    NitrogenNitrate(f64),
    NitrogenAmmonium(f64),
    Phosphorus(f64),
    PhosphorusPentoxide(f64),
    Potassium(f64),
    PotassiumOxide(f64),
    Calcium(f64),
    CalciumOxide(f64),
    Magnesium(f64),
    MagnesiumOxide(f64),
    Sulfur(f64),
    SulfurTrioxide(f64),
    SulfurTetroxide(f64),
    Iron(f64),
    Manganese(f64),
    Copper(f64),
    Zinc(f64),
    Boron(f64),
    Molybdenum(f64),
}

impl LabelComponent {
    pub fn symbol(&self) -> &str {
        match self {
            Self::Nitrogen(_) => "N",
            Self::NitrogenNitrate(_) => "NO3",
            Self::NitrogenAmmonium(_) => "NH4",
            Self::Phosphorus(_) => "P",
            Self::PhosphorusPentoxide(_) => "P2O5",
            Self::Potassium(_) => "K",
            Self::PotassiumOxide(_) => "K2O",
            Self::Calcium(_) => "Ca",
            Self::CalciumOxide(_) => "CaO",
            Self::Magnesium(_) => "Mg",
            Self::MagnesiumOxide(_) => "MgO",
            Self::Sulfur(_) => "S",
            Self::SulfurTrioxide(_) => "SO3",
            Self::SulfurTetroxide(_) => "SO4",
            Self::Iron(_) => "Fe",
            Self::Manganese(_) => "Mn",
            Self::Copper(_) => "Cu",
            Self::Zinc(_) => "Zn",
            Self::Boron(_) => "B",
            Self::Molybdenum(_) => "Mo",
        }
    }

    pub fn new(&self, value: f64) -> Self {
        match self {
            Self::Nitrogen(_) => Self::Nitrogen(value),
            Self::NitrogenNitrate(_) => Self::NitrogenNitrate(value),
            Self::NitrogenAmmonium(_) => Self::NitrogenAmmonium(value),
            Self::Phosphorus(_) => Self::Phosphorus(value),
            Self::PhosphorusPentoxide(_) => Self::PhosphorusPentoxide(value),
            Self::Potassium(_) => Self::Potassium(value),
            Self::PotassiumOxide(_) => Self::PotassiumOxide(value),
            Self::Calcium(_) => Self::Calcium(value),
            Self::CalciumOxide(_) => Self::CalciumOxide(value),
            Self::Magnesium(_) => Self::Magnesium(value),
            Self::MagnesiumOxide(_) => Self::MagnesiumOxide(value),
            Self::Sulfur(_) => Self::Sulfur(value),
            Self::SulfurTrioxide(_) => Self::SulfurTrioxide(value),
            Self::SulfurTetroxide(_) => Self::SulfurTetroxide(value),
            Self::Iron(_) => Self::Iron(value),
            Self::Manganese(_) => Self::Manganese(value),
            Self::Copper(_) => Self::Copper(value),
            Self::Zinc(_) => Self::Zinc(value),
            Self::Boron(_) => Self::Boron(value),
            Self::Molybdenum(_) => Self::Molybdenum(value),
        }
    }

    pub fn value(&self) -> f64 {
        match self {
            Self::Nitrogen(value) => *value,
            Self::NitrogenNitrate(value) => *value,
            Self::NitrogenAmmonium(value) => *value,
            Self::Phosphorus(value) => *value,
            Self::PhosphorusPentoxide(value) => *value,
            Self::Potassium(value) => *value,
            Self::PotassiumOxide(value) => *value,
            Self::Calcium(value) => *value,
            Self::CalciumOxide(value) => *value,
            Self::Magnesium(value) => *value,
            Self::MagnesiumOxide(value) => *value,
            Self::Sulfur(value) => *value,
            Self::SulfurTrioxide(value) => *value,
            Self::SulfurTetroxide(value) => *value,
            Self::Iron(value) => *value,
            Self::Manganese(value) => *value,
            Self::Copper(value) => *value,
            Self::Zinc(value) => *value,
            Self::Boron(value) => *value,
            Self::Molybdenum(value) => *value,
        }
    }

    pub fn nutrient(&self) -> NutrientAmount {
        match self {
            Self::Nitrogen(value) => NutrientAmount::Nitrogen(*value),
            Self::NitrogenNitrate(value) => NutrientAmount::NitrogenNitrate(*value),
            Self::NitrogenAmmonium(value) => NutrientAmount::NitrogenAmmonium(*value),
            Self::Phosphorus(value) => NutrientAmount::Phosphorus(*value),
            Self::PhosphorusPentoxide(value) => NutrientAmount::Phosphorus(*value * 0.436421),
            Self::Potassium(value) => NutrientAmount::Potassium(*value),
            Self::PotassiumOxide(value) => NutrientAmount::Potassium(*value * 0.830148),
            Self::Calcium(value) => NutrientAmount::Calcium(*value),
            Self::CalciumOxide(value) => NutrientAmount::Calcium(*value * 0.714691),
            Self::Magnesium(value) => NutrientAmount::Magnesium(*value),
            Self::MagnesiumOxide(value) => NutrientAmount::Magnesium(*value * 0.603036),
            Self::Sulfur(value) => NutrientAmount::Sulfur(*value),
            Self::SulfurTrioxide(value) => NutrientAmount::Sulfur(*value * 0.400496),
            Self::SulfurTetroxide(value) => NutrientAmount::Sulfur(*value * 0.333793),
            Self::Iron(value) => NutrientAmount::Iron(*value),
            Self::Manganese(value) => NutrientAmount::Manganese(*value),
            Self::Copper(value) => NutrientAmount::Copper(*value),
            Self::Zinc(value) => NutrientAmount::Zinc(*value),
            Self::Boron(value) => NutrientAmount::Boron(*value),
            Self::Molybdenum(value) => NutrientAmount::Molybdenum(*value),
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Self::Nitrogen(_) => 0,
            Self::NitrogenNitrate(_) => 1,
            Self::NitrogenAmmonium(_) => 2,
            Self::Phosphorus(_) => 3,
            Self::PhosphorusPentoxide(_) => 4,
            Self::Potassium(_) => 5,
            Self::PotassiumOxide(_) => 6,
            Self::Calcium(_) => 7,
            Self::CalciumOxide(_) => 8,
            Self::Magnesium(_) => 9,
            Self::MagnesiumOxide(_) => 10,
            Self::Sulfur(_) => 11,
            Self::SulfurTrioxide(_) => 12,
            Self::SulfurTetroxide(_) => 13,
            Self::Iron(_) => 14,
            Self::Manganese(_) => 15,
            Self::Copper(_) => 16,
            Self::Zinc(_) => 17,
            Self::Boron(_) => 18,
            Self::Molybdenum(_) => 19,
        }
    }
}
