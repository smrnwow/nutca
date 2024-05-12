use super::nutrient::Nutrient;
use crate::formula::formula::Formula;

#[derive(Clone, Debug)]
pub struct Fertiliser {
    name: String,

    pub vendor: String,

    nutrients: Vec<Nutrient>,
}

impl Fertiliser {
    pub fn from_formula(name: &str, vendor: &str, formula: Formula) -> Self {
        let mut nutrients: Vec<Nutrient> = Vec::new();

        formula.components().iter().for_each(|formula_component| {
            nutrients.push(Nutrient {
                element: formula_component.element().clone(),
                percent: formula_component.mass_percent(),
            });
        });

        Self {
            name: String::from(name),
            vendor: String::from(vendor),
            nutrients,
        }
    }

    pub fn percent_of(&self, nutrient_symbol: &str) -> Option<f32> {
        let nutrient = self
            .nutrients
            .iter()
            .find(|nutrient| nutrient.element.symbol == nutrient_symbol);

        match nutrient {
            Some(nutrient) => Some(nutrient.percent),
            None => None,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

/*
    pub fn from_label(label: labels::Label) -> Fertiliser {
        let composition = match label.units {
            labels::MeasurementUnit::WeightVolume => {
                let mut components: Vec<Component> = Vec::new();

                let total_weight: f32 = label
                    .nutrients
                    .values()
                    .map(|nutrient| nutrient.amount)
                    .sum();

                for nutrient in label.nutrients.values() {
                    let comp = Component {
                        element: nutrient.element.clone(),
                        percent: (nutrient.amount / total_weight) * 100.,
                    };

                    components.push(comp.clone());

                    /*
                    println!(
                        "{} has {} percent of {}",
                        label.name, comp.element.symbol, comp.percent
                    );
                    */
                }

                components
            }

            labels::MeasurementUnit::Percentage => {
                let mut components: Vec<Component> = Vec::new();

                for nutrient in label.nutrients.values() {
                    let comp = Component {
                        element: nutrient.element.clone(),
                        percent: nutrient.amount,
                    };

                    components.push(comp.clone());

                    /*
                    println!(
                        "{} has {} percent of {}",
                        label.name, comp.element.symbol, comp.percent
                    );
                    */
                }

                components
            }
        };

        Fertiliser {
            name: label.name.clone(),
            vendor: String::new(),
            composition,
        }
    }

    pub fn from_model(name: String, vendor: String, nutrients: Vec<Component>) -> Fertiliser {
        Fertiliser {
            name,
            vendor,
            composition: nutrients,
        }
    }

    pub fn components(&self) -> Vec<Component> {
        let mut components: Vec<Component> = Vec::new();

        for component in &self.composition {
            if component.element.nutrient {
                components.push(component.clone());
            }
        }

        components
    }

    pub fn contains(&self, nutrient: &String) -> bool {
        for component in &self.composition {
            if component.element.symbol.contains(nutrient) {
                return true;
            }
        }

        false
    }

    pub fn calculate(&self, amount: f32) {
        for component in &self.composition {
            if component.element.nutrient {
                println!(
                    "{} provide {} PPM of {} in {} amount",
                    self.name,
                    ((component.percent / 100.) * amount) * 1000.,
                    component.element.symbol,
                    amount
                );
            }
        }
    }
}
*/

/*
#[cfg(test)]
mod tests {
    use super::{labels, Fertiliser};

    #[test]
    fn from_formula() {
        Fertiliser::from_formula("", "Ca(NO3)2*3H2O");

        // Fertiliser::from_formula("KNO3");
        // Fertiliser::from_formula("MgSO4*7H2O");
    }

    #[test]
    fn from_weight_volume_label() {
        let label = labels::Label::new(
            "Унифлор микро",
            labels::MeasurementUnit::WeightVolume,
            vec![
                ("Mg", 15000.),
                ("Fe", 3200.),
                ("Na", 4800.),
                ("Mn", 1600.),
                ("B", 1200.),
                ("Zn", 360.),
                ("Cu", 320.),
                ("Mo", 102.),
                ("I", 80.),
                ("Co", 48.),
                ("Cr", 22.),
                ("Ni", 29.),
                ("Se", 10.),
                ("Br", 6.),
                ("Al", 0.9),
            ],
        );

        Fertiliser::from_label(label);
    }

    #[test]
    fn from_percentage_label() {
        let label = labels::Label::new(
            "Fertika люкс",
            labels::MeasurementUnit::Percentage,
            vec![
                ("N", 3.2),
                ("P2O5", 4.2),
                ("K2O", 5.4),
                ("Fe", 0.02),
                ("Mn", 0.02),
                ("B", 0.004),
                ("Cu", 0.002),
                ("Zn", 0.002),
                ("Mo", 0.0004),
            ],
        );

        Fertiliser::from_label(label).calculate(7.606);

        Fertiliser::from_formula("", "Ca(NO3)2").calculate(0.696);

        Fertiliser::from_formula("", "MgSO4*7H2O").calculate(0.486);
    }
}
 */
