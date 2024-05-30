use crate::model::chemistry::{formulas, Table};
use crate::model::fertilizers::Component;
use crate::model::fertilizers::NutrientContent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Formula {
    formula: String,
    components: Vec<Component>,
}

impl Formula {
    pub fn from(formula: &str) -> Self {
        Self::new(formula.to_string())
    }

    pub fn new(formula: String) -> Self {
        let compound = formulas::Tokenizer::new(&Table::new(), &formula)
            .tokenize()
            .unwrap();

        let formula_result = formulas::Formula::from_compound(compound);

        let mut components = vec![];

        formula_result.nutrients().iter().for_each(|nutrient| {
            let component = match nutrient {
                NutrientContent::Nitrogen(value) => Component::Nitrogen(*value),
                NutrientContent::NitrogenNitrate(value) => Component::NitrogenNitrate(*value),
                NutrientContent::NitrogenAmmonium(value) => Component::NitrogenAmmonium(*value),
                NutrientContent::Phosphor(value) => Component::Phosphor(*value),
                NutrientContent::Potassium(value) => Component::Potassium(*value),
                NutrientContent::Calcium(value) => Component::Calcium(*value),
                NutrientContent::Magnesium(value) => Component::Magnesium(*value),
                NutrientContent::Sulfur(value) => Component::Sulfur(*value),
                NutrientContent::Iron(value) => Component::Iron(*value),
                NutrientContent::Manganese(value) => Component::Manganese(*value),
                NutrientContent::Copper(value) => Component::Copper(*value),
                NutrientContent::Zinc(value) => Component::Zinc(*value),
                NutrientContent::Boron(value) => Component::Boron(*value),
                NutrientContent::Molybdenum(value) => Component::Molybdenum(*value),
            };

            components.push(component);
        });

        Self {
            formula,
            components,
        }
    }

    pub fn components(&self) -> Vec<Component> {
        self.components.clone()
    }

    pub fn value(&self) -> String {
        self.formula.clone()
    }
}
