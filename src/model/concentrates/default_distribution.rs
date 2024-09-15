use super::{CompositionFromSolution, Part};
use crate::model::chemistry::Nutrient;
use crate::model::solutions::Solution;

pub struct DefaultDistribution {
    solution: Solution,
    part_a: Part,
    part_b: Part,
    composition: CompositionFromSolution,
}

impl DefaultDistribution {
    pub fn new(solution: Solution) -> Self {
        Self {
            solution: solution.clone(),
            part_a: Part::new("A"),
            part_b: Part::new("B"),
            composition: CompositionFromSolution::from(solution),
        }
    }

    pub fn distribute(mut self) -> (CompositionFromSolution, Vec<Part>) {
        self.solution.fertilizers().values().for_each(|fertilizer| {
            let mut has_calcium = false;

            let mut has_sulfur_or_phosphorus = false;

            let mut micros_count = 0;

            for nutrient_amount in fertilizer.nutrients().list() {
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
                self.composition
                    .update_fertilizer_percent(self.part_a.id(), &fertilizer.id(), 100);
            }

            if has_sulfur_or_phosphorus || micros_count > 3 {
                self.composition
                    .update_fertilizer_percent(self.part_b.id(), &fertilizer.id(), 100);
            }
        });

        (self.composition, vec![self.part_a, self.part_b])
    }
}
