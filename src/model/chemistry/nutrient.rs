use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Nutrient {
    Nitrogen,
    NitrogenNitrate,
    NitrogenAmmonium,
    Phosphorus,
    Potassium,
    Calcium,
    Magnesium,
    Sulfur,
    Iron,
    Manganese,
    Copper,
    Zinc,
    Boron,
    Molybdenum,
}

impl Nutrient {
    pub fn index(&self) -> usize {
        match self {
            Self::Nitrogen => 0,
            Self::NitrogenNitrate => 1,
            Self::NitrogenAmmonium => 2,
            Self::Phosphorus => 3,
            Self::Potassium => 4,
            Self::Calcium => 5,
            Self::Magnesium => 6,
            Self::Sulfur => 7,
            Self::Iron => 8,
            Self::Manganese => 9,
            Self::Copper => 10,
            Self::Zinc => 11,
            Self::Boron => 12,
            Self::Molybdenum => 13,
        }
    }

    pub fn symbol(&self) -> &str {
        match self {
            Self::Nitrogen => "N",
            Self::NitrogenNitrate => "NO3",
            Self::NitrogenAmmonium => "NH4",
            Self::Phosphorus => "P",
            Self::Potassium => "K",
            Self::Calcium => "Ca",
            Self::Magnesium => "Mg",
            Self::Sulfur => "S",
            Self::Iron => "Fe",
            Self::Manganese => "Mn",
            Self::Copper => "Cu",
            Self::Zinc => "Zn",
            Self::Boron => "B",
            Self::Molybdenum => "Mo",
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::Nitrogen => String::from("Азот"),
            Self::NitrogenNitrate => String::from("Нитратный азот"),
            Self::NitrogenAmmonium => String::from("Аммонийный азот"),
            Self::Phosphorus => String::from("Фосфор"),
            Self::Potassium => String::from("Калий"),
            Self::Calcium => String::from("Кальций"),
            Self::Magnesium => String::from("Магний"),
            Self::Sulfur => String::from("Сера"),
            Self::Iron => String::from("Железо"),
            Self::Manganese => String::from("Марганец"),
            Self::Copper => String::from("Медь"),
            Self::Zinc => String::from("Цинк"),
            Self::Boron => String::from("Бор"),
            Self::Molybdenum => String::from("Молибден"),
        }
    }

    pub fn ionic_charge(&self) -> isize {
        match self {
            Self::Nitrogen => 0,
            Self::NitrogenNitrate => -1,
            Self::NitrogenAmmonium => 1,
            Self::Phosphorus => -1,
            Self::Potassium => 1,
            Self::Calcium => 2,
            Self::Magnesium => 2,
            Self::Sulfur => -2,
            Self::Iron => 2,
            Self::Manganese => 2,
            Self::Copper => 2,
            Self::Zinc => 2,
            Self::Boron => 0,
            Self::Molybdenum => 0,
        }
    }

    pub fn molar_mass(&self) -> f64 {
        match self {
            Self::Nitrogen => 14.007,
            Self::NitrogenNitrate => 14.007,
            Self::NitrogenAmmonium => 14.007,
            Self::Phosphorus => 30.973,
            Self::Potassium => 39.0983,
            Self::Calcium => 40.078,
            Self::Magnesium => 24.305,
            Self::Sulfur => 32.06,
            Self::Iron => 55.845,
            Self::Manganese => 54.938,
            Self::Copper => 63.546,
            Self::Zinc => 65.38,
            Self::Boron => 10.81,
            Self::Molybdenum => 95.95,
        }
    }

    pub fn limiting_molar_condictivity(&self) -> f64 {
        match self {
            Self::Nitrogen => 0.0,
            Self::NitrogenNitrate => 71.4,
            Self::NitrogenAmmonium => 74.4,
            Self::Phosphorus => 57.0,
            Self::Potassium => 73.6,
            Self::Calcium => 119.1,
            Self::Magnesium => 105.1,
            Self::Sulfur => 160.7,
            Self::Iron => 108.0,
            Self::Manganese => 103.4,
            Self::Copper => 110.1,
            Self::Zinc => 107.4,
            Self::Boron => 0.0,
            Self::Molybdenum => 298.0,
        }
    }
}
