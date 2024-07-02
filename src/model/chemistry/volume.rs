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
        match self.units {
            VolumeUnits::Litres => self.value as f64,
            VolumeUnits::Millilitres => (self.value / 1000) as f64,
            VolumeUnits::Gallons => self.value as f64 / 0.264172,
        }
    }

    pub fn units(&self) -> VolumeUnits {
        self.units
    }

    pub fn convert(&self, units: VolumeUnits) -> Self {
        let value = match self.units {
            VolumeUnits::Litres => match units {
                VolumeUnits::Litres => self.value(),
                VolumeUnits::Millilitres => self.value() * 1000,
                VolumeUnits::Gallons => (self.value() as f64 * 0.264172).ceil() as usize,
            },
            VolumeUnits::Millilitres => match units {
                VolumeUnits::Litres => self.value() / 1000,
                VolumeUnits::Millilitres => self.value(),
                VolumeUnits::Gallons => ((self.value() / 1000) as f64 * 0.264172).ceil() as usize,
            },
            VolumeUnits::Gallons => match units {
                VolumeUnits::Litres => (self.value() as f64 / 0.264172).ceil() as usize,
                VolumeUnits::Millilitres => (self.value() as f64 / 0.264172).ceil() as usize * 1000,
                VolumeUnits::Gallons => self.value(),
            },
        };

        Self::new(value, units)
    }
}

impl Default for Volume {
    fn default() -> Self {
        Self::new(1, VolumeUnits::Litres)
    }
}
