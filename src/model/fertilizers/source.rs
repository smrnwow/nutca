use crate::model::formulas::Formula;
use crate::model::labels::Label;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Source {
    Label(Label),
    Formula(Formula),
}
