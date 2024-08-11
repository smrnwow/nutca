use crate::model::chemistry::{Nutrient, NutrientAmount};
use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::Profile;
use crate::model::solutions::FertilizerWeight;
use ellp::{problem::VariableId, Bound, ConstraintOp, DualSimplexSolver, Problem, SolverResult};
use std::collections::HashMap;

pub struct Calculation<'a> {
    fertilizers: Vec<&'a Fertilizer>,
    problem: Problem,
    coefficients: HashMap<Nutrient, Vec<(VariableId, f64)>>,
}

impl<'a> Calculation<'a> {
    /// create new calculation instance and setup linear programming problem
    pub fn new(profile: &Profile, fertilizers: Vec<&'a Fertilizer>) -> Self {
        let mut calculation = Self {
            fertilizers: Vec::new(),
            problem: Problem::new(),
            coefficients: HashMap::new(),
        };

        fertilizers
            .iter()
            .for_each(|fertilizer| calculation.add_variable(*fertilizer));

        profile
            .nutrients()
            .list()
            .iter()
            .for_each(|required_nutrient| {
                calculation.add_constraint(*required_nutrient);
            });

        calculation
    }

    /// calculate fertilizers weights to achieve desired nutrients profile
    pub fn calculate(self) -> Result<Vec<FertilizerWeight>, ()> {
        if let Ok(result) = DualSimplexSolver::default().solve(self.problem) {
            if let SolverResult::Optimal(solution) = result {
                let mut fertilizers_weights = Vec::new();

                solution.x().iter().enumerate().for_each(|(index, amount)| {
                    if let Some(fertilizer) = self.fertilizers.get(index) {
                        let fertilizer = *fertilizer;

                        fertilizers_weights
                            .push(FertilizerWeight::new(fertilizer.clone(), *amount));
                    }
                });

                return Ok(fertilizers_weights);
            }
        }

        Err(())
    }

    fn add_variable(&mut self, fertilizer: &'a Fertilizer) {
        self.fertilizers.push(fertilizer);

        let variable_name = format!("x{}", self.problem.variables.len() + 1);

        let variable_id = self
            .problem
            .add_var(1.0, Bound::Lower(0.0), Some(variable_name))
            .unwrap();

        fertilizer
            .nutrients()
            .list()
            .iter()
            .for_each(|nutrient_amount| {
                let coefficient = (variable_id, nutrient_amount.value());

                self.coefficients
                    .entry(nutrient_amount.nutrient())
                    .and_modify(|coefficients| coefficients.push(coefficient))
                    .or_insert(Vec::from([coefficient]));
            });
    }

    fn add_constraint(&mut self, nutrient: NutrientAmount) {
        if let Some(coefficients) = self.coefficients.get(&nutrient.nutrient()) {
            if coefficients.len() > 0 {
                let (op, nutrient_requirement) = match nutrient {
                    NutrientAmount::Sulfur(_) => (ConstraintOp::Gte, 0.0),
                    _ => (ConstraintOp::Eq, nutrient.value()),
                };

                self.problem
                    .add_constraint(coefficients.clone(), op, nutrient_requirement)
                    .unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Calculation;
    use crate::model::chemistry::NutrientAmount;
    use crate::model::fertilizers::{FertilizerBuilder, LabelComponent};
    use crate::model::profiles::ProfileBuilder;

    #[test]
    fn correctly_handles_empty_fertilizers() {
        let profile = ProfileBuilder::new()
            .nutrient_requirement(NutrientAmount::Nitrogen(10.0))
            .nutrient_requirement(NutrientAmount::NitrogenNitrate(10.0))
            .nutrient_requirement(NutrientAmount::NitrogenAmmonium(0.0))
            .build();

        let calculation = Calculation::new(&profile, vec![]);

        assert_eq!(0, calculation.calculate().unwrap().len());
    }

    #[test]
    fn correctly_handles_single_fertilizer() {
        let profile = ProfileBuilder::new()
            .nutrient_requirement(NutrientAmount::Nitrogen(10.0))
            .nutrient_requirement(NutrientAmount::NitrogenNitrate(10.0))
            .build();

        let fertilizer = FertilizerBuilder::new()
            .label_component(LabelComponent::Nitrogen(10.))
            .build();

        let calculation = Calculation::new(&profile, vec![&fertilizer]);

        let result = calculation.calculate().unwrap();

        assert_eq!(1, result.len());

        assert_eq!(1.0, result.get(0).unwrap().weight());
    }

    #[test]
    fn correctly_handles_multiple_fertilizers() {
        let profile = ProfileBuilder::new()
            .nutrient_requirement(NutrientAmount::Nitrogen(10.0))
            .nutrient_requirement(NutrientAmount::NitrogenNitrate(10.0))
            .nutrient_requirement(NutrientAmount::Phosphorus(10.0))
            .nutrient_requirement(NutrientAmount::Potassium(20.0))
            .build();

        let fertilizer_1 = FertilizerBuilder::new()
            .label_component(LabelComponent::Nitrogen(10.))
            .label_component(LabelComponent::Potassium(10.))
            .build();

        let fertilizer_2 = FertilizerBuilder::new()
            .label_component(LabelComponent::Phosphorus(10.))
            .label_component(LabelComponent::Potassium(10.))
            .build();

        let calculation = Calculation::new(&profile, vec![&fertilizer_1, &fertilizer_2]);

        let result = calculation.calculate().unwrap();

        assert_eq!(2, result.len());

        assert_eq!(1.0, result.get(0).unwrap().weight());

        assert_eq!(1.0, result.get(1).unwrap().weight());
    }

    #[test]
    fn correctly_handles_complex_problem() {
        let profile = ProfileBuilder::new()
            .nutrient_requirement(NutrientAmount::Nitrogen(10.0))
            .nutrient_requirement(NutrientAmount::NitrogenNitrate(10.0))
            .nutrient_requirement(NutrientAmount::Phosphorus(10.0))
            .nutrient_requirement(NutrientAmount::Potassium(30.0))
            .nutrient_requirement(NutrientAmount::Calcium(15.0))
            .build();

        let fertilizer_1 = FertilizerBuilder::new()
            .label_component(LabelComponent::Nitrogen(5.))
            .label_component(LabelComponent::Calcium(15.))
            .build();

        let fertilizer_2 = FertilizerBuilder::new()
            .label_component(LabelComponent::Nitrogen(10.))
            .label_component(LabelComponent::Potassium(10.))
            .build();

        let fertilizer_3 = FertilizerBuilder::new()
            .label_component(LabelComponent::Phosphorus(10.))
            .label_component(LabelComponent::Potassium(10.))
            .build();

        let fertilizer_4 = FertilizerBuilder::new()
            .label_component(LabelComponent::Potassium(10.))
            .label_component(LabelComponent::Sulfur(10.))
            .build();

        let calculation = Calculation::new(
            &profile,
            vec![&fertilizer_1, &fertilizer_2, &fertilizer_3, &fertilizer_4],
        );

        let result = calculation.calculate().unwrap();

        assert_eq!(4, result.len());

        assert_eq!(1.0, result.get(0).unwrap().weight());

        assert_eq!(0.5, result.get(1).unwrap().weight());

        assert_eq!(1.0, result.get(2).unwrap().weight());

        assert_eq!(1.5, result.get(3).unwrap().weight());
    }
}
