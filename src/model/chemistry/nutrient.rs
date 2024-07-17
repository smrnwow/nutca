use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq)]
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
}
