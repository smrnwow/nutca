use super::{Error, FertilizerWeight, NutrientRequirement, Profile, ResultProfile};
use crate::model::fertilizers::{Fertilizer, Nutrient};
use ellp::{problem::VariableId, Bound, ConstraintOp, DualSimplexSolver, Problem, SolverResult};
use std::collections::HashMap;

pub struct Calculation {
    desired_profile: Profile,
    fertilizers: Vec<Fertilizer>,
    problem: Problem,
    coefficients: HashMap<Nutrient, Vec<(VariableId, f64)>>,
}

impl Calculation {
    pub fn new(desired_profile: Profile, fertilizers: Vec<Fertilizer>) -> Result<Self, Error> {
        let mut calculation = Self {
            problem: Problem::new(),
            desired_profile,
            fertilizers: Vec::new(),
            coefficients: HashMap::from([
                (Nutrient::Nitrogen, vec![]),
                (Nutrient::NitrogenNitrate, vec![]),
                (Nutrient::NitrogenAmmonium, vec![]),
                (Nutrient::Phosphor, vec![]),
                (Nutrient::Potassium, vec![]),
                (Nutrient::Calcium, vec![]),
                (Nutrient::Magnesium, vec![]),
                (Nutrient::Sulfur, vec![]),
                (Nutrient::Iron, vec![]),
                (Nutrient::Manganese, vec![]),
                (Nutrient::Copper, vec![]),
                (Nutrient::Zinc, vec![]),
                (Nutrient::Boron, vec![]),
                (Nutrient::Molybdenum, vec![]),
            ]),
        };

        fertilizers.iter().for_each(|fertilizer| {
            calculation.add_fertilizer(fertilizer);
        });

        desired_profile
            .requirements()
            .iter()
            .for_each(|requirement| {
                calculation.add_requirement(*requirement);
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

        fertilizer
            .nutrient_contents()
            .nutrients()
            .iter()
            .for_each(|nutrient| {
                if let Some(coefficients) = self.coefficients.get_mut(&nutrient.nutrient()) {
                    coefficients.push((variable_id, nutrient.value()));
                }
            });
    }

    fn add_requirement(&mut self, requirement: NutrientRequirement) {
        let coefficients = self
            .coefficients
            .get(&requirement.nutrient())
            .unwrap()
            .clone();

        match requirement.nutrient() {
            Nutrient::Nitrogen
            | Nutrient::Phosphor
            | Nutrient::Potassium
            | Nutrient::Magnesium
            | Nutrient::Calcium
            | Nutrient::Boron => {
                self.problem
                    .add_constraint(coefficients, ConstraintOp::Eq, requirement.amount())
                    .unwrap();
            }
            Nutrient::Sulfur
            | Nutrient::Iron
            | Nutrient::Manganese
            | Nutrient::Zinc
            | Nutrient::Copper
            | Nutrient::Molybdenum => {
                self.problem
                    .add_constraint(coefficients, ConstraintOp::Gte, 0.0)
                    .unwrap();
            }
            _ => {}
        };
    }

    pub fn solve(&self, tank_size: i32) -> Result<ResultProfile, Error> {
        println!("{}", self.problem);

        let result = DualSimplexSolver::default()
            .solve(self.problem.clone())
            .unwrap();

        if let SolverResult::Optimal(sol) = result {
            let result = sol.x();

            let mut result_profile = ResultProfile::new();

            result.iter().enumerate().for_each(|(idx, amount)| {
                if let Some(fertilizer) = self.fertilizers.get(idx) {
                    let fertilizer_weight = FertilizerWeight::new(fertilizer.clone(), *amount);

                    result_profile.add_fertilizer_weight(fertilizer_weight);
                }
            });

            Ok(result_profile)
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
