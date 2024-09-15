use super::NutrientResult;
use crate::model::chemistry::{Nutrient, NutrientAmount, Nutrients};
use crate::model::fertilizers::FertilizerAmount;
use crate::model::profiles::Profile;

#[derive(Clone, Debug, PartialEq)]
pub struct NutrientComposition {
    nutrition_program: Profile,
    nutrients: Nutrients,
}

impl NutrientComposition {
    pub fn new(nutrition_program: Profile, nutrients: Nutrients) -> Self {
        Self {
            nutrition_program,
            nutrients,
        }
    }

    pub fn nutrients(&self) -> &Nutrients {
        &self.nutrients
    }

    pub fn nutrition_program(&self) -> &Profile {
        &self.nutrition_program
    }

    pub fn nutrient_value(&self, nutrient: Nutrient) -> NutrientAmount {
        self.nutrients.value_of(nutrient)
    }

    pub fn nutrient_diff(&self, nutrient: Nutrient) -> NutrientResult {
        let nutrient_value = self.nutrients.value_of(nutrient).value();

        NutrientResult::new(
            self.nutrition_program
                .nutrient_requirement(nutrient)
                .value(),
            format!("{:.3}", nutrient_value).parse().unwrap(),
        )
    }

    pub fn with_nutrient_requirement(&mut self, nutrient_requirement: NutrientAmount) {
        if self.nutrition_program.is_saved() {
            self.nutrition_program = Profile::extend(&self.nutrition_program);
        }

        self.nutrition_program
            .update_nutrient_requirement(nutrient_requirement);
    }

    pub fn with_nutrition_program(&mut self, nutrition_program: Option<Profile>) {
        match nutrition_program {
            Some(nutrition_program) => self.nutrition_program = nutrition_program,
            None => self.nutrition_program = Profile::default(),
        }
    }

    pub fn with_fertilizers_amounts(&mut self, fertilizers_amounts: Vec<&FertilizerAmount>) {
        let mut nutrients = Nutrients::new();

        fertilizers_amounts.iter().for_each(|fertilizer_amount| {
            fertilizer_amount
                .nutrients()
                .list()
                .iter()
                .for_each(|nutrient_amount| {
                    nutrients.add(nutrient_amount.new(nutrient_amount.value() * 10.));
                });
        });

        self.nutrients = nutrients;
    }
}

impl Default for NutrientComposition {
    fn default() -> Self {
        Self::new(Profile::default(), Nutrients::new())
    }
}
