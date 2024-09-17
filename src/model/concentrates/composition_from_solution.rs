use super::{DefaultDistribution, Distribution, Part};
use crate::model::fertilizers::FertilizerAmount;
use crate::model::solutions::Solution;

#[derive(Clone, Debug, PartialEq)]
pub struct CompositionFromSolution {
    solution: Solution,
    distribution: Distribution,
    parts: Vec<Part>,
}

impl CompositionFromSolution {
    pub fn restore(solution: Solution, distribution: Distribution, parts: Vec<Part>) -> Self {
        Self {
            solution,
            parts,
            distribution,
        }
    }

    pub fn add_part(&mut self, part: Part) {
        if self.parts.len() < 5 {
            self.parts.push(part);
        }
    }

    pub fn get_part(&mut self, part_id: &String) -> Option<&mut Part> {
        let position = self.parts.iter().position(|part| *part.id() == *part_id);

        match position {
            Some(index) => self.parts.get_mut(index),
            None => None,
        }
    }

    pub fn update_fertilizer_percent(
        &mut self,
        part_id: &String,
        fertilizer_id: &String,
        usage_percent: usize,
    ) {
        self.distribution.add(part_id, fertilizer_id, usage_percent);
    }

    pub fn remove_fertilizer(&mut self, part_id: &String, fertilizer_id: &String) {
        self.distribution.free(part_id, fertilizer_id);
    }

    pub fn remove_part(&mut self, part_id: &String) {
        self.distribution.free_part(part_id);

        if let Some(index) = self.parts.iter().position(|part| *part.id() == *part_id) {
            self.parts.remove(index);
        }
    }

    pub fn parts(&self) -> Vec<&Part> {
        self.parts.iter().collect()
    }

    pub fn fertilizers_by_part(&self, part: &Part) -> Vec<FertilizerAmount> {
        match self.distribution.get_part(part.id()) {
            Some(part_distribution) => {
                let mut fertilizers: Vec<FertilizerAmount> = Vec::new();

                part_distribution.keys().for_each(|fertilizer_id| {
                    if let Some(fertilizer_amount) = self.solution.fertilizer(fertilizer_id) {
                        let mut fertilizer_amount = fertilizer_amount.clone();

                        fertilizer_amount.concentration(part.concentration());

                        fertilizer_amount.volume(part.volume());

                        fertilizers.push(fertilizer_amount);
                    }
                });

                fertilizers
            }

            None => Vec::new(),
        }
    }

    pub fn usage(&self) -> Vec<(String, String, usize)> {
        let mut fertilizers_usage: Vec<(String, String, usize)> = Vec::new();

        self.distribution
            .usage()
            .iter()
            .for_each(|(fertilizer_id, percent)| {
                if let Some(fertilizer_amount) = self.solution.fertilizer(fertilizer_id) {
                    fertilizers_usage.push((
                        fertilizer_amount.fertilizer().id(),
                        fertilizer_amount.fertilizer().name(),
                        *percent,
                    ));
                }
            });

        fertilizers_usage
    }

    pub fn solution(&self) -> &Solution {
        &self.solution
    }

    pub fn distribution(&self) -> &Distribution {
        &self.distribution
    }
}

impl From<Solution> for CompositionFromSolution {
    fn from(solution: Solution) -> Self {
        let (distribution, parts) = DefaultDistribution::distribute(&solution);

        Self {
            solution,
            distribution,
            parts,
        }
    }
}

impl Default for CompositionFromSolution {
    fn default() -> Self {
        Self {
            solution: Solution::default(),
            parts: Vec::new(),
            distribution: Distribution::default(),
        }
    }
}
