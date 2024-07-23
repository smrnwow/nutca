use crate::model::chemistry::NutrientAmount;
use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::Profile;
use crate::model::solutions::FertilizerWeight;
use ellp::{problem::VariableId, Bound, ConstraintOp, DualSimplexSolver, Problem, SolverResult};

pub struct Calculation {
    fertilizers: Vec<Fertilizer>,
    problem: Problem,
    coefficients: Vec<Vec<(VariableId, f64)>>,
}

impl Calculation {
    pub fn new() -> Self {
        Self {
            problem: Problem::new(),
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
        }
    }

    pub fn solve(&self) -> Result<Vec<FertilizerWeight>, ()> {
        if self.fertilizers.len() == 0 {
            return Ok(Vec::new());
        }

        // println!("problem: {}", self.problem);

        if let Ok(result) = DualSimplexSolver::default().solve(self.problem.clone()) {
            if let SolverResult::Optimal(solution) = result {
                let mut fertilizers_weights = Vec::new();

                solution.x().iter().enumerate().for_each(|(index, amount)| {
                    if let Some(fertilizer) = self.fertilizers.get(index) {
                        fertilizers_weights
                            .push(FertilizerWeight::new(fertilizer.clone(), *amount));
                    }
                });

                return Ok(fertilizers_weights);
            }
        }

        Err(())
    }

    pub fn with_fertilizers(mut self, fertilizers: Vec<Fertilizer>) -> Self {
        fertilizers.iter().for_each(|fertilizer| {
            self.add_variable(fertilizer);
        });

        self
    }

    pub fn with_profile(mut self, profile: &Profile) -> Self {
        profile
            .nutrients()
            .list()
            .iter()
            .for_each(|required_nutrient| {
                self.add_constraint(*required_nutrient);
            });

        self
    }

    fn add_variable(&mut self, fertilizer: &Fertilizer) {
        self.fertilizers.push(fertilizer.clone());

        let variable_name = format!("x{}", self.problem.variables.len() + 1);

        let variable_id = self
            .problem
            .add_var(1.0, Bound::Lower(0.0), Some(variable_name))
            .unwrap();

        fertilizer.nutrients().list().iter().for_each(|nutrient| {
            self.coefficients[nutrient.nutrient().index()].push((variable_id, nutrient.value()));
        });
    }

    fn add_constraint(&mut self, nutrient: NutrientAmount) {
        if let Some(coefficients) = self.coefficients.get(nutrient.nutrient().index()) {
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
mod tests {}
