use crate::model::calculation::{Amount, Calculation};
use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::Profile;

pub struct Solver<'a> {
    profile: &'a Profile,
    fertilizers: Vec<Fertilizer>,
    redurant_fertilizers: Vec<Fertilizer>,
}

impl<'a> Solver<'a> {
    pub fn new(profile: &'a Profile, fertilizers: Vec<Fertilizer>) -> Self {
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
                .with_profile(self.profile);

            match calculation.solve() {
                Ok(mut amounts) => {
                    // println!("{} try result:", try_count + 1);

                    amounts.iter().for_each(|amount| {
                        println!("{} = {}", amount.fertilizer().name(), amount.amount());
                    });

                    if let Some(rotten_index) = self.test_rotten(&amounts) {
                        self.exclude_fertilizer(rotten_index);
                        continue;
                    }

                    if let Some(negative_index) = self.test_negative(&amounts) {
                        self.exclude_fertilizer(negative_index);
                        continue;
                    }

                    self.redurant_fertilizers.iter().for_each(|fertilizer| {
                        amounts.push(Amount::new(fertilizer.clone(), 1, 0.0));
                    });

                    return amounts;
                }

                Err(error) => {
                    // println!("{} try error: {:#?}", try_count + 1, error);

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

    fn test_rotten(&self, amounts: &Vec<Amount>) -> Option<usize> {
        amounts
            .iter()
            .filter(|amount| amount.amount().abs() > 100000.0)
            .map(|amount| amount.fertilizer_index())
            .max()
    }

    fn test_negative(&self, amounts: &Vec<Amount>) -> Option<usize> {
        let mut negatives: Vec<(usize, f64)> = amounts
            .iter()
            .filter(|amount| amount.amount().is_sign_negative())
            .map(|amount| (amount.fertilizer_index(), amount.amount()))
            .collect();

        if negatives.len() == 0 {
            return None;
        }

        negatives.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        println!("negatives {:#?}", negatives);

        match negatives.get(0) {
            Some(negative) => Some(negative.0),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {}
