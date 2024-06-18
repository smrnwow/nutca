use crate::model::chemistry::Nutrient;
use crate::model::solutions::{FertilizerWeight, Solution};

pub struct StockSolutionBuilder {
    solution: Solution,
    part_a: Vec<FertilizerWeight>,
    part_b: Vec<FertilizerWeight>,
    concentration_factor: usize,
    volume: usize,
}

impl StockSolutionBuilder {
    pub fn new() -> Self {
        Self {
            solution: Solution::new(),
            part_a: Vec::new(),
            part_b: Vec::new(),
            concentration_factor: 100,
            volume: 100,
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
            let mut is_b_part = false;

            for nutrient in fertilizer_weight.fertilizer.nutrients() {
                match nutrient {
                    Nutrient::Sulfur(_) | Nutrient::Phosphorus(_) => {
                        is_b_part = true;

                        break;
                    }

                    _ => {}
                }
            }

            if is_b_part {
                self.part_b.push(fertilizer_weight.clone());
            } else {
                self.part_a.push(fertilizer_weight.clone());
            }
        });

        self.solution = solution;
    }

    pub fn update_concentration_factor(&mut self, concentration_factor: usize) {
        self.concentration_factor = concentration_factor;
    }

    pub fn update_volume(&mut self, volume: usize) {
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
                    * self.volume as f64;

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
                    * self.volume as f64;

                FertilizerWeight::new(fertilizer_weight.fertilizer.clone(), stock_weight)
            })
            .collect()
    }

    pub fn concentration_factor(&self) -> usize {
        self.concentration_factor
    }

    pub fn volume(&self) -> usize {
        self.volume
    }
}
