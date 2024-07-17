use crate::model::calculation::{Amount, Calculation};
use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::Profile;

pub struct Solver {
    profile: Profile,
    fertilizers: Vec<Fertilizer>,
    redurant_fertilizers: Vec<Fertilizer>,
}

impl Solver {
    pub fn new(profile: Profile, fertilizers: Vec<Fertilizer>) -> Self {
        Self {
            profile,
            fertilizers,
            redurant_fertilizers: Vec::new(),
        }
    }

    pub fn solve(&mut self) -> Vec<Amount> {
        if self.fertilizers.len() == 0 {
            return Vec::new();
        }

        let mut try_count = 0;

        while try_count < 4 {
            let calculation = Calculation::new()
                .with_fertilizers(self.fertilizers.clone())
                .with_profile(self.profile.clone());

            match calculation.solve() {
                Ok(mut amounts) => {
                    let latest_rotten_index: Option<usize> = amounts
                        .iter()
                        .cloned()
                        .filter(|amount| amount.amount().abs() > 100000.0)
                        .map(|amount| amount.fertilizer_index())
                        .max();

                    match latest_rotten_index {
                        Some(fertilizer_index) => {
                            self.exclude_fertilizer(fertilizer_index);
                        }

                        None => {
                            self.redurant_fertilizers.iter().for_each(|fertilizer| {
                                amounts.push(Amount::new(fertilizer.clone(), 1, 0.0));
                            });

                            return amounts;
                        }
                    }
                }

                Err(error) => {
                    println!("error: {:#?}", error);

                    if self.fertilizers.len() > 0 {
                        let last_index = self.fertilizers.len() - 1;

                        self.exclude_fertilizer(last_index);
                    }

                    // return Err(error);
                }
            }

            try_count += 1;
        }

        Vec::new()
    }

    fn exclude_fertilizer(&mut self, fertilizer_index: usize) {
        if let Some(fertilizer) = self.fertilizers.get(fertilizer_index) {
            self.redurant_fertilizers.push(fertilizer.clone());

            self.fertilizers = self
                .fertilizers
                .iter()
                .enumerate()
                .filter(|(index, _)| *index != fertilizer_index)
                .map(|(_, fertilizer)| fertilizer.clone())
                .collect();
        }
    }
}

#[cfg(test)]
mod tests {}
