use crate::model::chemistry::{Nutrient, NutrientAmount, Nutrients};

pub struct Conductivity {
    solution_composition: Nutrients,
    ionic_strength: f64,
}

impl Conductivity {
    pub fn new(solution_composition: Nutrients) -> Self {
        let ionic_strength = Self::ionic_strength(&solution_composition);

        Self {
            solution_composition,
            ionic_strength,
        }
    }

    pub fn conductivity(&self) -> f64 {
        self.solution_composition
            .list()
            .iter()
            .map(|nutrient_amount| {
                let nutrient = nutrient_amount.nutrient();

                let activity_coefficient = self.activity_coefficient(nutrient);

                /*
                println!(
                    "{:#?} activity coefficient {:#?}",
                    nutrient, activity_coefficient
                );
                */

                let dissolve_coefficient = self.dissolve_coefficient(nutrient);

                /*
                println!(
                    "{:#?} dissolve coefficient {:#?}",
                    nutrient, dissolve_coefficient
                );
                */

                let ion_concentration = Self::ppms_to_moles(*nutrient_amount);

                nutrient.limiting_molar_condictivity()
                    * activity_coefficient.powf(dissolve_coefficient)
                    * ion_concentration
            })
            .sum()
    }

    fn ionic_strength(solution_composition: &Nutrients) -> f64 {
        0.5 * solution_composition
            .list()
            .iter()
            .map(|nutrient_amount| {
                let charge = nutrient_amount.nutrient().ionic_charge().abs() as f64;

                let molarity = Self::ppms_to_moles(*nutrient_amount);

                molarity * (charge * charge)
            })
            .sum::<f64>()
    }

    fn a_constant(&self) -> f64 {
        let molarity = self
            .solution_composition
            .list()
            .iter()
            .map(|nutrient_amount| Self::ppms_to_moles(*nutrient_amount))
            .sum::<f64>();

        0.5085 * molarity.powf(-0.5)
    }

    fn activity_coefficient(&self, nutrient: Nutrient) -> f64 {
        let charge = nutrient.ionic_charge().abs() as f64;

        let coefficient = -((10. as f64).ln())
            * self.a_constant()
            * (charge * charge)
            * self.ionic_strength.sqrt();

        /*
        println!(
            "coefficient {:#?}, {:#?}, {:#?}, {:#?}, {:#?} = {:#?}",
            nutrient,
            -((10. as f64).ln()),
            self.a_constant,
            (charge * charge),
            self.ionic_strength.sqrt(),
            coefficient
        );
        */

        coefficient.exp()
    }

    fn dissolve_coefficient(&self, nutrient: Nutrient) -> f64 {
        let charge = nutrient.ionic_charge().abs() as f64;

        if self.ionic_strength <= 0.36 * charge {
            return 0.6 / charge.powf(0.5);
        }

        self.ionic_strength.sqrt() / charge
    }

    fn ppms_to_moles(nutrient_amount: NutrientAmount) -> f64 {
        nutrient_amount.value() / (1000. * nutrient_amount.nutrient().molar_mass())
    }
}

#[cfg(test)]
mod tests {
    use super::{Conductivity, NutrientAmount, Nutrients};

    #[test]
    fn solution_conductivity() {
        let mut composition = Nutrients::new();

        composition.add(NutrientAmount::NitrogenNitrate(210.));
        composition.add(NutrientAmount::Phosphorus(60.));
        composition.add(NutrientAmount::Potassium(350.));
        composition.add(NutrientAmount::Calcium(180.));
        composition.add(NutrientAmount::Magnesium(50.));
        composition.add(NutrientAmount::Sulfur(71.));
        composition.add(NutrientAmount::Iron(2.0));

        let conductivity = Conductivity::new(composition);

        println!(
            "ionic strength: {:#?}, conductivity: {:#?}",
            conductivity.ionic_strength,
            conductivity.conductivity()
        );
    }
}
