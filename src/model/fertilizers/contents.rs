use super::NutrientContent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Contents {
    pub nitrogen: f64,
    pub nitrogen_nitrate: f64,
    pub nitrogen_ammonium: f64,
    pub phosphor: f64,
    pub potassium: f64,
    pub calcium: f64,
    pub magnesium: f64,
    pub sulfur: f64,
    pub iron: f64,
    pub manganese: f64,
    pub copper: f64,
    pub zinc: f64,
    pub boron: f64,
    pub molybdenum: f64,
}

impl Contents {
    pub fn new() -> Self {
        Self {
            nitrogen: 0.0,
            nitrogen_nitrate: 0.0,
            nitrogen_ammonium: 0.0,
            phosphor: 0.0,
            potassium: 0.0,
            calcium: 0.0,
            magnesium: 0.0,
            sulfur: 0.0,
            iron: 0.0,
            manganese: 0.0,
            copper: 0.0,
            zinc: 0.0,
            boron: 0.0,
            molybdenum: 0.0,
        }
    }

    pub fn add_nutrient_content(&mut self, nutrient_content: NutrientContent) {
        match nutrient_content {
            NutrientContent::Nitrogen(value) => self.nitrogen += value,
            NutrientContent::NitrogenNitrate(value) => self.nitrogen_nitrate += value,
            NutrientContent::NitrogenAmmonium(value) => self.nitrogen_ammonium += value,
            NutrientContent::Phosphor(value) => self.phosphor += value,
            NutrientContent::Potassium(value) => self.potassium += value,
            NutrientContent::Calcium(value) => self.calcium += value,
            NutrientContent::Magnesium(value) => self.magnesium += value,
            NutrientContent::Sulfur(value) => self.sulfur += value,
            NutrientContent::Iron(value) => self.iron += value,
            NutrientContent::Manganese(value) => self.manganese += value,
            NutrientContent::Copper(value) => self.copper += value,
            NutrientContent::Zinc(value) => self.zinc += value,
            NutrientContent::Boron(value) => self.boron += value,
            NutrientContent::Molybdenum(value) => self.molybdenum += value,
        }
    }

    pub fn nutrients(&self) -> Vec<NutrientContent> {
        let mut nutrients = vec![];

        if self.nitrogen > 0.0 {
            nutrients.push(self.nitrogen());
        };

        if self.nitrogen_nitrate > 0.0 {
            nutrients.push(self.nitrogen_nitrate());
        }

        if self.nitrogen_ammonium > 0.0 {
            nutrients.push(self.nitrogen_ammonium());
        }
        if self.phosphor > 0.0 {
            nutrients.push(self.phosphor());
        }

        if self.potassium > 0.0 {
            nutrients.push(self.potassium());
        }

        if self.calcium > 0.0 {
            nutrients.push(self.calcium());
        }

        if self.magnesium > 0.0 {
            nutrients.push(self.magnesium());
        }

        if self.sulfur > 0.0 {
            nutrients.push(self.sulfur());
        }

        if self.iron > 0.0 {
            nutrients.push(self.iron());
        }

        if self.manganese > 0.0 {
            nutrients.push(self.manganese());
        }

        if self.copper > 0.0 {
            nutrients.push(self.copper());
        }

        if self.zinc > 0.0 {
            nutrients.push(self.zinc());
        }

        if self.boron > 0.0 {
            nutrients.push(self.boron());
        }

        if self.molybdenum > 0.0 {
            nutrients.push(self.molybdenum());
        }

        nutrients
    }

    pub fn nitrogen(&self) -> NutrientContent {
        NutrientContent::Nitrogen(self.nitrogen)
    }

    pub fn nitrogen_nitrate(&self) -> NutrientContent {
        NutrientContent::NitrogenNitrate(self.nitrogen_nitrate)
    }

    pub fn nitrogen_ammonium(&self) -> NutrientContent {
        NutrientContent::NitrogenAmmonium(self.nitrogen_ammonium)
    }

    pub fn phosphor(&self) -> NutrientContent {
        NutrientContent::Phosphor(self.phosphor)
    }

    pub fn potassium(&self) -> NutrientContent {
        NutrientContent::Potassium(self.potassium)
    }

    pub fn calcium(&self) -> NutrientContent {
        NutrientContent::Calcium(self.calcium)
    }

    pub fn magnesium(&self) -> NutrientContent {
        NutrientContent::Magnesium(self.magnesium)
    }

    pub fn sulfur(&self) -> NutrientContent {
        NutrientContent::Sulfur(self.sulfur)
    }

    pub fn iron(&self) -> NutrientContent {
        NutrientContent::Iron(self.iron)
    }

    pub fn manganese(&self) -> NutrientContent {
        NutrientContent::Manganese(self.manganese)
    }

    pub fn copper(&self) -> NutrientContent {
        NutrientContent::Copper(self.copper)
    }

    pub fn zinc(&self) -> NutrientContent {
        NutrientContent::Zinc(self.zinc)
    }

    pub fn boron(&self) -> NutrientContent {
        NutrientContent::Boron(self.boron)
    }

    pub fn molybdenum(&self) -> NutrientContent {
        NutrientContent::Molybdenum(self.molybdenum)
    }
}
