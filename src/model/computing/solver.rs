use super::avoid_nullish_nutrient_requirement_policy::AvoidNullishNutrientRequirementPolicy;
use super::avoid_same_nutrients_sources_policy::AvoidSameNutrientsSourcesPolicy;
use super::{Calculation, CalculationResult};
use crate::model::chemistry::{Nutrient, Nutrients};
use crate::model::fertilizers::Fertilizer;
use std::collections::HashMap;

pub struct Solver<'a> {
    profile: &'a Nutrients,
    fertilizers: HashMap<String, &'a Nutrients>,
    excluded_fertilizers: HashMap<String, CalculationResult>,
}

impl<'a> Solver<'a> {
    pub fn new(profile: &'a Nutrients, mut fertilizers: Vec<&'a Fertilizer>) -> Self {
        let mut solver = Self {
            profile,
            fertilizers: HashMap::new(),
            excluded_fertilizers: HashMap::new(),
        };

        fertilizers.sort_by(|a, b| {
            a.nutrients()
                .total_amount()
                .partial_cmp(&b.nutrients().total_amount())
                .unwrap()
        });

        let nullish_nutrient_policy = AvoidNullishNutrientRequirementPolicy::new(profile);

        let mut same_nutrients_source_policy = AvoidSameNutrientsSourcesPolicy::new();

        fertilizers.iter().for_each(|fertilizer| {
            if !same_nutrients_source_policy.check(fertilizer.nutrients()) {
                solver
                    .excluded_fertilizers
                    .insert(fertilizer.id(), CalculationResult::DuplicatedNutrientSource);

                return;
            }

            if !nullish_nutrient_policy.check(fertilizer.nutrients()) {
                solver.excluded_fertilizers.insert(
                    fertilizer.id(),
                    CalculationResult::NullishNutrientRequirement,
                );

                return;
            }

            solver
                .fertilizers
                .insert(fertilizer.id(), fertilizer.nutrients());
        });

        solver
    }

    pub fn solve(&mut self) -> HashMap<String, CalculationResult> {
        loop {
            let calculation = Calculation::new(&self.profile, self.fertilizers());

            match calculation.calculate() {
                Ok(mut calculation_results) => match self.test_negative(&calculation_results) {
                    Some(fertilizer_id) => {
                        self.exclude_fertilizer(fertilizer_id, CalculationResult::NegativeAmount);
                        continue;
                    }

                    None => {
                        self.excluded_fertilizers.iter().for_each(
                            |(fertilizer_id, calculation_result)| {
                                calculation_results
                                    .insert(fertilizer_id.clone(), *calculation_result);
                            },
                        );

                        return calculation_results;
                    }
                },

                Err(_) => {
                    let fertilizers = self.fertilizers();

                    let last_index = fertilizers.len() - 1;

                    match fertilizers.get(last_index) {
                        Some(fertilizer) => {
                            self.exclude_fertilizer(
                                fertilizer.id(),
                                CalculationResult::RedurantFertilizer,
                            );
                        }

                        None => {
                            let mut calculation_results: HashMap<String, CalculationResult> =
                                HashMap::new();

                            fertilizers.into_iter().for_each(|(fertilizer_id, nutrients)| {
                                calculation_results
                                    .insert(fertilizer_id.clone(), CalculationResult::RedurantFertilizer);
                            });

                            self.excluded_fertilizers.iter().for_each(
                                |(fertilizer_id, calculation_result)| {
                                    calculation_results
                                        .insert(fertilizer_id.clone(), *calculation_result);
                                },
                            );

                            return calculation_results;
                        }
                    }
                }
            }
        }
    }

    fn exclude_fertilizer(&mut self, fertilizer_id: String, calculation_result: CalculationResult) {
        self.excluded_fertilizers
            .entry(fertilizer_id)
            .or_insert(calculation_result);
    }

    fn test_negative(
        &self,
        calculation_results: &HashMap<String, CalculationResult>,
    ) -> Option<String> {
        let mut negatives: Vec<(&String, &CalculationResult)> = calculation_results
            .iter()
            .filter(|(_, calculation_result)| calculation_result.amount().is_sign_negative())
            .collect();

        if negatives.len() == 0 {
            return None;
        }

        negatives.sort_by(|a, b| a.1.amount().partial_cmp(&b.1.amount()).unwrap());

        match negatives.get(0) {
            Some(negative) => Some(negative.0.clone()),
            None => None,
        }
    }

