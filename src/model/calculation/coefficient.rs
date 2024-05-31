use crate::model::chemistry::{NitrogenForm, NutrientAmount};
use ellp::problem::VariableId;

pub enum Coefficient {
    NitrogenForm(NitrogenForm, VariableId),
    NutrientAmount(NutrientAmount, VariableId),
}

impl Coefficient {
    pub fn value(&self) -> (VariableId, f64) {
        match self {
            Self::NitrogenForm(nitrogen_form, variable_id) => (*variable_id, nitrogen_form.value()),
            Self::NutrientAmount(nutrient_amount, variable_id) => {
                (*variable_id, nutrient_amount.value())
            }
        }
    }
}
