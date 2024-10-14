use crate::model::chemistry::VolumeUnits;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Volume {
    value: usize,
    units: VolumeUnits,
}

impl Volume {
    pub fn new(value: usize, units: VolumeUnits) -> Self {
        Self { value, units }
    }

    pub fn value(&self) -> usize {
        self.value
    }

    pub fn to_litres(&self) -> f64 {
        let value = self.value as f64;

        match self.units {
            VolumeUnits::Litres => value,
            VolumeUnits::Gallons => value * 3.7854,
        }
    }

    pub fn units(&self) -> VolumeUnits {
        self.units
    }
}

impl Default for Volume {
    fn default() -> Self {
        Self::new(1, VolumeUnits::Litres)
    }
}