    fn find_fertilizer(&self, fertilizer_id: &String) -> Option<&Nutrients> {
        match self.fertilizers.get(fertilizer_id) {
            Some(nutrients) => Some(*nutrients),
            None => None,
        }
    }

    fn fertilizers(&self) -> Vec<(&String, &Nutrients)> {
        self.fertilizers
            .iter()
            .filter(|(fertilizer_id, _)| !self.excluded_fertilizers.contains_key(*fertilizer_id))
            .map(|(fertilizer_id, nutrients)| (fertilizer_id, *nutrients))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    /*
        use super::Solver;
        use crate::model::chemistry::NutrientAmount;
        use crate::model::fertilizers::{FertilizerBuilder, LabelComponent};

        #[test]
        fn duplicated_nutrients_source_excluded() {
            let profile = ProfileBuilder::new()
                .nutrient_requirement(NutrientAmount::Nitrogen(10.0))
                .nutrient_requirement(NutrientAmount::NitrogenNitrate(10.0))
                .nutrient_requirement(NutrientAmount::Potassium(20.0))
                .build();

            let fertilizer_1 = FertilizerBuilder::new()
                .label_component(LabelComponent::Nitrogen(10.0))
                .label_component(LabelComponent::Potassium(20.0))
                .build();

            let fertilizer_2 = FertilizerBuilder::new()
                .label_component(LabelComponent::Nitrogen(20.0))
                .label_component(LabelComponent::Potassium(40.0))
                .build();

            let fertilizer_3 = FertilizerBuilder::new()
                .label_component(LabelComponent::Nitrogen(15.0))
                .label_component(LabelComponent::Potassium(30.0))
                .build();

            let mut solver = Solver::new(&profile, vec![&fertilizer_1, &fertilizer_2, &fertilizer_3]);

            let fertilizers_weights = solver.solve();

            assert_eq!(
                0.0,
                fertilizers_weights
                    .get(&fertilizer_1.id())
                    .unwrap()
                    .amount()
            );

            assert_eq!(
                0.5,
                fertilizers_weights
                    .get(&fertilizer_2.id())
                    .unwrap()
                    .amount()
            );

            assert_eq!(
                0.0,
                fertilizers_weights
                    .get(&fertilizer_3.id())
                    .unwrap()
                    .amount()
            );
        }

        #[test]
        fn nullish_nutrient_requirement_excluded() {
            let profile = ProfileBuilder::new()
                .nutrient_requirement(NutrientAmount::Nitrogen(10.0))
                .nutrient_requirement(NutrientAmount::NitrogenNitrate(10.0))
                .nutrient_requirement(NutrientAmount::Potassium(20.0))
                .build();

            let fertilizer_1 = FertilizerBuilder::new()
                .label_component(LabelComponent::Calcium(10.0))
                .build();

            let fertilizer_2 = FertilizerBuilder::new()
                .label_component(LabelComponent::Phosphorus(20.0))
                .label_component(LabelComponent::Potassium(40.0))
                .build();

            let fertilizer_3 = FertilizerBuilder::new()
                .label_component(LabelComponent::Nitrogen(20.0))
                .label_component(LabelComponent::Potassium(40.0))
                .label_component(LabelComponent::Phosphorus(10.0))
                .label_component(LabelComponent::Calcium(10.0))
                .label_component(LabelComponent::Sulfur(10.0))
                .build();

            let mut solver = Solver::new(&profile, vec![&fertilizer_1, &fertilizer_2, &fertilizer_3]);

            let fertilizers_weights = solver.solve();

            assert_eq!(
                0.0,
                fertilizers_weights
                    .get(&fertilizer_1.id())
                    .unwrap()
                    .amount()
            );

            assert_eq!(
                0.0,
                fertilizers_weights
                    .get(&fertilizer_2.id())
                    .unwrap()
                    .amount()
            );

            assert_eq!(
                0.5,
                fertilizers_weights
                    .get(&fertilizer_3.id())
                    .unwrap()
                    .amount()
            );
    }
    */
}
