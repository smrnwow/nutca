use super::{Calculation, CalculationResult};
use crate::model::chemistry::Nutrients;
use crate::model::fertilizers::Fertilizer;
use std::collections::HashMap;

pub struct Solver<'a> {
    profile: &'a Nutrients,
    fertilizers: Vec<&'a Fertilizer>,
    sources: HashMap<String, String>,
    excluded_fertilizers: HashMap<String, CalculationResult>,
}

impl<'a> Solver<'a> {
    pub fn new(profile: &'a Nutrients, fertilizers: Vec<&'a Fertilizer>) -> Self {
        let mut solver = Self {
            profile,
            fertilizers: Vec::new(),
            sources: HashMap::new(),
            excluded_fertilizers: HashMap::new(),
        };

        fertilizers
            .iter()
            .for_each(|fertilizer| solver.add_fertilizer(fertilizer));

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

                            fertilizers.iter().for_each(|fertilizer| {
                                calculation_results
                                    .insert(fertilizer.id(), CalculationResult::RedurantFertilizer);
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

    fn add_fertilizer(&mut self, fertilizer: &'a Fertilizer) {
        self.test_nutrients_source_duplication(fertilizer);

        if self.test_nullish_nutrient_requirement(fertilizer) {
            self.exclude_fertilizer(
                fertilizer.id(),
                CalculationResult::NullishNutrientRequirement,
            );
        }

        self.fertilizers.push(fertilizer);
    }

    fn exclude_fertilizer(&mut self, fertilizer_id: String, calculation_result: CalculationResult) {
        self.excluded_fertilizers
            .entry(fertilizer_id)
            .or_insert(calculation_result);
    }

    fn test_nutrients_source_duplication(&mut self, fertilizer: &Fertilizer) {
        let nutrients_string = fertilizer.nutrients().stringify();

        match self.sources.get(&nutrients_string) {
            Some(same_nutrients_source) => {
                if let Some(other_fertilizer) = self.find_fertilizer(same_nutrients_source) {
                    if fertilizer.compare(other_fertilizer) {
                        self.exclude_fertilizer(
                            other_fertilizer.id(),
                            CalculationResult::DuplicatedNutrientSource,
                        );

                        self.sources.insert(nutrients_string, fertilizer.id());
                    } else {
                        self.exclude_fertilizer(
                            fertilizer.id(),
                            CalculationResult::DuplicatedNutrientSource,
                        );
                    }
                }
            }

            None => {
                self.sources.insert(nutrients_string, fertilizer.id());
            }
        }
    }

    fn test_nullish_nutrient_requirement(&self, fertilizer: &Fertilizer) -> bool {
        let nutrients = fertilizer.nutrients().list();

        let mut has_nullish_requirement = false;

        if nutrients.len() < 4 {
            nutrients.iter().for_each(|nutrient_amount| {
                let nutrient_requirement = self.profile.value_of(nutrient_amount.nutrient());

                if nutrient_requirement.value() == 0.0 {
                    has_nullish_requirement = true;
                }
            });
        }

        has_nullish_requirement
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

    fn find_fertilizer(&self, fertilizer_id: &String) -> Option<&Fertilizer> {
        let fertilizer = self
            .fertilizers
            .iter()
            .find(|fertilizer| fertilizer.id() == *fertilizer_id);

        match fertilizer {
            Some(fertilizer) => Some(*fertilizer),
            None => None,
        }
    }

    fn fertilizers(&self) -> Vec<&Fertilizer> {
        self.fertilizers
            .iter()
            .filter(|fertilizer| !self.excluded_fertilizers.contains_key(&fertilizer.id()))
            .map(|fertilizer| *fertilizer)
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
