use crate::model::fertilizers::Nutrient;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum NutrientRequirement {
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

impl NutrientRequirement {
    pub fn symbol(&self) -> &str {
        match self {
            NutrientRequirement::Nitrogen(_) => "N",
            NutrientRequirement::NitrogenNitrate(_) => "NO3",
            NutrientRequirement::NitrogenAmmonium(_) => "NH4",
            NutrientRequirement::Phosphor(_) => "P",
            NutrientRequirement::Potassium(_) => "K",
            NutrientRequirement::Calcium(_) => "Ca",
            NutrientRequirement::Magnesium(_) => "Mg",
            NutrientRequirement::Sulfur(_) => "S",
            NutrientRequirement::Iron(_) => "Fe",
            NutrientRequirement::Manganese(_) => "Mn",
            NutrientRequirement::Copper(_) => "Cu",
            NutrientRequirement::Zinc(_) => "Zn",
            NutrientRequirement::Boron(_) => "B",
            NutrientRequirement::Molybdenum(_) => "Mo",
        }
    }

    pub fn amount(&self) -> f64 {
        match self {
            NutrientRequirement::Nitrogen(amount) => *amount,
            NutrientRequirement::NitrogenNitrate(amount) => *amount,
            NutrientRequirement::NitrogenAmmonium(amount) => *amount,
            NutrientRequirement::Phosphor(amount) => *amount,
            NutrientRequirement::Potassium(amount) => *amount,
            NutrientRequirement::Calcium(amount) => *amount,
            NutrientRequirement::Magnesium(amount) => *amount,
            NutrientRequirement::Sulfur(amount) => *amount,
            NutrientRequirement::Iron(amount) => *amount,
            NutrientRequirement::Manganese(amount) => *amount,
            NutrientRequirement::Copper(amount) => *amount,
            NutrientRequirement::Zinc(amount) => *amount,
            NutrientRequirement::Boron(amount) => *amount,
            NutrientRequirement::Molybdenum(amount) => *amount,
        }
    }

    pub fn new(&self, amount: f64) -> Self {
        match self {
            NutrientRequirement::Nitrogen(_) => NutrientRequirement::Nitrogen(amount),
            NutrientRequirement::NitrogenNitrate(_) => NutrientRequirement::NitrogenNitrate(amount),
            NutrientRequirement::NitrogenAmmonium(_) => {
                NutrientRequirement::NitrogenAmmonium(amount)
            }
            NutrientRequirement::Phosphor(_) => NutrientRequirement::Phosphor(amount),
            NutrientRequirement::Potassium(_) => NutrientRequirement::Potassium(amount),
            NutrientRequirement::Calcium(_) => NutrientRequirement::Calcium(amount),
            NutrientRequirement::Magnesium(_) => NutrientRequirement::Magnesium(amount),
            NutrientRequirement::Sulfur(_) => NutrientRequirement::Sulfur(amount),
            NutrientRequirement::Iron(_) => NutrientRequirement::Iron(amount),
            NutrientRequirement::Manganese(_) => NutrientRequirement::Manganese(amount),
            NutrientRequirement::Copper(_) => NutrientRequirement::Copper(amount),
            NutrientRequirement::Zinc(_) => NutrientRequirement::Zinc(amount),
            NutrientRequirement::Boron(_) => NutrientRequirement::Boron(amount),
            NutrientRequirement::Molybdenum(_) => NutrientRequirement::Molybdenum(amount),
        }
    }

    pub fn add(&self, amount: f64) -> Self {
        match self {
            NutrientRequirement::Nitrogen(value) => NutrientRequirement::Nitrogen(value + amount),
            NutrientRequirement::NitrogenNitrate(value) => {
                NutrientRequirement::NitrogenNitrate(value + amount)
            }
            NutrientRequirement::NitrogenAmmonium(value) => {
                NutrientRequirement::NitrogenAmmonium(value + amount)
            }
            NutrientRequirement::Phosphor(value) => NutrientRequirement::Phosphor(value + amount),
            NutrientRequirement::Potassium(value) => NutrientRequirement::Potassium(value + amount),
            NutrientRequirement::Calcium(value) => NutrientRequirement::Calcium(value + amount),
            NutrientRequirement::Magnesium(value) => NutrientRequirement::Magnesium(value + amount),
            NutrientRequirement::Sulfur(value) => NutrientRequirement::Sulfur(value + amount),
            NutrientRequirement::Iron(value) => NutrientRequirement::Iron(value + amount),
            NutrientRequirement::Manganese(value) => NutrientRequirement::Manganese(value + amount),
            NutrientRequirement::Copper(value) => NutrientRequirement::Copper(value + amount),
            NutrientRequirement::Zinc(value) => NutrientRequirement::Zinc(value + amount),
            NutrientRequirement::Boron(value) => NutrientRequirement::Boron(value + amount),
            NutrientRequirement::Molybdenum(value) => {
                NutrientRequirement::Molybdenum(value + amount)
            }
        }
    }

    pub fn nutrient(&self) -> Nutrient {
        match self {
            NutrientRequirement::Nitrogen(_) => Nutrient::Nitrogen,
            NutrientRequirement::NitrogenNitrate(_) => Nutrient::NitrogenNitrate,
            NutrientRequirement::NitrogenAmmonium(_) => Nutrient::NitrogenAmmonium,
            NutrientRequirement::Phosphor(_) => Nutrient::Phosphor,
            NutrientRequirement::Potassium(_) => Nutrient::Potassium,
            NutrientRequirement::Calcium(_) => Nutrient::Calcium,
            NutrientRequirement::Magnesium(_) => Nutrient::Magnesium,
            NutrientRequirement::Sulfur(_) => Nutrient::Sulfur,
            NutrientRequirement::Iron(_) => Nutrient::Iron,
            NutrientRequirement::Manganese(_) => Nutrient::Manganese,
            NutrientRequirement::Copper(_) => Nutrient::Copper,
            NutrientRequirement::Zinc(_) => Nutrient::Zinc,
            NutrientRequirement::Boron(_) => Nutrient::Boron,
            NutrientRequirement::Molybdenum(_) => Nutrient::Molybdenum,
        }
    }

    pub fn index(&self) -> usize {
        match self {
            NutrientRequirement::Nitrogen(_) => 0,
            NutrientRequirement::NitrogenNitrate(_) => 1,
            NutrientRequirement::NitrogenAmmonium(_) => 2,
            NutrientRequirement::Phosphor(_) => 3,
            NutrientRequirement::Potassium(_) => 4,
            NutrientRequirement::Calcium(_) => 5,
            NutrientRequirement::Magnesium(_) => 6,
            NutrientRequirement::Sulfur(_) => 7,
            NutrientRequirement::Iron(_) => 8,
            NutrientRequirement::Manganese(_) => 9,
            NutrientRequirement::Copper(_) => 10,
            NutrientRequirement::Zinc(_) => 11,
            NutrientRequirement::Boron(_) => 12,
            NutrientRequirement::Molybdenum(_) => 13,
        }
    }
}
