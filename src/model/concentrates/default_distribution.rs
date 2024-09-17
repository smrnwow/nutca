use super::{Distribution, Part};
use crate::model::chemistry::Nutrient;
use crate::model::solutions::Solution;

pub struct DefaultDistribution {}

impl DefaultDistribution {
    pub fn distribute(solution: &Solution) -> (Distribution, Vec<Part>) {
        let mut distribution = Distribution::init(solution);

        let part_a = Part::new("A");

        let part_b = Part::new("B");

        solution
            .fertilizers()
            .values()
            .for_each(|fertilizer_amount| {
                let mut has_calcium = false;

                let mut has_sulfur_or_phosphorus = false;

                let mut micros_count = 0;

                for nutrient_amount in fertilizer_amount.nutrients().list() {
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
                    distribution.add(part_a.id(), &fertilizer_amount.fertilizer().id(), 100);
                }

                if has_sulfur_or_phosphorus || micros_count > 3 {
                    distribution.add(part_b.id(), &fertilizer_amount.fertilizer().id(), 100);
                }
            });

        (distribution, Vec::from([part_a, part_b]))
    }
}
