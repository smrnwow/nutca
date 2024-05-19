use crate::chemistry::Nutrient;
use crate::fertilizers::Fertiliser;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ResultProfile<'a> {
    fertilizers: Vec<&'a Fertiliser>,
    nutrients: HashMap<Nutrient, f64>,
}

impl<'a> ResultProfile<'a> {
    pub fn add_fertilizer(&mut self, fertilizer: &'a Fertiliser, weight: f64) {
        self.fertilizers.push(fertilizer);

        fertilizer.nutrients().iter().for_each(|nutrient| {
            if let Some(nutrient_value) = self.nutrients.get_mut(&nutrient.symbol()) {
                *nutrient_value += nutrient.percent() * weight;
            };
        });
    }
}

impl<'a> From<Vec<(&'a Fertiliser, f64)>> for ResultProfile<'a> {
    fn from(fertilizers_weights: Vec<(&'a Fertiliser, f64)>) -> Self {
        let mut result_profile = Self {
            nutrients: HashMap::from([
                (Nutrient::Nitrogen, 0.0),
                (Nutrient::NitrogenNitrate, 0.0),
                (Nutrient::NitrogenAmmonium, 0.0),
                (Nutrient::Phosphorus, 0.0),
                (Nutrient::Potassium, 0.0),
                (Nutrient::Calcium, 0.0),
                (Nutrient::Magnesium, 0.0),
                (Nutrient::Sulfur, 0.0),
                (Nutrient::Iron, 0.0),
                (Nutrient::Boron, 0.0),
                (Nutrient::Zink, 0.0),
                (Nutrient::Manganese, 0.0),
                (Nutrient::Copper, 0.0),
                (Nutrient::Molybdenum, 0.0),
            ]),
            fertilizers: Vec::new(),
        };

        fertilizers_weights.iter().for_each(|(fertilizer, weight)| {
            result_profile.add_fertilizer(fertilizer, *weight);
        });

        result_profile
    }
}
