use super::composition::Composition;
use super::error::Error;
use crate::fertilizers::fertilizer::Fertiliser;
use ellp::{Bound, ConstraintOp, DualSimplexSolver, Problem, SolverResult};

pub struct Solver<'a> {
    composition: Composition<'a>,
    fertilizers: Vec<Fertiliser>,
    problem: Problem,
}

impl<'a> Solver<'a> {
    pub fn new(composition: Composition<'a>, fertilizers: Vec<Fertiliser>) -> Result<Self, Error> {
        let mut solver = Self {
            problem: Problem::new(),
            composition,
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
                self.composition.add_coefficient(
                    &nutrient.element.symbol,
                    (variable_id, nutrient.percent.into()),
                );
            });
        });
    }

    fn setup_constraints(&mut self) {
        self.composition.nutrients().iter().for_each(|nutrient| {
            match nutrient.symbol() {
                "N" | "P" | "K" | "Mg" | "Ca" => {
                    self.problem
                        .add_constraint(
                            nutrient.coefficients(),
                            ConstraintOp::Eq,
                            nutrient.amount(),
                        )
                        .unwrap();
                }
                "B" => {
                    self.problem
                        .add_constraint(
                            nutrient.coefficients(),
                            ConstraintOp::Eq,
                            nutrient.amount(),
                        )
                        .unwrap();
                }
                "S" | "Fe" | "Mn" | "Zn" | "Cu" | "Mo" => {
                    self.problem
                        .add_constraint(nutrient.coefficients(), ConstraintOp::Gte, 0.0)
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

            result.iter().enumerate().for_each(|(idx, amount)| {
                if let Some(fertilizer) = self.fertilizers.get(idx) {
                    println!(
                        "{:#?} amount = {}",
                        fertilizer.get_name(),
                        amount / 10. * tank_size as f64
                    );
                }
            });

            Ok(())
        } else {
            Err(Error::new("impossible profile".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solver;
    use crate::calculation::composition::Composition;
    use crate::calculation::nutrient;
    use crate::fertilizers::fertilizer::Fertiliser;
    use crate::fertilizers::labels::label::Label;
    use crate::fertilizers::labels::nutrient::Nutrient;
    use crate::fertilizers::labels::units::Units;
    use crate::formula::builder::Builder;

    #[test]
    fn basic_nutrient_profile() {
        let formula = Builder::new();

        /*
        profile.add_fertilizer(Fertiliser::from_formula(
            "iron chelate",
            "",
            formula_builder.build("C14H18N3O10Fe(NH4)2").unwrap(),
        ));

        profile.add_fertilizer(Fertiliser::from_formula(
            "manganese sulfate",
            "",
            formula_builder.build("MnSO4*H2O").unwrap(),
        ));

        profile.add_fertilizer(Fertiliser::from_formula(
            "boric acid",
            "",
            formula_builder.build("H3BO3").unwrap(),
        ));

        profile.add_fertilizer(Fertiliser::from_formula(
            "molibden acid",
            "",
            formula_builder.build("Na2MoO4*2H2O").unwrap(),
        ));

        profile.add_fertilizer(Fertiliser::from_formula(
            "zink sulfate",
            "",
            formula_builder.build("ZnSO4*7H2O").unwrap(),
        ));

        profile.add_fertilizer(Fertiliser::from_formula(
            "cuprum sulfate",
            "",
            formula_builder.build("CuSO4*5H2O").unwrap(),
        ));
        */

        match Solver::new(
            Composition::new(&vec![
                nutrient::Nutrient::new("N", 189.),
                nutrient::Nutrient::new("P", 39.),
                nutrient::Nutrient::new("K", 341.),
                nutrient::Nutrient::new("Ca", 170.),
                nutrient::Nutrient::new("Mg", 48.),
                nutrient::Nutrient::new("S", 150.),
                nutrient::Nutrient::new("Fe", 2.),
                nutrient::Nutrient::new("Mn", 0.55),
                nutrient::Nutrient::new("Zn", 0.33),
                nutrient::Nutrient::new("B", 0.28),
                nutrient::Nutrient::new("Cu", 0.05),
                nutrient::Nutrient::new("Mo", 0.05),
            ]),
            vec![
                Fertiliser::from(formula.build("Ca(NO3)2").unwrap()).name("calcium nitrate"),
                Fertiliser::from(formula.build("KNO3").unwrap()).name("pottasium nitrate"),
                Fertiliser::from(formula.build("NH4NO3").unwrap()).name("ammonium nitrate"),
                Fertiliser::from(formula.build("MgSO4*7H2O").unwrap()).name("magnesium sulfate"),
                Fertiliser::from(formula.build("K2SO4").unwrap()).name("pottasium sulfate"),
                Fertiliser::from(formula.build("KH2PO4").unwrap()).name("monopottasium phosphate"),
                Fertiliser::from(Label::from(
                    Units::WeightVolume,
                    vec![
                        Nutrient::Magnesium(Some(15000.), None),
                        Nutrient::Iron(Some(3200.)),
                        Nutrient::Manganese(Some(1600.)),
                        Nutrient::Boron(Some(1200.)),
                        Nutrient::Zink(Some(360.)),
                        Nutrient::Cuprum(Some(320.)),
                        Nutrient::Molibden(Some(102.)),
                    ],
                ))
                .name("uniflor micro"),
            ],
        ) {
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
