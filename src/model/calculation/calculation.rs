use super::{Coefficient, Error};
use crate::model::chemistry::{NitrogenForm, NutrientAmount};
use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::Profile;
use crate::model::solutions::Solution;
use ellp::{Bound, ConstraintOp, DualSimplexSolver, Problem, SolverResult};

pub struct Calculation {
    desired_profile: Profile,
    fertilizers: Vec<Fertilizer>,
    problem: Problem,
    coefficients: Vec<Coefficient>,
}

impl Calculation {
    pub fn new(desired_profile: Profile, fertilizers: Vec<Fertilizer>) -> Result<Self, Error> {
        let mut calculation = Self {
            problem: Problem::new(),
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
            .for_each(|required_amount| {
                calculation.add_nutrient_amount_requirement(*required_amount);
            });

        desired_profile
            .nitrogen_forms()
            .iter()
            .for_each(|nitrogen_form| {
                calculation.add_nitrogen_form_requirement(*nitrogen_form);
            });

        Ok(calculation)
    }

    fn add_fertilizer(&mut self, fertilizer: &Fertilizer) {
        self.fertilizers.push(fertilizer.clone());

        let variable_name = format!("x{}", self.problem.variables.len() + 1);

        let variable_id = self
            .problem
            .add_var(1., Bound::Lower(0.), Some(variable_name))
            .unwrap();

        fertilizer.nutrients().iter().for_each(|nutrient_amount| {
            self.coefficients
                .push(Coefficient::NutrientAmount(*nutrient_amount, variable_id));
        });

        fertilizer
            .nitrogen_forms()
            .iter()
            .for_each(|nitrogen_form| {
                self.coefficients
                    .push(Coefficient::NitrogenForm(*nitrogen_form, variable_id));
            })
    }

    fn add_nutrient_amount_requirement(&mut self, requirement: NutrientAmount) {
        let coefficients = self
            .coefficients
            .iter()
            .filter(|coefficient| match coefficient {
                Coefficient::NutrientAmount(nutrient_amount, _) => {
                    nutrient_amount.index() == requirement.index()
                }

                _ => false,
            })
            .map(|coefficient| coefficient.value())
            .collect();

        match requirement {
            NutrientAmount::Nitrogen(required_amount)
            | NutrientAmount::Phosphorus(required_amount)
            | NutrientAmount::Potassium(required_amount)
            | NutrientAmount::Magnesium(required_amount)
            | NutrientAmount::Calcium(required_amount)
            | NutrientAmount::Boron(required_amount) => {
                self.problem
                    .add_constraint(coefficients, ConstraintOp::Eq, required_amount)
                    .unwrap();
            }
            NutrientAmount::Sulfur(_)
            | NutrientAmount::Iron(_)
            | NutrientAmount::Manganese(_)
            | NutrientAmount::Zinc(_)
            | NutrientAmount::Copper(_)
            | NutrientAmount::Molybdenum(_) => {
                self.problem
                    .add_constraint(coefficients, ConstraintOp::Gte, 0.0)
                    .unwrap();
            }
        };
    }

    fn add_nitrogen_form_requirement(&mut self, requirement: NitrogenForm) {
        let coefficients = self
            .coefficients
            .iter()
            .filter(|coefficient| match coefficient {
                Coefficient::NitrogenForm(nitrogen_form, _) => {
                    nitrogen_form.index() == requirement.index()
                }

                _ => false,
            })
            .map(|coefficient| coefficient.value())
            .collect();

        match requirement {
            NitrogenForm::Nitrate(required_amount) | NitrogenForm::Ammonium(required_amount) => {
                self.problem
                    .add_constraint(coefficients, ConstraintOp::Eq, required_amount)
                    .unwrap();
            }
        };
    }

    pub fn solve(&self, tank_size: i32) -> Result<Solution, Error> {
        println!("{}", self.problem);

        let result = DualSimplexSolver::default()
            .solve(self.problem.clone())
            .unwrap();

        if let SolverResult::Optimal(sol) = result {
            let result = sol.x();

            let mut solution = Solution::from(self.desired_profile.clone());

            result.iter().enumerate().for_each(|(idx, amount)| {
                if let Some(fertilizer) = self.fertilizers.get(idx) {
                    solution.add_fertilizer_weight(fertilizer.clone(), *amount);
                }
            });

            Ok(solution)
        } else {
            Err(Error::new("impossible profile".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    /*
    use super::Calculation;
    use crate::calculation::desired_profile::DesiredProfile;
    use crate::calculation::requirement::Requirement;
    use crate::chemistry::Nutrient;
    use crate::fertilizers::{Component, FertilizerBuilder, Units};

    #[test]
    fn basic_nutrient_profile() {
        let fertilizer_builder = FertilizerBuilder::new();

        let desired_profile = DesiredProfile::new(vec![
            Requirement::new(Nutrient::Nitrogen, 189.),
            Requirement::new(Nutrient::Phosphorus, 39.),
            Requirement::new(Nutrient::Potassium, 341.),
            Requirement::new(Nutrient::Calcium, 170.),
            Requirement::new(Nutrient::Magnesium, 48.),
            Requirement::new(Nutrient::Sulfur, 150.),
            Requirement::new(Nutrient::Iron, 2.),
            Requirement::new(Nutrient::Manganese, 0.55),
            Requirement::new(Nutrient::Zink, 0.33),
            Requirement::new(Nutrient::Boron, 0.28),
            Requirement::new(Nutrient::Copper, 0.05),
            Requirement::new(Nutrient::Molybdenum, 0.05),
        ]);

        let fertilizers = vec![
            fertilizer_builder
                .from_formula("Ca(NO3)2")
                .name("calcium nitrate"),
            fertilizer_builder
                .from_formula("KNO3")
                .name("potassium nitrate"),
            fertilizer_builder
                .from_formula("NH4NO3")
                .name("ammonium nitrate"),
            fertilizer_builder
                .from_formula("MgSO4*7H2O")
                .name("magnesium sulfate"),
            fertilizer_builder
                .from_formula("K2SO4")
                .name("potassium sulfate"),
            fertilizer_builder
                .from_formula("KH2PO4")
                .name("monopotassium phosphate"),
            fertilizer_builder
                .from_label(
                    Units::WeightVolume,
                    vec![
                        Component::Magnesium(Some(15000.), None),
                        Component::Iron(Some(3200.)),
                        Component::Manganese(Some(1600.)),
                        Component::Boron(Some(1200.)),
                        Component::Zink(Some(360.)),
                        Component::Copper(Some(320.)),
                        Component::Molybdenum(Some(102.)),
                    ],
                )
                .name("uniflor micro"),
            /*
            fertilizer_builder
                .from_formula("C14H18N3O10Fe(NH4)2")
                .name("iron chelate"),
            fertilizer_builder
                .from_formula("MnSO4*H2O")
                .name("manganese sulfate"),
            fertilizer_builder.from_formula("H3BO3").name("boric acid"),
            fertilizer_builder
                .from_formula("Na2MoO4*2H2O")
                .name("molybdenum acid"),
            fertilizer_builder
                .from_formula("ZnSO4*7H2O")
                .name("zink sulfate"),
            fertilizer_builder
                .from_formula("CuSO4*5H2O")
                .name("copper sulfate"),
            */
        ];

        match Calculation::new(desired_profile, fertilizers) {
            Ok(solver) => match solver.solve(1) {
                Ok(_) => {
                    println!("solved");
                }
                Err(error) => {
                    println!("{:#?}", error);
                }
            },
            Err(errors) => {
                println!("{:#?}", errors);
            }
        }
    }
    */
}
