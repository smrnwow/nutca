use super::labels::label::Label;
use super::nutrient::Nutrient;
use crate::formula::formula::Formula;

#[derive(Clone, Debug)]
pub struct Fertiliser {
    name: String,
    vendor: String,
    nutrients: Vec<Nutrient>,
}

impl Fertiliser {
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn vendor(mut self, vendor: &str) -> Self {
        self.vendor = vendor.to_string();
        self
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

    pub fn nutrients(&self) -> Vec<&Nutrient> {
        self.nutrients
            .iter()
            .filter(|nutrient| nutrient.element.is_nutrient())
            .collect()
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

impl From<Label> for Fertiliser {
    fn from(label: Label) -> Self {
        let mut nutrients: Vec<Nutrient> = Vec::new();

        label.components().iter().for_each(|component| {
            nutrients.push(Nutrient {
                element: component.element.clone(),
                percent: component.percent,
            });
        });

        Self {
            name: String::from(""),
            vendor: String::from(""),
            nutrients,
        }
    }
}

impl From<Formula> for Fertiliser {
    fn from(formula: Formula) -> Self {
        let mut nutrients: Vec<Nutrient> = Vec::new();

        formula.components().iter().for_each(|formula_component| {
            nutrients.push(Nutrient {
                element: formula_component.element().clone(),
                percent: formula_component.mass_percent(),
            });
        });

        Self {
            name: String::from(""),
            vendor: String::from(""),
            nutrients,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fertilizers::labels::label::Label;
    use crate::fertilizers::labels::nutrient::Nutrient;
    use crate::fertilizers::labels::units::Units;
    use crate::formula::builder::Builder;

    use super::Fertiliser;

    #[test]
    fn from_label() {
        let fertilizer = Fertiliser::from(Label::from(
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
        .name("uniflor micro");

        // println!("{:#?}", fertilizer);
    }

    #[test]
    fn from_formula() {
        let formula = Builder::new().build("MgSO4*7H2O").unwrap();

        let fertilizer = Fertiliser::from(formula).name("magnesium sulfate");

        // println!("{:#?}", fertilizer);
    }
}
