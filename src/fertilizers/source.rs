use crate::fertilizers::formulas::Formula;
use crate::fertilizers::labels::Label;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Source {
    Label(Label),
    Formula(Formula),
}
