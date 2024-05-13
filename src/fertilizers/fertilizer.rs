use super::labels::label::Label;
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

    pub fn from_label(name: &str, vendor: &str, label: Label) -> Self {
        let mut nutrients: Vec<Nutrient> = Vec::new();

        label.components().iter().for_each(|component| {
            nutrients.push(Nutrient {
                element: component.element.clone(),
                percent: component.percent,
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

#[cfg(test)]
mod tests {
    use crate::fertilizers::labels::label::Label;
    use crate::fertilizers::labels::nutrient::Nutrient;
    use crate::fertilizers::labels::units::Units;
    use crate::formula::builder::Builder;

    use super::Fertiliser;

    #[test]
    fn from_label() {
        let mut label = Label::new(Units::WeightVolume);

        label.add_nutrient(Nutrient::Magnesium(Some(15000.), None));
        label.add_nutrient(Nutrient::Iron(Some(3200.)));
        label.add_nutrient(Nutrient::Manganese(Some(1600.)));
        label.add_nutrient(Nutrient::Boron(Some(1200.)));
        label.add_nutrient(Nutrient::Zink(Some(360.)));
        label.add_nutrient(Nutrient::Cuprum(Some(320.)));
        label.add_nutrient(Nutrient::Molibden(Some(102.)));

        let fertilizer = Fertiliser::from_label("uniflor micro", "", label);

        println!("{:#?}", fertilizer);
    }

    #[test]
    fn from_formula() {
        let formula = Builder::new().build("MgSO4*7H2O").unwrap();

        let fertilizer = Fertiliser::from_formula("magnesium sulfate", "", formula);

        println!("{:#?}", fertilizer);
    }
}
