use super::{Contents, Formula, Label};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Composition {
    Label(Label),
    Formula(Formula),
}

impl Into<Contents> for Composition {
    fn into(self) -> Contents {
        match self {
            Composition::Label(label) => {
                let mut contents = Contents::new();

                label.components().iter().for_each(|component| {
                    contents.add_nutrient_content(component.nutrient_content());
                });

                contents
            }

            Composition::Formula(formula) => {
                let mut contents = Contents::new();

                formula.components().iter().for_each(|component| {
                    contents.add_nutrient_content(component.nutrient_content());
                });

                contents
            }
        }
    }
}
