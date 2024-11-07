use crate::model::chemistry::{Nutrient, Nutrients};
use crate::model::solutions::fertilizers::FertilizerAmount;
use std::collections::HashMap;

#[derive(Debug)]
pub struct NutrientsContent {
    fertilizers: HashMap<String, Nutrients>,
}

impl NutrientsContent {
    pub fn new() -> Self {
        Self {
            fertilizers: HashMap::new(),
        }
    }

    pub fn nutrients(&self) -> Nutrients {
        self.fertilizers
            .values()
            .into_iter()
            .fold(Nutrients::new(), |nutrients_content, nutrients| {
                nutrients_content.plus(*nutrients)
            })
    }

    pub fn plus(&mut self, fertilizer_amount: FertilizerAmount) {
        self.fertilizers.insert(
            fertilizer_amount.id().to_string(),
            fertilizer_amount.nutrients(),
        );
    }

    pub fn minus(&mut self, fertilizer_amount: FertilizerAmount) {
        self.fertilizers.remove(fertilizer_amount.id());
    }
}

#[cfg(test)]
mod tests {
    use super::NutrientsContent;
    use crate::model::chemistry::{Nutrient, NutrientAmount};
    use crate::model::solutions::fertilizers::{Fertilizer, FertilizerAmount};

    #[test]
    fn account_added_fertilizers_nutrients() {
        let mut nutrients_content = NutrientsContent::new();

        let fertilizer_1 = Fertilizer::from(vec![
            NutrientAmount::NitrogenNitrate(20.0),
            NutrientAmount::Phosphorus(10.0),
            NutrientAmount::Potassium(30.0),
        ]);

        let fertilizer_2 = Fertilizer::from(vec![
            NutrientAmount::Potassium(15.0),
            NutrientAmount::Calcium(20.0),
        ]);

        nutrients_content.plus(FertilizerAmount::new(&fertilizer_1, 1.0));

        nutrients_content.plus(FertilizerAmount::new(&fertilizer_2, 1.0));

        let nutrients = nutrients_content.nutrients();

        assert_eq!(20.0, nutrients.value_of(Nutrient::NitrogenNitrate).value());

        assert_eq!(10.0, nutrients.value_of(Nutrient::Phosphorus).value());

        assert_eq!(45.0, nutrients.value_of(Nutrient::Potassium).value());

        assert_eq!(20.0, nutrients.value_of(Nutrient::Calcium).value());
    }

    #[test]
    fn remove_nutrients_of_removed_fertilizer() {
        let mut nutrients_content = NutrientsContent::new();

        let fertilizer_1 = Fertilizer::from(vec![
            NutrientAmount::NitrogenNitrate(20.0),
            NutrientAmount::Phosphorus(10.0),
            NutrientAmount::Potassium(30.0),
        ]);

        let fertilizer_2 = Fertilizer::from(vec![
            NutrientAmount::Potassium(15.0),
            NutrientAmount::Calcium(20.0),
        ]);

        let fertilizer_amount = FertilizerAmount::new(&fertilizer_1, 1.0);

        nutrients_content.plus(fertilizer_amount.clone());

        nutrients_content.plus(FertilizerAmount::new(&fertilizer_2, 1.0));

        nutrients_content.minus(fertilizer_amount.clone());

        let nutrients = nutrients_content.nutrients();

        assert_eq!(0.0, nutrients.value_of(Nutrient::NitrogenNitrate).value());

        assert_eq!(0.0, nutrients.value_of(Nutrient::Phosphorus).value());

        assert_eq!(15.0, nutrients.value_of(Nutrient::Potassium).value());

        assert_eq!(20.0, nutrients.value_of(Nutrient::Calcium).value());
    }
}
