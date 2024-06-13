use crate::model::chemistry::Nutrient;
use ellp::problem::VariableId;

pub enum Coefficient {
    Nutrient(Nutrient, VariableId),
}

impl Coefficient {
    pub fn value(&self) -> (VariableId, f64) {
        match self {
            Self::Nutrient(nutrient_amount, variable_id) => (*variable_id, nutrient_amount.value()),
        }
    }
}
