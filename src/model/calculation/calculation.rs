use crate::model::calculation::Error;
use crate::model::chemistry::Nutrient;
use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::Profile;
use crate::model::solutions::Solution;
use ellp::{problem::VariableId, Bound, ConstraintOp, DualSimplexSolver, Problem, SolverResult};

fn round(value: f64) -> f64 {
    format!("{:.6}", value).parse().unwrap()
}

pub struct Calculation {
    desired_profile: Profile,
    fertilizers: Vec<Fertilizer>,
    problem: Problem,
    coefficients: Vec<Vec<(VariableId, f64)>>,
}

impl Calculation {
    pub fn new(desired_profile: Profile, fertilizers: Vec<Fertilizer>) -> Result<Self, Error> {
        let mut calculation = Self {
            problem: Problem::new(),
            desired_profile,
            fertilizers: Vec::new(),
            coefficients: Vec::from([
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            ]),
        };

        fertilizers.iter().for_each(|fertilizer| {
            calculation.add_fertilizer(fertilizer);
        });

        calculation
            .desired_profile
            .nutrients()
            .iter()
            .for_each(|required_nutrient| {
                calculation.add_nutrient_requirement(*required_nutrient);
            });

        Ok(calculation)
    }

    fn add_fertilizer(&mut self, fertilizer: &Fertilizer) {
        self.fertilizers.push(fertilizer.clone());

        let variable_name = format!("x{}", self.problem.variables.len() + 1);

        let variable_id = self
            .problem
            .add_var(1., Bound::Lower(0.0), Some(variable_name))
            .unwrap();

        fertilizer.nutrients().iter().for_each(|nutrient| {
            self.coefficients[nutrient.index()].push((variable_id, nutrient.value()));
        });
    }

    fn add_nutrient_requirement(&mut self, nutrient: Nutrient) {
        let coefficients = self.coefficients[nutrient.index()].clone();

        if coefficients.len() > 0 {
            match nutrient {
                Nutrient::Nitrogen(required_amount)
                | Nutrient::NitrogenNitrate(required_amount)
                | Nutrient::NitrogenAmmonium(required_amount)
                | Nutrient::Phosphorus(required_amount)
                | Nutrient::Potassium(required_amount)
                | Nutrient::Magnesium(required_amount)
                | Nutrient::Calcium(required_amount)
                | Nutrient::Iron(required_amount)
                | Nutrient::Manganese(required_amount)
                | Nutrient::Zinc(required_amount)
                | Nutrient::Copper(required_amount)
                | Nutrient::Boron(required_amount)
                | Nutrient::Molybdenum(required_amount) => {
                    self.problem
                        .add_constraint(coefficients, ConstraintOp::Eq, required_amount)
                        .unwrap();
                }
                Nutrient::Sulfur(_) => {
                    self.problem
                        .add_constraint(coefficients, ConstraintOp::Gte, 0.0)
                        .unwrap();
                }
            };
        }
    }

    pub fn solve(&self) -> Result<Solution, Error> {
        // println!("{}", self.problem);

        let result = DualSimplexSolver::default()
            .solve(self.problem.clone())
            .unwrap();

        if let SolverResult::Optimal(sol) = result {
            let result = sol.x();

            let mut solution = Solution::from(self.desired_profile.clone());

            result.iter().enumerate().for_each(|(idx, amount)| {
                if let Some(fertilizer) = self.fertilizers.get(idx) {
                    solution.add_fertilizer_weight(fertilizer.clone(), round(*amount / 10.));
                }
            });

            Ok(solution)
        } else {
            Err(Error::new("impossible profile".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {}
