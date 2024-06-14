use super::Error;
use crate::model::calculation::Calculation;
use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::Profile;
use crate::model::solutions::Solution;

pub struct Solver {
    profile: Profile,
    fertilizers: Vec<Fertilizer>,
    redurant_complex_fertilizers: Vec<Fertilizer>,
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
            redurant_complex_fertilizers: Vec::new(),
        }
    }

    pub fn solve(&mut self) -> Result<Solution, Error> {
        let mut try_count = 0;

        while try_count < 4 {
            let calculation = Calculation::new(self.profile.clone(), self.fertilizers.clone());

            if let Ok(mut solution) = calculation.unwrap().solve() {
                let zero_complex_fetilizer = self.has_zero_complex_fertilizer(&solution);

                match zero_complex_fetilizer {
                    Some(fertilizer) => {
                        self.exclude_zero_complex_fertilizer(fertilizer);
                    }

                    None => {
                        self.redurant_complex_fertilizers
                            .iter()
                            .for_each(|fertilizer| {
                                solution.add_fertilizer_weight(fertilizer.clone(), 0.)
                            });

                        return Ok(solution);
                    }
                }
            } else {
                if !self.exclude_redurant_fertilizers() {
                    break;
                }
            }

            try_count += 1;
        }

        Ok(Solution::empty(self.fertilizers.clone()))
    }

    fn exclude_redurant_fertilizers(&mut self) -> bool {
        if let Some(fertilizer) = self.fertilizers.last() {
            if fertilizer.is_complex() {
                self.redurant_complex_fertilizers.push(fertilizer.clone());

                self.fertilizers = self
                    .fertilizers
                    .iter()
                    .filter(|f| f.id() != fertilizer.id())
                    .map(|fertilizer| fertilizer.clone())
                    .collect();

                return true;
            }
        }

        return false;
    }

    fn exclude_zero_complex_fertilizer(&mut self, fertilizer: Fertilizer) {
        self.redurant_complex_fertilizers.push(fertilizer.clone());

        self.fertilizers = self
            .fertilizers
            .iter()
            .filter(|f| f.id() != fertilizer.id())
            .map(|fertilizer| fertilizer.clone())
            .collect();
    }

    fn has_zero_complex_fertilizer(&self, solution: &Solution) -> Option<Fertilizer> {
        let solution_fertilizers = solution.fertilizers();

        let zero_complex_fertilizer = solution_fertilizers.iter().find(|fertilizer_weight| {
            fertilizer_weight.fertilizer.is_complex() && fertilizer_weight.weight == 0.
        });

        match zero_complex_fertilizer {
            Some(zero_complex_fertilizer) => Some(zero_complex_fertilizer.fertilizer.clone()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {}
