use crate::model::solutions::Solution;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Distribution {
    distribution: HashMap<String, HashMap<String, usize>>,
    usage: HashMap<String, usize>,
}

impl Distribution {
    pub fn init(solution: &Solution) -> Self {
        let mut usage: HashMap<String, usize> = HashMap::new();

        solution.fertilizers().keys().for_each(|fertilizer_id| {
            usage.insert(fertilizer_id.clone(), 100);
        });

        Self {
            distribution: HashMap::new(),
            usage,
        }
    }

    pub fn restore(
        solution: &Solution,
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
            distribution,
            usage,
        }
    }

    pub fn add(&mut self, part_id: &String, fertilizer_id: &String, usage_percent: usize) {
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

    pub fn free(&mut self, part_id: &String, fertilizer_id: &String) {
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

    pub fn free_part(&mut self, part_id: &String) {
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

    pub fn get_part(&self, part_id: &String) -> Option<&HashMap<String, usize>> {
        self.distribution.get(part_id)
    }

    pub fn usage(&self) -> &HashMap<String, usize> {
        &self.usage
    }

    pub fn distribution(&self) -> &HashMap<String, HashMap<String, usize>> {
        &self.distribution
    }
}

impl Default for Distribution {
    fn default() -> Self {
        Self {
            distribution: HashMap::new(),
            usage: HashMap::new(),
        }
    }
}
