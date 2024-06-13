use crate::model::chemistry::Nutrient;
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

    pub fn nutrient(&self) -> Nutrient {
        match self {
            Component::Nitrogen(value) => Nutrient::Nitrogen(*value),
            Component::NitrogenNitrate(value) => Nutrient::NitrogenNitrate(*value),
            Component::NitrogenAmmonium(value) => Nutrient::NitrogenAmmonium(*value),
            Component::Phosphor(value) => Nutrient::Phosphorus(*value),
            Component::PhosphorPentoxide(value) => Nutrient::Phosphorus(*value * 0.436421),
            Component::Potassium(value) => Nutrient::Potassium(*value),
            Component::PotassiumOxide(value) => Nutrient::Potassium(*value * 0.830148),
            Component::Calcium(value) => Nutrient::Calcium(*value),
            Component::CalciumOxide(value) => Nutrient::Calcium(*value * 0.714691),
            Component::Magnesium(value) => Nutrient::Magnesium(*value),
            Component::MagnesiumOxide(value) => Nutrient::Magnesium(*value * 0.603036),
            Component::Sulfur(value) => Nutrient::Sulfur(*value),
            Component::SulfurTrioxide(value) => Nutrient::Sulfur(*value * 0.400496),
            Component::SulfurTetroxide(value) => Nutrient::Sulfur(*value * 0.333793),
            Component::Iron(value) => Nutrient::Iron(*value),
            Component::Manganese(value) => Nutrient::Manganese(*value),
            Component::Copper(value) => Nutrient::Copper(*value),
            Component::Zinc(value) => Nutrient::Zinc(*value),
            Component::Boron(value) => Nutrient::Boron(*value),
            Component::Molybdenum(value) => Nutrient::Molybdenum(*value),
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
