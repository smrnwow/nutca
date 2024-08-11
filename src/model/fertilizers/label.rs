use crate::model::chemistry::Nutrients;
use crate::model::fertilizers::{LabelComponent, LabelUnits};
use serde::{Deserialize, Serialize};
use std::ops::Index;

/// A source composition derived from the label of a commercial fertilizer
#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Label {
    units: LabelUnits,
    components: [LabelComponent; 20],
}

impl Label {
    pub fn new(units: LabelUnits) -> Self {
        Self {
            units,
            components: [
                LabelComponent::Nitrogen(0.0),
                LabelComponent::NitrogenNitrate(0.0),
                LabelComponent::NitrogenAmmonium(0.0),
                LabelComponent::Phosphorus(0.0),
                LabelComponent::PhosphorusPentoxide(0.0),
                LabelComponent::Potassium(0.0),
                LabelComponent::PotassiumOxide(0.0),
                LabelComponent::Calcium(0.0),
                LabelComponent::CalciumOxide(0.0),
                LabelComponent::Magnesium(0.0),
                LabelComponent::MagnesiumOxide(0.0),
                LabelComponent::Sulfur(0.0),
                LabelComponent::SulfurTrioxide(0.0),
                LabelComponent::SulfurTetroxide(0.0),
                LabelComponent::Iron(0.0),
                LabelComponent::Manganese(0.0),
                LabelComponent::Copper(0.0),
                LabelComponent::Zinc(0.0),
                LabelComponent::Boron(0.0),
                LabelComponent::Molybdenum(0.0),
            ],
        }
    }

    pub fn update_component(&mut self, component: LabelComponent) {
        self.components[component.index()] = component;

        match component {
            LabelComponent::Nitrogen(value) => {
                let nitrate_value =
                    value - self.components[LabelComponent::NitrogenAmmonium(0.0).index()].value();

                self.components[LabelComponent::NitrogenNitrate(0.0).index()] =
                    LabelComponent::NitrogenNitrate(nitrate_value);
            }

            LabelComponent::NitrogenNitrate(value) => {
                let ammonium_value =
                    self.components[LabelComponent::Nitrogen(0.0).index()].value() - value;

                self.components[LabelComponent::NitrogenAmmonium(0.0).index()] =
                    LabelComponent::NitrogenAmmonium(ammonium_value);
            }

            LabelComponent::NitrogenAmmonium(value) => {
                let nitrogen_value =
                    self.components[LabelComponent::Nitrogen(0.0).index()].value() - value;

                self.components[LabelComponent::NitrogenNitrate(0.0).index()] =
                    LabelComponent::NitrogenNitrate(nitrogen_value);
            }

            _ => {}
        }
    }

    pub fn update_units(&mut self, units: LabelUnits) {
        self.units = units;
    }

    pub fn nutrients(&self) -> Nutrients {
        let mut nutrients = Nutrients::new();

        self.components.iter().for_each(|component| {
            nutrients.add(self.apply_units(*component).nutrient());
        });

        nutrients
    }

    pub fn units(&self) -> LabelUnits {
        self.units
    }

    fn apply_units(&self, component: LabelComponent) -> LabelComponent {
        if self.units == LabelUnits::WeightVolume {
            component.new(component.value() / 10000.)
        } else {
            component
        }
    }
}

impl Index<LabelComponent> for Label {
    type Output = LabelComponent;

    fn index(&self, component: LabelComponent) -> &Self::Output {
        &self.components[component.index()]
    }
}

impl Default for Label {
    fn default() -> Self {
        Self::new(LabelUnits::Percent)
    }
}
