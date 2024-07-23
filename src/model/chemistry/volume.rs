use crate::model::chemistry::VolumeUnits;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Volume {
    value: f64,
    units: VolumeUnits,
}

impl Volume {
    pub fn new(value: f64, units: VolumeUnits) -> Self {
        Self { value, units }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn to_litres(&self) -> f64 {
        match self.units {
            VolumeUnits::Litres => self.value,
            VolumeUnits::Gallons => self.value * 3.7854,
        }
    }

    pub fn units(&self) -> VolumeUnits {
        self.units
    }

    pub fn convert(&self, units: VolumeUnits) -> Self {
        let value = match self.units {
            VolumeUnits::Litres => match units {
                VolumeUnits::Litres => self.value(),
                VolumeUnits::Gallons => self.value() * 0.2642,
            },
            VolumeUnits::Gallons => match units {
                VolumeUnits::Litres => self.value() * 3.7854,
                VolumeUnits::Gallons => self.value(),
            },
        };

        Self::new(value, units)
    }
}

impl Default for Volume {
    fn default() -> Self {
        Self::new(1.0, VolumeUnits::Litres)
    }
}
