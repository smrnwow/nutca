use super::Error;
use crate::model::calculation::Calculation;
use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::Profile;
use crate::model::solutions::Solution;

pub struct Solver {
    profile: Profile,
    fertilizers: Vec<Fertilizer>,
    redurant_fertilizers: Vec<Fertilizer>,
}

impl Solver {
    pub fn new(profile: Profile, mut fertilizers: Vec<Fertilizer>) -> Self {
        fertilizers.sort_by(|a, b| {
            let a_count = a.nutrients().len();

            let b_count = b.nutrients().len();

            a_count.partial_cmp(&b_count).unwrap()
        });

        Self {
            profile,
            fertilizers,
            redurant_fertilizers: Vec::new(),
        }
    }

    pub fn solve(&mut self) -> Result<Solution, Error> {
        println!("calculation");

        let mut try_count = 0;

        while try_count < 4 {
            if try_count > 0 {
                println!("calculation try {}", try_count);
            }

            let calculation =
                Calculation::new(self.profile.clone(), self.fertilizers.clone()).unwrap();

            if let Ok(mut solution) = calculation.solve() {
                match self.has_negative_fertilizer(&solution) {
                    Some(fertilizer) => {
                        println!("found negative fertilizer {}", fertilizer.name());
                        self.exclude_fertilizer(fertilizer);
                    }

                    None => {
                        self.redurant_fertilizers.iter().for_each(|fertilizer| {
                            solution.add_fertilizer_weight(fertilizer.clone(), 0.)
                        });

                        return Ok(solution);
                    }
                }
            } else {
                if let Some(fertilizer) = self.fertilizers.last() {
                    if fertilizer.is_complex() {
                        self.exclude_fertilizer(fertilizer.clone());
                    }
                }
            }

            try_count += 1;
        }

        Ok(Solution::empty(self.fertilizers.clone()))
    }

    fn exclude_fertilizer(&mut self, fertilizer: Fertilizer) {
        self.redurant_fertilizers.push(fertilizer.clone());

        self.fertilizers = self
            .fertilizers
            .iter()
            .filter(|f| f.id() != fertilizer.id())
            .map(|fertilizer| fertilizer.clone())
            .collect();
    }

    fn has_negative_fertilizer(&self, solution: &Solution) -> Option<Fertilizer> {
        let mut solution_fertilizers = solution.fertilizers();

        if let Some(fertilizer) = solution_fertilizers.last() {
            if fertilizer.weight < 0. {
                return Some(fertilizer.fertilizer.clone());
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {}
