use super::Error;
use crate::model::chemistry::Nutrient;
use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::Profile;
use crate::model::solutions::Solution;
// use ellp::{Bound, ConstraintOp, DualSimplexSolver, Problem, SolverResult};
use minilp::{ComparisonOp, OptimizationDirection, Problem, Variable};

pub struct Solver {
    desired_profile: Profile,
    fertilizers: Vec<Fertilizer>,
    problem: Problem,
    coefficients: Vec<(Variable, Nutrient)>,
}

impl Solver {
    pub fn new(desired_profile: Profile, fertilizers: Vec<Fertilizer>) -> Result<Self, Error> {
        let mut calculation = Self {
            problem: Problem::new(OptimizationDirection::Minimize),
            desired_profile: desired_profile.clone(),
            fertilizers: Vec::new(),
            coefficients: Vec::new(),
        };

        fertilizers.iter().for_each(|fertilizer| {
            calculation.add_fertilizer(fertilizer);
        });

        desired_profile
            .nutrients()
            .iter()
            .for_each(|required_nutrient| {
                calculation.add_nutrient_requirement(*required_nutrient);
            });

        Ok(calculation)
    }

    fn add_fertilizer(&mut self, fertilizer: &Fertilizer) {
        self.fertilizers.push(fertilizer.clone());

        let variable_id = self.problem.add_var(1., (0.0, f64::INFINITY));

        fertilizer.nutrients().iter().for_each(|nutrient| {
            self.coefficients.push((variable_id, *nutrient));
        });
    }

    fn add_nutrient_requirement(&mut self, nutrient: Nutrient) {
        let coefficients: Vec<(Variable, f64)> = self
            .coefficients
            .iter()
            .filter(|coefficient| coefficient.1.index() == nutrient.index())
            .map(|coefficient| (coefficient.0, coefficient.1.value()))
            .collect();

        match nutrient {
            Nutrient::Nitrogen(required_amount)
            | Nutrient::NitrogenNitrate(required_amount)
            | Nutrient::NitrogenAmmonium(required_amount)
            | Nutrient::Phosphorus(required_amount)
            | Nutrient::Potassium(required_amount)
            | Nutrient::Magnesium(required_amount)
            | Nutrient::Calcium(required_amount)
            | Nutrient::Boron(required_amount) => {
                self.problem
                    .add_constraint(coefficients, ComparisonOp::Eq, required_amount);
            }
            Nutrient::Sulfur(_)
            | Nutrient::Iron(_)
            | Nutrient::Manganese(_)
            | Nutrient::Zinc(_)
            | Nutrient::Copper(_)
            | Nutrient::Molybdenum(_) => {
                self.problem
                    .add_constraint(coefficients, ComparisonOp::Ge, 0.0);
            }
        };
    }

    pub fn solve(&self) -> Result<Solution, Error> {
        let solution = self.problem.solve();

        match solution {
            Ok(solution) => {
                println!("solution {:#?}", solution);

                Ok(Solution::empty(self.fertilizers.clone()))
            }

            Err(error) => {
                println!("calculation error {:#?}", error);

                Ok(Solution::empty(self.fertilizers.clone()))
            }
        }
    }
}

#[cfg(test)]
mod tests {}
