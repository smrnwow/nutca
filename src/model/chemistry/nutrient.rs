use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq)]
pub enum Nutrient {
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

impl Nutrient {
    pub fn index(&self) -> usize {
        match self {
            Self::Nitrogen(_) => 0,
            Self::NitrogenNitrate(_) => 1,
            Self::NitrogenAmmonium(_) => 2,
            Self::Phosphorus(_) => 3,
            Self::Potassium(_) => 4,
            Self::Calcium(_) => 5,
            Self::Magnesium(_) => 6,
            Self::Sulfur(_) => 7,
            Self::Iron(_) => 8,
            Self::Manganese(_) => 9,
            Self::Copper(_) => 10,
            Self::Zinc(_) => 11,
            Self::Boron(_) => 12,
            Self::Molybdenum(_) => 13,
        }
    }

    pub fn symbol(&self) -> &str {
        match self {
            Self::Nitrogen(_) => "N",
            Self::NitrogenNitrate(_) => "NO3",
            Self::NitrogenAmmonium(_) => "NH4",
            Self::Phosphorus(_) => "P",
            Self::Potassium(_) => "K",
            Self::Calcium(_) => "Ca",
            Self::Magnesium(_) => "Mg",
            Self::Sulfur(_) => "S",
            Self::Iron(_) => "Fe",
            Self::Manganese(_) => "Mn",
            Self::Copper(_) => "Cu",
            Self::Zinc(_) => "Zn",
            Self::Boron(_) => "B",
            Self::Molybdenum(_) => "Mo",
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

    pub fn name(&self) -> String {
        match self {
            Self::Nitrogen(_) => String::from("Азот (N) - макроэлемент"),
            Self::NitrogenNitrate(_) => String::from("Нитратная форма азота (NO3)"),
            Self::NitrogenAmmonium(_) => String::from("Аммонийная форма азота (NH4)"),
            Self::Phosphorus(_) => String::from("Фосфор (P) - макроэлемен"),
            Self::Potassium(_) => String::from("Калий (K) - макроэлемент"),
            Self::Calcium(_) => String::from("Кальций (Ca) - макроэлемент"),
            Self::Magnesium(_) => String::from("Магний (Mg) - макроэлемент"),
            Self::Sulfur(_) => String::from("Сера (S) - макроэлемент"),
            Self::Iron(_) => String::from("Железо (Fe) - микроэлемент"),
            Self::Manganese(_) => String::from("Марганец (Mn) - микроэлемент"),
            Self::Copper(_) => String::from("Медь (Cu) - микроэлемент"),
            Self::Zinc(_) => String::from("Цинк (Zn) - микроэлемент"),
            Self::Boron(_) => String::from("Бор (B) - микроэлемент"),
            Self::Molybdenum(_) => String::from("Молибден (Mo) - микроэлемент"),
        }
    }
}
