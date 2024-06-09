use crate::model::chemistry::{NitrogenForm, NutrientAmount};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq)]
pub enum Component {
    Nutrient(NutrientAmount),
    NitrogenForm(NitrogenForm),
}
