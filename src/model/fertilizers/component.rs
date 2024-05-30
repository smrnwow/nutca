use super::NutrientContent;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Component {
    Nitrogen(f64),
    NitrogenNitrate(f64),
    NitrogenAmmonium(f64),
    Phosphor(f64),
    PhosphorPentoxide(f64),
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

impl Component {
    pub fn symbol(&self) -> &str {
        match self {
            Component::Nitrogen(_) => "N",
            Component::NitrogenNitrate(_) => "NO3",
            Component::NitrogenAmmonium(_) => "NH4",
            Component::Phosphor(_) => "P",
            Component::PhosphorPentoxide(_) => "P2O5",
            Component::Potassium(_) => "K",
            Component::PotassiumOxide(_) => "K2O",
            Component::Calcium(_) => "Ca",
            Component::CalciumOxide(_) => "CaO",
            Component::Magnesium(_) => "Mg",
            Component::MagnesiumOxide(_) => "MgO",
            Component::Sulfur(_) => "S",
            Component::SulfurTrioxide(_) => "SO3",
            Component::SulfurTetroxide(_) => "SO4",
            Component::Iron(_) => "Fe",
            Component::Manganese(_) => "Mn",
            Component::Copper(_) => "Cu",
            Component::Zinc(_) => "Zn",
            Component::Boron(_) => "B",
            Component::Molybdenum(_) => "Mo",
        }
    }

    pub fn new(&self, value: f64) -> Self {
        match self {
            Component::Nitrogen(_) => Component::Nitrogen(value),
            Component::NitrogenNitrate(_) => Component::NitrogenNitrate(value),
            Component::NitrogenAmmonium(_) => Component::NitrogenAmmonium(value),
            Component::Phosphor(_) => Component::Phosphor(value),
            Component::PhosphorPentoxide(_) => Component::PhosphorPentoxide(value),
            Component::Potassium(_) => Component::Potassium(value),
            Component::PotassiumOxide(_) => Component::PotassiumOxide(value),
            Component::Calcium(_) => Component::Calcium(value),
            Component::CalciumOxide(_) => Component::CalciumOxide(value),
            Component::Magnesium(_) => Component::Magnesium(value),
            Component::MagnesiumOxide(_) => Component::MagnesiumOxide(value),
            Component::Sulfur(_) => Component::Sulfur(value),
            Component::SulfurTrioxide(_) => Component::SulfurTrioxide(value),
            Component::SulfurTetroxide(_) => Component::SulfurTetroxide(value),
            Component::Iron(_) => Component::Iron(value),
            Component::Manganese(_) => Component::Manganese(value),
            Component::Copper(_) => Component::Copper(value),
            Component::Zinc(_) => Component::Zinc(value),
            Component::Boron(_) => Component::Boron(value),
            Component::Molybdenum(_) => Component::Molybdenum(value),
        }
    }

    pub fn value(&self) -> f64 {
        match self {
            Component::Nitrogen(value) => *value,
            Component::NitrogenNitrate(value) => *value,
            Component::NitrogenAmmonium(value) => *value,
            Component::Phosphor(value) => *value,
            Component::PhosphorPentoxide(value) => *value,
            Component::Potassium(value) => *value,
            Component::PotassiumOxide(value) => *value,
            Component::Calcium(value) => *value,
            Component::CalciumOxide(value) => *value,
            Component::Magnesium(value) => *value,
            Component::MagnesiumOxide(value) => *value,
            Component::Sulfur(value) => *value,
            Component::SulfurTrioxide(value) => *value,
            Component::SulfurTetroxide(value) => *value,
            Component::Iron(value) => *value,
            Component::Manganese(value) => *value,
            Component::Copper(value) => *value,
            Component::Zinc(value) => *value,
            Component::Boron(value) => *value,
            Component::Molybdenum(value) => *value,
        }
    }

    pub fn nutrient_content(&self) -> NutrientContent {
        match self {
            Component::Nitrogen(value) => NutrientContent::Nitrogen(*value),
            Component::NitrogenNitrate(value) => NutrientContent::NitrogenNitrate(*value),
            Component::NitrogenAmmonium(value) => NutrientContent::NitrogenAmmonium(*value),
            Component::Phosphor(value) => NutrientContent::Phosphor(*value),
            Component::PhosphorPentoxide(value) => NutrientContent::Phosphor(*value * 0.436421),
            Component::Potassium(value) => NutrientContent::Potassium(*value),
            Component::PotassiumOxide(value) => NutrientContent::Potassium(*value * 0.830148),
            Component::Calcium(value) => NutrientContent::Calcium(*value),
            Component::CalciumOxide(value) => NutrientContent::Calcium(*value * 0.714691),
            Component::Magnesium(value) => NutrientContent::Magnesium(*value),
            Component::MagnesiumOxide(value) => NutrientContent::Magnesium(*value * 0.603036),
            Component::Sulfur(value) => NutrientContent::Sulfur(*value),
            Component::SulfurTrioxide(value) => NutrientContent::Sulfur(*value * 0.400496),
            Component::SulfurTetroxide(value) => NutrientContent::Sulfur(*value * 0.333793),
            Component::Iron(value) => NutrientContent::Iron(*value),
            Component::Manganese(value) => NutrientContent::Manganese(*value),
            Component::Copper(value) => NutrientContent::Copper(*value),
            Component::Zinc(value) => NutrientContent::Zinc(*value),
            Component::Boron(value) => NutrientContent::Boron(*value),
            Component::Molybdenum(value) => NutrientContent::Molybdenum(*value),
        }
    }

    pub fn index(&self) -> usize {
        match self {
            Component::Nitrogen(_) => 0,
            Component::NitrogenNitrate(_) => 1,
            Component::NitrogenAmmonium(_) => 2,
            Component::Phosphor(_) => 3,
            Component::PhosphorPentoxide(_) => 4,
            Component::Potassium(_) => 5,
            Component::PotassiumOxide(_) => 6,
            Component::Calcium(_) => 7,
            Component::CalciumOxide(_) => 8,
            Component::Magnesium(_) => 9,
            Component::MagnesiumOxide(_) => 10,
            Component::Sulfur(_) => 11,
            Component::SulfurTrioxide(_) => 12,
            Component::SulfurTetroxide(_) => 13,
            Component::Iron(_) => 14,
            Component::Manganese(_) => 15,
            Component::Copper(_) => 16,
            Component::Zinc(_) => 17,
            Component::Boron(_) => 18,
            Component::Molybdenum(_) => 19,
        }
    }
}
