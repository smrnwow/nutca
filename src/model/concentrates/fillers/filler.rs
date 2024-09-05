use super::{AutoFiller, ManualFiller};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Filler {
    Auto(AutoFiller),
    Manual(ManualFiller),
}
