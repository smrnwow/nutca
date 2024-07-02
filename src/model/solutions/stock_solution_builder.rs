use crate::model::chemistry::{Nutrient, Volume};
use crate::model::solutions::{FertilizerWeight, Solution};

pub struct StockSolutionBuilder {
    solution: Solution,
    part_a: Vec<FertilizerWeight>,
    part_b: Vec<FertilizerWeight>,
    concentration_factor: usize,
    volume: Volume,
}

impl StockSolutionBuilder {
    pub fn new() -> Self {
        Self {
            solution: Solution::new(),
            part_a: Vec::new(),
            part_b: Vec::new(),
            concentration_factor: 100,
            volume: Volume::default(),
        }
    }

    pub fn from(solution: Solution) -> Self {
        let mut stock_solution = Self::new();

        stock_solution.update_solution(Some(solution));

        stock_solution
    }

    pub fn update_solution(&mut self, solution: Option<Solution>) {
        self.part_a = Vec::new();

        self.part_b = Vec::new();

        let solution = match solution {
            Some(solution) => solution,
            None => Solution::new(),
        };

        solution.fertilizers().iter().for_each(|fertilizer_weight| {
            let mut has_calcium = false;

            let mut has_sulfur_or_phosphorus = false;

            let mut micros_count = 0;

            for nutrient in fertilizer_weight.fertilizer.nutrients() {
                match nutrient {
                    Nutrient::Calcium(_) => {
                        has_calcium = true;
                    }

                    Nutrient::Sulfur(_) | Nutrient::Phosphorus(_) => {
                        has_sulfur_or_phosphorus = true;
                    }

                    Nutrient::Iron(_)
                    | Nutrient::Manganese(_)
                    | Nutrient::Copper(_)
                    | Nutrient::Zinc(_)
                    | Nutrient::Boron(_)
                    | Nutrient::Molybdenum(_) => {
                        micros_count += 1;
                    }

                    _ => {}
                }
            }

            if has_calcium && has_sulfur_or_phosphorus {
                println!("fertilizer contains both calcium and sulfur/phosphorus");
            }

            if has_calcium || (!has_sulfur_or_phosphorus && micros_count < 3) {
                self.part_a.push(fertilizer_weight.clone());
            }

            if has_sulfur_or_phosphorus || micros_count > 3 {
                self.part_b.push(fertilizer_weight.clone());
            }
        });

        self.solution = solution;
    }

    pub fn update_concentration_factor(&mut self, concentration_factor: usize) {
        self.concentration_factor = concentration_factor;
    }

    pub fn update_volume(&mut self, volume: Volume) {
        self.volume = volume;
    }

    pub fn solution(&self) -> &Solution {
        &self.solution
    }

    pub fn part_a(&self) -> Vec<FertilizerWeight> {
        self.part_a
            .iter()
            .map(|fertilizer_weight| {
                let stock_weight = fertilizer_weight.weight
                    * self.concentration_factor as f64
                    * self.volume.to_litres();

                FertilizerWeight::new(fertilizer_weight.fertilizer.clone(), stock_weight)
            })
            .collect()
    }

    pub fn part_b(&self) -> Vec<FertilizerWeight> {
        self.part_b
            .iter()
            .map(|fertilizer_weight| {
                let stock_weight = fertilizer_weight.weight
                    * self.concentration_factor as f64
                    * self.volume.to_litres();

                FertilizerWeight::new(fertilizer_weight.fertilizer.clone(), stock_weight)
            })
            .collect()
    }

    pub fn concentration_factor(&self) -> usize {
        self.concentration_factor
    }

    pub fn volume(&self) -> Volume {
        self.volume
    }
}
