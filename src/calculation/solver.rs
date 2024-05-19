use super::desired_profile::DesiredProfile;
use super::error::Error;
use super::result_profile::ResultProfile;
use crate::chemistry::Nutrient;
use crate::fertilizers::Fertiliser;
use ellp::{Bound, ConstraintOp, DualSimplexSolver, Problem, SolverResult};

pub struct Solver {
    desired_profile: DesiredProfile,
    fertilizers: Vec<Fertiliser>,
    problem: Problem,
}

impl Solver {
    pub fn new(
        desired_profile: DesiredProfile,
        fertilizers: Vec<Fertiliser>,
    ) -> Result<Self, Error> {
        let mut solver = Self {
            problem: Problem::new(),
            desired_profile,
            fertilizers,
        };

        solver.setup_coefficients();

        solver.setup_constraints();

        Ok(solver)
    }

    fn setup_coefficients(&mut self) {
        self.fertilizers.iter().for_each(|fertilizer| {
            let variable_name = format!("x{}", self.problem.variables.len() + 1);

            let variable_id = self
                .problem
                .add_var(1., Bound::Lower(0.), Some(variable_name))
                .unwrap();

            fertilizer.nutrients().iter().for_each(|nutrient| {
                self.desired_profile
                    .add_coefficient(&nutrient.symbol(), (variable_id, nutrient.percent()));
            });
        });
    }

    fn setup_constraints(&mut self) {
        self.desired_profile
            .requirements()
            .iter()
            .for_each(|requirement| {
                match requirement.symbol() {
                    Nutrient::Nitrogen
                    | Nutrient::NitrogenNitrate
                    | Nutrient::NitrogenAmmonium
                    | Nutrient::Phosphorus
                    | Nutrient::Potassium
                    | Nutrient::Magnesium
                    | Nutrient::Calcium
                    | Nutrient::Boron => {
                        self.problem
                            .add_constraint(
                                requirement.coefficients(),
                                ConstraintOp::Eq,
                                requirement.amount(),
                            )
                            .unwrap();
                    }
                    Nutrient::Sulfur
                    | Nutrient::Iron
                    | Nutrient::Manganese
                    | Nutrient::Zink
                    | Nutrient::Copper
                    | Nutrient::Molybdenum => {
                        self.problem
                            .add_constraint(requirement.coefficients(), ConstraintOp::Gte, 0.0)
                            .unwrap();
                    }
                    _ => {}
                };
            });
    }

    pub fn solve(&self, tank_size: i32) -> Result<(), Error> {
        println!("{}", self.problem);

        let result = DualSimplexSolver::default()
            .solve(self.problem.clone())
            .unwrap();

        if let SolverResult::Optimal(sol) = result {
            let result = sol.x();

            let mut result_vector = Vec::new();

            result.iter().enumerate().for_each(|(idx, amount)| {
                if let Some(fertilizer) = self.fertilizers.get(idx) {
                    result_vector.push((fertilizer, *amount));
                }
            });

            let result_profile = ResultProfile::from(result_vector);

            println!("result_profile = {:#?}", result_profile);

            Ok(())
        } else {
            Err(Error::new("impossible profile".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;
    use crate::calculation::desired_profile::DesiredProfile;
    use crate::calculation::requirement::Requirement;
    use crate::chemistry::Nutrient;
    use crate::fertilizers::labels::{Component, Units};
    use crate::fertilizers::FertilizerBuilder;

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

        match Solver::new(desired_profile, fertilizers) {
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
}
