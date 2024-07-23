use crate::model::calculation::{Calculation, ExclusionReason};
use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::Profile;
use crate::model::solutions::{FertilizerWeight, FertilizersSet};
use std::collections::HashMap;

pub struct Solver {
    profile: Profile,
    fertilizers: Vec<Fertilizer>,
    sources: HashMap<String, String>,
    excluded_fertilizers: HashMap<String, ExclusionReason>,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            profile: Profile::default(),
            fertilizers: Vec::new(),
            sources: HashMap::new(),
            excluded_fertilizers: HashMap::new(),
        }
    }

    pub fn with_profile(&mut self, profile: Profile) -> &mut Self {
        self.profile = profile;
        self
    }

    pub fn with_fertilizers(&mut self, fertilizers: Vec<Fertilizer>) -> &mut Self {
        for fertilizer in fertilizers {
            self.add_fertilizer(fertilizer);
        }

        self
    }

    pub fn add_fertilizer(&mut self, fertilizer: Fertilizer) -> &mut Self {
        self.test_nutrients_source_duplication(&fertilizer);

        self.test_nullish_nutrient_requirement(&fertilizer);

        self.fertilizers.push(fertilizer);

        self
    }

    pub fn solve(&mut self) -> FertilizersSet {
        let mut try_count = 0;

        while try_count < 4 {
            let calculation = Calculation::new()
                .with_fertilizers(self.fertilizers())
                .with_profile(&self.profile);

            match calculation.solve() {
                Ok(fertilizers_weights) => {
                    // println!("{} try result:", try_count + 1);

                    /*
                    fertilizers_weights.iter().for_each(|fertilizers_weight| {
                        println!(
                            "weight {} = {}",
                            fertilizers_weight.name(),
                            fertilizers_weight.weight(),
                        );
                    });
                    */

                    match self.test_negative(&fertilizers_weights) {
                        Some(fertilizer_id) => {
                            self.exclude_fertilizer(fertilizer_id, ExclusionReason::NegativeAmount);
                            continue;
                        }

                        None => {
                            let mut fertilizers_set = FertilizersSet::new(fertilizers_weights);

                            self.excluded_fertilizers
                                .iter()
                                .for_each(|excluded_fertilizer| {
                                    let fertilizer = self
                                        .fertilizers
                                        .iter()
                                        .find(|f| f.id() == *excluded_fertilizer.0);

                                    if let Some(fertilizer) = fertilizer {
                                        fertilizers_set.add_fertilizer_weight(
                                            FertilizerWeight::new(fertilizer.clone(), 0.0),
                                        );
                                    }
                                });

                            return fertilizers_set;
                        }
                    }
                }

                Err(_) => {
                    // println!("{} try error", try_count + 1);

                    /*
                    if self.fertilizers.len() > 0 {
                        let last_index = self.fertilizers.len() - 1;

                        if let Some(fertilizer) = self.fertilizers.get(last_index) {
                            self.exclude_fertilizer(fertilizer.id());
                        }
                    }
                    */

                    // return Err(error);
                }
            }

            try_count += 1;
        }

        FertilizersSet::from(self.fertilizers.clone())
    }

    fn test_nutrients_source_duplication(&mut self, fertilizer: &Fertilizer) {
        let nutrients_string = fertilizer.nutrients().stringify();

        match self.sources.get(&nutrients_string) {
            Some(same_nutrients_source) => {
                let other_fertilizer = self
                    .fertilizers
                    .iter()
                    .find(|f| f.id() == *same_nutrients_source);

                if let Some(other_fertilizer) = other_fertilizer {
                    let other_has_more_nutrients = other_fertilizer.nutrients().total_nutrients()
                        >= fertilizer.nutrients().total_nutrients();

                    if other_has_more_nutrients {
                        self.exclude_fertilizer(
                            fertilizer.id(),
                            ExclusionReason::DuplicatedNutrientSource,
                        );
                    } else {
                        self.exclude_fertilizer(
                            other_fertilizer.id(),
                            ExclusionReason::DuplicatedNutrientSource,
                        );
                    }
                }
            }
            None => {
                self.sources.insert(nutrients_string, fertilizer.id());
            }
        }
    }

    fn test_nullish_nutrient_requirement(&mut self, fertilizer: &Fertilizer) {
        let nutrients = fertilizer.nutrients().list();

        if nutrients.len() < 4 {
            let mut has_nullish_requirement = false;

            nutrients.iter().for_each(|nutrient_amount| {
                let nutrient_requirement = self
                    .profile
                    .nutrient_requirement(nutrient_amount.nutrient());

                if nutrient_requirement.value() == 0.0 {
                    has_nullish_requirement = true;
                }
            });

            if has_nullish_requirement {
                self.exclude_fertilizer(
                    fertilizer.id(),
                    ExclusionReason::NullishNutrientRequirement,
                );
            }
        }
    }

    fn exclude_fertilizer(&mut self, fertilizer_id: String, reason: ExclusionReason) {
        if let None = self.excluded_fertilizers.get(&fertilizer_id) {
            self.excluded_fertilizers.insert(fertilizer_id, reason);
        }
    }

    fn test_negative(&self, fertilizers_weights: &Vec<FertilizerWeight>) -> Option<String> {
        let mut negatives: Vec<&FertilizerWeight> = fertilizers_weights
            .iter()
            .filter(|fertilizers_weight| fertilizers_weight.weight().is_sign_negative())
            .collect();

        if negatives.len() == 0 {
            return None;
        }

        negatives.sort_by(|a, b| a.weight().partial_cmp(&b.weight()).unwrap());

        /*
        negatives.iter().for_each(|amount| {
            println!(
                "negative {} = {}",
                amount.fertilizer().name(),
                amount.amount()
            );
        });
        */

        match negatives.get(0) {
            Some(negative) => Some(negative.id()),
            None => None,
        }
    }

    fn fertilizers(&self) -> Vec<Fertilizer> {
        self.fertilizers
            .iter()
            .filter(|fertilizer| !self.excluded_fertilizers.contains_key(&fertilizer.id()))
            .map(|fertilizer| fertilizer.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {}
