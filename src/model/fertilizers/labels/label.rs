use crate::model::fertilizers::labels::{Component, Units};
use serde::{Deserialize, Serialize};
use std::ops::Index;

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Label {
    units: Units,
    components: [Component; 20],
}

impl Label {
    pub fn new(units: Units) -> Self {
        Self {
            units,
            components: [
                Component::Nitrogen(0.0),
                Component::NitrogenNitrate(0.0),
                Component::NitrogenAmmonium(0.0),
                Component::Phosphor(0.0),
                Component::PhosphorPentoxide(0.0),
                Component::Potassium(0.0),
                Component::PotassiumOxide(0.0),
                Component::Calcium(0.0),
                Component::CalciumOxide(0.0),
                Component::Magnesium(0.0),
                Component::MagnesiumOxide(0.0),
                Component::Sulfur(0.0),
                Component::SulfurTrioxide(0.0),
                Component::SulfurTetroxide(0.0),
                Component::Iron(0.0),
                Component::Manganese(0.0),
                Component::Copper(0.0),
                Component::Zinc(0.0),
                Component::Boron(0.0),
                Component::Molybdenum(0.0),
            ],
        }
    }

    pub fn components(&self) -> Vec<Component> {
        let mut components: Vec<Component> = vec![];

        for component in &self.components {
            components.push(self.apply_units(*component));
        }

        components
    }

    pub fn units(&self) -> Units {
        self.units
    }

    pub fn update_units(&mut self, units: Units) {
        self.units = units;
    }

    pub fn update_component(&mut self, component: Component) {
        self.components[component.index()] = component;

        match component {
            Component::Nitrogen(value) => {
                let nitrate_value =
                    value - self.components[Component::NitrogenAmmonium(0.0).index()].value();

                self.components[Component::NitrogenNitrate(0.0).index()] =
                    Component::NitrogenNitrate(nitrate_value);
            }

            Component::NitrogenNitrate(value) => {
                let ammonium_value =
                    self.components[Component::Nitrogen(0.0).index()].value() - value;

                self.components[Component::NitrogenAmmonium(0.0).index()] =
                    Component::NitrogenAmmonium(ammonium_value);
            }

            Component::NitrogenAmmonium(value) => {
                let nitrogen_value =
                    self.components[Component::Nitrogen(0.0).index()].value() - value;

                self.components[Component::NitrogenNitrate(0.0).index()] =
                    Component::NitrogenNitrate(nitrogen_value);
            }

            _ => {}
        }
    }

    fn apply_units(&self, component: Component) -> Component {
        if self.units == Units::WeightVolume {
            component.new(component.value() / 10000.)
        } else {
            component
        }
    }
}

impl Index<Component> for Label {
    type Output = Component;

    fn index(&self, component: Component) -> &Self::Output {
        &self.components[component.index()]
    }
}

impl Default for Label {
    fn default() -> Self {
        Self::new(Units::Percent)
    }
}
