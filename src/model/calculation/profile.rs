use super::NutrientRequirement;

#[derive(Clone, Copy, Debug)]
pub struct Profile {
    nutrients: [NutrientRequirement; 14],
}

impl Profile {
    pub fn new() -> Self {
        Self {
            nutrients: [
                NutrientRequirement::Nitrogen(0.0),
                NutrientRequirement::NitrogenNitrate(0.0),
                NutrientRequirement::NitrogenAmmonium(0.0),
                NutrientRequirement::Phosphor(0.0),
                NutrientRequirement::Potassium(0.0),
                NutrientRequirement::Calcium(0.0),
                NutrientRequirement::Magnesium(0.0),
                NutrientRequirement::Sulfur(0.0),
                NutrientRequirement::Iron(0.0),
                NutrientRequirement::Manganese(0.0),
                NutrientRequirement::Copper(0.0),
                NutrientRequirement::Zinc(0.0),
                NutrientRequirement::Boron(0.0),
                NutrientRequirement::Molybdenum(0.0),
            ],
        }
    }

    pub fn requirements(&self) -> Vec<NutrientRequirement> {
        vec![
            self.nitrogen(),
            self.nitrogen_nitrate(),
            self.nitrogen_ammonium(),
            self.phosphor(),
            self.potassium(),
            self.calcium(),
            self.magnesium(),
            self.sulfur(),
            self.iron(),
            self.manganese(),
            self.copper(),
            self.zinc(),
            self.boron(),
            self.molybdenum(),
        ]
    }

    pub fn set_nutrient(&mut self, nutrient_requirement: NutrientRequirement) {
        match nutrient_requirement {
            NutrientRequirement::Nitrogen(value) => {
                self.nutrients[nutrient_requirement.index()] = nutrient_requirement;

                self.nutrients[self.nitrogen_nitrate().index()] =
                    NutrientRequirement::NitrogenNitrate(value - self.nitrogen_ammonium().amount());
            }

            NutrientRequirement::NitrogenNitrate(value) => {
                self.nutrients[nutrient_requirement.index()] = nutrient_requirement;

                self.nutrients[self.nitrogen().index()] =
                    NutrientRequirement::Nitrogen(value + self.nitrogen_ammonium().amount());
            }

            NutrientRequirement::NitrogenAmmonium(value) => {
                self.nutrients[nutrient_requirement.index()] = nutrient_requirement;

                self.nutrients[self.nitrogen().index()] =
                    NutrientRequirement::Nitrogen(value + self.nitrogen_nitrate().amount());
            }

            _ => {
                self.nutrients[nutrient_requirement.index()] = nutrient_requirement;
            }
        }
    }

    pub fn update_nutrient(&mut self, nutrient_requirement: NutrientRequirement) {
        let nutrient = self.nutrients[nutrient_requirement.index()];

        self.nutrients[nutrient_requirement.index()] = nutrient.add(nutrient_requirement.amount());
    }

    pub fn nitrogen(&self) -> NutrientRequirement {
        self.nutrients[0]
        // NutrientRequirement::Nitrogen(self.nitrogen)
    }

    pub fn nitrogen_nitrate(&self) -> NutrientRequirement {
        self.nutrients[1]
        // NutrientRequirement::NitrogenNitrate(self.nitrogen_nitrate)
    }

    pub fn nitrogen_ammonium(&self) -> NutrientRequirement {
        self.nutrients[2]
        // NutrientRequirement::NitrogenAmmonium(self.nitrogen_ammonium)
    }

    pub fn phosphor(&self) -> NutrientRequirement {
        self.nutrients[3]
        // NutrientRequirement::Phosphor(self.phosphor)
    }

    pub fn potassium(&self) -> NutrientRequirement {
        self.nutrients[4]
        // NutrientRequirement::Potassium(self.potassium)
    }

    pub fn calcium(&self) -> NutrientRequirement {
        self.nutrients[5]
        // NutrientRequirement::Calcium(self.calcium)
    }

    pub fn magnesium(&self) -> NutrientRequirement {
        self.nutrients[6]
        // NutrientRequirement::Magnesium(self.magnesium)
    }

    pub fn sulfur(&self) -> NutrientRequirement {
        self.nutrients[7]
        // NutrientRequirement::Sulfur(self.sulfur)
    }

    pub fn iron(&self) -> NutrientRequirement {
        self.nutrients[8]
        // NutrientRequirement::Iron(self.iron)
    }

    pub fn manganese(&self) -> NutrientRequirement {
        self.nutrients[9]
        // NutrientRequirement::Manganese(self.manganese)
    }

    pub fn copper(&self) -> NutrientRequirement {
        self.nutrients[10]
        // NutrientRequirement::Copper(self.copper)
    }

    pub fn zinc(&self) -> NutrientRequirement {
        self.nutrients[11]
        // NutrientRequirement::Zinc(self.zinc)
    }

    pub fn boron(&self) -> NutrientRequirement {
        self.nutrients[12]
        // NutrientRequirement::Boron(self.boron)
    }

    pub fn molybdenum(&self) -> NutrientRequirement {
        self.nutrients[13]
        // NutrientRequirement::Molybdenum(self.molybdenum)
    }
}
