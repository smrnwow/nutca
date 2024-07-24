use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::Profile;
use crate::model::solutions::{Calculation, ExclusionReason, FertilizerWeight, FertilizersSet};
use std::collections::HashMap;

pub struct Solver<'a> {
    profile: &'a Profile,
    fertilizers: Vec<&'a Fertilizer>,
    sources: HashMap<String, String>,
    excluded_fertilizers: HashMap<String, ExclusionReason>,
}

impl<'a> Solver<'a> {
    pub fn new(profile: &'a Profile, fertilizers: Vec<&'a Fertilizer>) -> Self {
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

    pub fn solve(&mut self) -> FertilizersSet {
        loop {
            let calculation = Calculation::new(&self.profile, self.fertilizers());

            match calculation.calculate() {
                Ok(fertilizers_weights) => match self.test_negative(&fertilizers_weights) {
                    Some(fertilizer_id) => {
                        self.exclude_fertilizer(fertilizer_id, ExclusionReason::NegativeAmount);
                        continue;
                    }

                    None => {
                        let mut fertilizers_set = FertilizersSet::new(fertilizers_weights);

                        self.excluded_fertilizers.iter().for_each(|exclusion| {
                            if let Some(fertilizer) = self.find_fertilizer(exclusion.0) {
                                let fertilizer_weight = FertilizerWeight::from(fertilizer.clone());
                                fertilizers_set.add_fertilizer_weight(fertilizer_weight);
                            }
                        });

                        return fertilizers_set;
                    }
                },

                Err(_) => {
                    let fertilizers = self.fertilizers();

                    let last_index = fertilizers.len() - 1;

                    match fertilizers.get(last_index) {
                        Some(fertilizer) => {
                            self.exclude_fertilizer(
                                fertilizer.id(),
                                ExclusionReason::RedurantFertilizer,
                            );
                        }

                        None => {
                            return FertilizersSet::from(&self.fertilizers);
                        }
                    }
                }
            }
        }
    }

    fn add_fertilizer(&mut self, fertilizer: &'a Fertilizer) {
        self.test_nutrients_source_duplication(fertilizer);

        if self.test_nullish_nutrient_requirement(fertilizer) {
            self.exclude_fertilizer(fertilizer.id(), ExclusionReason::NullishNutrientRequirement);
        }

        self.fertilizers.push(fertilizer);
    }

    fn exclude_fertilizer(&mut self, fertilizer_id: String, reason: ExclusionReason) {
        self.excluded_fertilizers
            .entry(fertilizer_id)
            .or_insert(reason);
    }

    fn test_nutrients_source_duplication(&mut self, fertilizer: &Fertilizer) {
        let nutrients_string = fertilizer.nutrients().stringify();

        match self.sources.get(&nutrients_string) {
            Some(same_nutrients_source) => {
                if let Some(other_fertilizer) = self.find_fertilizer(same_nutrients_source) {
                    if fertilizer.contains_more_nutrients(other_fertilizer) {
                        self.exclude_fertilizer(
                            other_fertilizer.id(),
                            ExclusionReason::DuplicatedNutrientSource,
                        );

                        self.sources.insert(nutrients_string, fertilizer.id());
                    } else {
                        self.exclude_fertilizer(
                            fertilizer.id(),
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

    fn test_nullish_nutrient_requirement(&self, fertilizer: &Fertilizer) -> bool {
        let nutrients = fertilizer.nutrients().list();

        let mut has_nullish_requirement = false;

        if nutrients.len() < 4 {
            nutrients.iter().for_each(|nutrient_amount| {
                let nutrient_requirement = self
                    .profile
                    .nutrient_requirement(nutrient_amount.nutrient());

                if nutrient_requirement.value() == 0.0 {
                    has_nullish_requirement = true;
                }
            });
        }

        has_nullish_requirement
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

        match negatives.get(0) {
            Some(negative) => Some(negative.id()),
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
    use super::Solver;
    use crate::model::chemistry::{Nutrient, NutrientAmount};
    use crate::model::fertilizers::{labels::Component, FertilizerBuilder};
    use crate::model::profiles::ProfileBuilder;

    #[test]
    fn duplicated_nutrients_source_excluded() {
        let profile = ProfileBuilder::new()
            .nutrient_requirement(NutrientAmount::Nitrogen(10.0))
            .nutrient_requirement(NutrientAmount::NitrogenNitrate(10.0))
            .nutrient_requirement(NutrientAmount::Potassium(20.0))
            .build();

        let fertilizer_1 = FertilizerBuilder::new()
            .label_component(Component::Nitrogen(10.0))
            .label_component(Component::Potassium(20.0))
            .build();

        let fertilizer_2 = FertilizerBuilder::new()
            .label_component(Component::Nitrogen(20.0))
            .label_component(Component::Potassium(40.0))
            .build();

        let fertilizer_3 = FertilizerBuilder::new()
            .label_component(Component::Nitrogen(15.0))
            .label_component(Component::Potassium(30.0))
            .build();

        let mut solver = Solver::new(&profile, vec![&fertilizer_1, &fertilizer_2, &fertilizer_3]);

        let fertilizers_weights = solver.solve().list();

        assert_eq!(
            0.0,
            fertilizers_weights
                .iter()
                .find(|fertilizer| fertilizer.id() == fertilizer_1.id())
                .unwrap()
                .weight()
        );

        assert_eq!(
            0.5,
            fertilizers_weights
                .iter()
                .find(|fertilizer| fertilizer.id() == fertilizer_2.id())
                .unwrap()
                .weight()
        );

        assert_eq!(
            0.0,
            fertilizers_weights
                .iter()
                .find(|fertilizer| fertilizer.id() == fertilizer_3.id())
                .unwrap()
                .weight()
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
            .label_component(Component::Calcium(10.0))
            .build();

        let fertilizer_2 = FertilizerBuilder::new()
            .label_component(Component::Phosphor(20.0))
            .label_component(Component::Potassium(40.0))
            .build();

        let fertilizer_3 = FertilizerBuilder::new()
            .label_component(Component::Nitrogen(20.0))
            .label_component(Component::Potassium(40.0))
            .label_component(Component::Phosphor(10.0))
            .label_component(Component::Calcium(10.0))
            .label_component(Component::Sulfur(10.0))
            .build();

        let mut solver = Solver::new(&profile, vec![&fertilizer_1, &fertilizer_2, &fertilizer_3]);

        let fertilizers_weights = solver.solve().list();

        assert_eq!(
            0.0,
            fertilizers_weights
                .iter()
                .find(|fertilizer| fertilizer.id() == fertilizer_1.id())
                .unwrap()
                .weight()
        );

        assert_eq!(
            0.0,
            fertilizers_weights
                .iter()
                .find(|fertilizer| fertilizer.id() == fertilizer_2.id())
                .unwrap()
                .weight()
        );

        assert_eq!(
            0.5,
            fertilizers_weights
                .iter()
                .find(|fertilizer| fertilizer.id() == fertilizer_3.id())
                .unwrap()
                .weight()
        );
    }

    #[test]
    fn desired_and_result_profiles_equals() {
        let profile = ProfileBuilder::new()
            .nutrient_requirement(NutrientAmount::Nitrogen(10.0))
            .nutrient_requirement(NutrientAmount::NitrogenNitrate(10.0))
            .nutrient_requirement(NutrientAmount::Potassium(20.0))
            .nutrient_requirement(NutrientAmount::Calcium(20.0))
            .build();

        let fertilizer_1 = FertilizerBuilder::new()
            .label_component(Component::Calcium(10.0))
            .build();

        let fertilizer_2 = FertilizerBuilder::new()
            .label_component(Component::Phosphor(20.0))
            .label_component(Component::Potassium(40.0))
            .build();

        let fertilizer_3 = FertilizerBuilder::new()
            .label_component(Component::Nitrogen(20.0))
            .label_component(Component::Potassium(40.0))
            .label_component(Component::Phosphor(10.0))
            .label_component(Component::Calcium(10.0))
            .label_component(Component::Sulfur(10.0))
            .build();

        let mut solver = Solver::new(&profile, vec![&fertilizer_1, &fertilizer_2, &fertilizer_3]);

        let nutrients = solver.solve().nutrients();

        assert_eq!(
            NutrientAmount::Nitrogen(10.0),
            nutrients.value_of(Nutrient::Nitrogen)
        );

        assert_eq!(
            NutrientAmount::NitrogenNitrate(10.0),
            nutrients.value_of(Nutrient::NitrogenNitrate)
        );

        assert_eq!(
            NutrientAmount::Potassium(20.0),
            nutrients.value_of(Nutrient::Potassium)
        );

        assert_eq!(
            NutrientAmount::Calcium(20.0),
            nutrients.value_of(Nutrient::Calcium)
        );
    }
}
