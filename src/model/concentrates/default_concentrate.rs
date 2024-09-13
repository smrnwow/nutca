use crate::model::chemistry::Nutrient;
use crate::model::concentrates::parts::{AutoPart, FertilizerPercent};
use crate::model::solutions::FertilizerWeight;

pub struct DefaultConcentrate {
    part_a: AutoPart,
    part_b: AutoPart,
}

impl DefaultConcentrate {
    pub fn new(fertilizers: Vec<&FertilizerWeight>) -> Self {
        let mut concentrate = Self {
            part_a: AutoPart::new("A"),
            part_b: AutoPart::new("B"),
        };

        concentrate.calculate(fertilizers);

        concentrate
    }

    pub fn parts(&self) -> Vec<&AutoPart> {
        vec![&self.part_a, &self.part_b]
    }

    fn calculate(&mut self, fertilizers: Vec<&FertilizerWeight>) {
        fertilizers.iter().for_each(|fertilizer_weight| {
            let mut has_calcium = false;

            let mut has_sulfur_or_phosphorus = false;

            let mut micros_count = 0;

            for nutrient_amount in fertilizer_weight.nutrients().list() {
                match nutrient_amount.nutrient() {
                    Nutrient::Calcium => {
                        has_calcium = true;
                    }

                    Nutrient::Sulfur | Nutrient::Phosphorus => {
                        has_sulfur_or_phosphorus = true;
                    }

                    Nutrient::Iron
                    | Nutrient::Manganese
                    | Nutrient::Copper
                    | Nutrient::Zinc
                    | Nutrient::Boron
                    | Nutrient::Molybdenum => {
                        micros_count += 1;
                    }

                    _ => {}
                }
            }

            if has_calcium && has_sulfur_or_phosphorus {
                println!("fertilizer contains both calcium and sulfur/phosphorus");
            }

            if has_calcium || (!has_sulfur_or_phosphorus && micros_count < 3) {
                self.part_a
                    .add_fertilizer(FertilizerPercent::new(fertilizer_weight.id(), 100));
            }

            if has_sulfur_or_phosphorus || micros_count > 3 {
                self.part_b
                    .add_fertilizer(FertilizerPercent::new(fertilizer_weight.id(), 100));
            }
        });
    }
}
