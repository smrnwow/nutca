use serde::{Deserialize, Serialize};

/// An enumeration of measurement units for fertilizer components listed on a label.
///
/// - **Percent**: percentage concentration
/// - **WeightVolume**: weight-to-volume ratio
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum LabelUnits {
    Percent,
    WeightVolume,
}
