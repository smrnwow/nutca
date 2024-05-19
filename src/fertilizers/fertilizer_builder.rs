use super::{Component, Fertiliser, Units};
use crate::chemistry::formulas::{Formula, Tokenizer};
use crate::chemistry::Table;

pub struct FertilizerBuilder {
    table: Table,
}

impl FertilizerBuilder {
    pub fn new() -> Self {
        Self {
            table: Table::new(),
        }
    }

    pub fn from_label(&self, units: Units, components: Vec<Component>) -> Fertiliser {
        let mut fertilizer = Fertiliser::new();

        components.iter().for_each(|component| {
            fertilizer.add_nutrient(component.nutrient_percent(units));
        });

        fertilizer
    }

    pub fn from_formula(&self, formulation: &str) -> Fertiliser {
        let compound = Tokenizer::new(&self.table, formulation).tokenize().unwrap();

        let formula = Formula::from_compound(compound);

        let mut fertilizer = Fertiliser::new();

        formula.nutrients().iter().for_each(|nutrient| {
            fertilizer.add_nutrient(*nutrient);
        });

        fertilizer
    }
}

#[cfg(test)]
mod tests {
    use super::{Component, FertilizerBuilder, Units};

    #[test]
    fn from_weight_volume_label() {
        let fertilizer_builder = FertilizerBuilder::new();

        let fertilizer = fertilizer_builder
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
            .name("uniflor micro");

        println!("from weight volume label {:#?}", fertilizer);
    }

    #[test]
    fn from_percentage_label() {
        let fertilizer_builder = FertilizerBuilder::new();

        let fertilizer = fertilizer_builder
            .from_label(
                Units::Percentage,
                vec![
                    Component::Nitrogen(Some(19.0)),
                    Component::Phosphorus(None, Some(6.0)),
                    Component::Potassium(None, Some(20.0)),
                    Component::Magnesium(None, Some(3.0)),
                    Component::Sulfur(Some(3.0), None, None),
                    Component::Iron(Some(0.07)),
                    Component::Manganese(Some(0.04)),
                    Component::Boron(Some(0.025)),
                    Component::Copper(Some(0.01)),
                    Component::Zink(Some(0.025)),
                    Component::Molybdenum(Some(0.004)),
                ],
            )
            .name("fertika lux");

        println!("from percentage label {:#?}", fertilizer);
    }

    #[test]
    fn from_formula() {
        let fertilizer_builder = FertilizerBuilder::new();

        let fertilizer = fertilizer_builder
            .from_formula("Na2MoO4*2H2O")
            .name("magnesium sulfate");

        println!("from formula {:#?}", fertilizer);
    }
}
