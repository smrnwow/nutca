use crate::chemistry::Nutrient;
use std::{ops::AddAssign, ops::Mul};

#[derive(Copy, Clone, Debug)]
pub enum NutrientPercent {
    Nitrogen(f64),
    NitrogenNitrate(f64),
    NitrogenAmmonium(f64),
    Phosphorus(f64),
    Potassium(f64),
    Calcium(f64),
    Magnesium(f64),
    Sulfur(f64),
    Iron(f64),
    Zink(f64),
    Manganese(f64),
    Boron(f64),
    Copper(f64),
    Molybdenum(f64),
}

impl NutrientPercent {
    pub fn percent(&self) -> f64 {
        match self {
            Self::Nitrogen(percent) => *percent,
            Self::NitrogenNitrate(percent) => *percent,
            Self::NitrogenAmmonium(percent) => *percent,
            Self::Phosphorus(percent) => *percent,
            Self::Potassium(percent) => *percent,
            Self::Calcium(percent) => *percent,
            Self::Magnesium(percent) => *percent,
            Self::Sulfur(percent) => *percent,
            Self::Iron(percent) => *percent,
            Self::Boron(percent) => *percent,
            Self::Manganese(percent) => *percent,
            Self::Zink(percent) => *percent,
            Self::Copper(percent) => *percent,
            Self::Molybdenum(percent) => *percent,
        }
    }

    pub fn symbol(&self) -> Nutrient {
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
            Self::Boron(_) => Nutrient::Boron,
            Self::Manganese(_) => Nutrient::Manganese,
            Self::Zink(_) => Nutrient::Zink,
            Self::Copper(_) => Nutrient::Copper,
            Self::Molybdenum(_) => Nutrient::Molybdenum,
        }
    }
}

impl AddAssign for NutrientPercent {
    fn add_assign(&mut self, rhs: Self) {
        let new_percent_value = self.percent() + rhs.percent();

        *self = match self {
            Self::Nitrogen(_) => Self::Nitrogen(new_percent_value),
            Self::NitrogenNitrate(_) => Self::NitrogenNitrate(new_percent_value),
            Self::NitrogenAmmonium(_) => Self::NitrogenAmmonium(new_percent_value),
            Self::Phosphorus(_) => Self::Phosphorus(new_percent_value),
            Self::Potassium(_) => Self::Potassium(new_percent_value),
            Self::Calcium(_) => Self::Calcium(new_percent_value),
            Self::Magnesium(_) => Self::Magnesium(new_percent_value),
            Self::Sulfur(_) => Self::Sulfur(new_percent_value),
            Self::Iron(_) => Self::Iron(new_percent_value),
            Self::Boron(_) => Self::Boron(new_percent_value),
            Self::Manganese(_) => Self::Manganese(new_percent_value),
            Self::Zink(_) => Self::Zink(new_percent_value),
            Self::Copper(_) => Self::Copper(new_percent_value),
            Self::Molybdenum(_) => Self::Molybdenum(new_percent_value),
        }
    }
}

impl Mul<f64> for NutrientPercent {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let new_percent_value = self.percent() + rhs;

        match self {
            Self::Nitrogen(_) => Self::Nitrogen(new_percent_value),
            Self::NitrogenNitrate(_) => Self::NitrogenNitrate(new_percent_value),
            Self::NitrogenAmmonium(_) => Self::NitrogenAmmonium(new_percent_value),
            Self::Phosphorus(_) => Self::Phosphorus(new_percent_value),
            Self::Potassium(_) => Self::Potassium(new_percent_value),
            Self::Calcium(_) => Self::Calcium(new_percent_value),
            Self::Magnesium(_) => Self::Magnesium(new_percent_value),
            Self::Sulfur(_) => Self::Sulfur(new_percent_value),
            Self::Iron(_) => Self::Iron(new_percent_value),
            Self::Boron(_) => Self::Boron(new_percent_value),
            Self::Manganese(_) => Self::Manganese(new_percent_value),
            Self::Zink(_) => Self::Zink(new_percent_value),
            Self::Copper(_) => Self::Copper(new_percent_value),
            Self::Molybdenum(_) => Self::Molybdenum(new_percent_value),
        }
    }
}
