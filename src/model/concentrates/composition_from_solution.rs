use super::Part;
use crate::model::fertilizers::FertilizerAmount;
use crate::model::solutions::Solution;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct CompositionFromSolution {
    solution: Solution,
    distribution: HashMap<String, HashMap<String, usize>>,
    usage: HashMap<String, usize>,
}

impl CompositionFromSolution {
    pub fn restore(
        solution: Solution,
        distribution: HashMap<String, HashMap<String, usize>>,
    ) -> Self {
        let mut usage: HashMap<String, usize> = HashMap::new();

        solution.fertilizers().keys().for_each(|fertilizer_id| {
            usage.insert(fertilizer_id.clone(), 100);
        });

        distribution.values().for_each(|part_distribution| {
            part_distribution
                .iter()
                .for_each(|(fertilizer_id, amount)| {
                    if let Some(fertilizer_usage) = usage.get_mut(fertilizer_id) {
                        *fertilizer_usage -= *amount;
                    }
                });
        });

        Self {
            solution,
            distribution,
            usage,
        }
    }

    pub fn update_fertilizer_percent(
        &mut self,
        part_id: &String,
        fertilizer_id: &String,
        usage_percent: usize,
    ) {
        match self.distribution.get_mut(part_id) {
            Some(part_distribution) => match part_distribution.get_mut(fertilizer_id) {
                Some(fertilizer_usage) => {
                    *fertilizer_usage += usage_percent;
                }

                None => {
                    part_distribution.insert(fertilizer_id.clone(), usage_percent);
                }
            },

            None => {
                let mut part_distribution: HashMap<String, usize> = HashMap::new();

                part_distribution.insert(fertilizer_id.clone(), usage_percent);

                self.distribution.insert(part_id.clone(), part_distribution);
            }
        }

        if let Some(fertilizer_usage) = self.usage.get_mut(fertilizer_id) {
            *fertilizer_usage -= usage_percent;
        }
    }

    pub fn remove_fertilizer(&mut self, part_id: &String, fertilizer_id: &String) {
        if let Some(part_distribution) = self.distribution.get_mut(part_id) {
            let freed_percent = match part_distribution.remove(fertilizer_id) {
                Some(fertilizer_usage) => fertilizer_usage,
                None => 0,
            };

            if let Some(fertilizer_usage) = self.usage.get_mut(fertilizer_id) {
                *fertilizer_usage += freed_percent;
            }
        }
    }

    pub fn remove_part(&mut self, part_id: &String) {
        if let Some(part_distribution) = self.distribution.remove(part_id) {
            part_distribution
                .iter()
                .for_each(|(fertilizer_id, usage_percent)| {
                    if let Some(fertilizer_usage) = self.usage.get_mut(fertilizer_id) {
                        *fertilizer_usage += *usage_percent;
                    }
                });
        }
    }

    pub fn fertilizers_by_part(&self, part: &Part) -> Vec<FertilizerAmount> {
        match self.distribution.get(part.id()) {
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

        self.usage.iter().for_each(|(fertilizer_id, percent)| {
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

    pub fn distribution(&self) -> &HashMap<String, HashMap<String, usize>> {
        &self.distribution
    }
}

impl From<Solution> for CompositionFromSolution {
    fn from(solution: Solution) -> Self {
        let mut usage: HashMap<String, usize> = HashMap::new();

        solution.fertilizers().keys().for_each(|fertilizer_id| {
            usage.insert(fertilizer_id.clone(), 100);
        });

        Self {
            solution,
            distribution: HashMap::new(),
            usage,
        }
    }
}

impl Default for CompositionFromSolution {
    fn default() -> Self {
        Self {
            solution: Solution::default(),
            distribution: HashMap::new(),
            usage: HashMap::new(),
        }
    }
}
