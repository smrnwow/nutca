use crate::fertilizers::labels::Label;
use crate::fertilizers::Formula;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Source {
    Label(Label),
    Formula(Formula),
}

impl Default for Source {
    fn default() -> Self {
        Self::Label(Label::default())
    }
}
