use super::{Component, Units};
use serde::{Deserialize, Serialize};

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

    pub fn from(units: Units, components: Vec<Component>) -> Self {
        let mut label = Self::new(units);

        components.iter().for_each(|component| {
            label.update_component(*component);
        });

        println!("label {:#?}", label);

        label
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

    pub fn nitrogen(&self) -> Component {
        self.components[0]
        // Component::Nitrogen(self.nitrogen)
    }

    pub fn nitrogen_nitrate(&self) -> Component {
        self.components[1]
        // Component::NitrogenNitrate(self.nitrogen_nitrate)
    }

    pub fn nitrogen_ammonium(&self) -> Component {
        self.components[2]
        // Component::NitrogenAmmonium(self.nitrogen_ammonium)
    }

    pub fn phosphor(&self) -> Component {
        self.components[3]
        // Component::Phosphor(self.phosphor)
    }

    pub fn phosphor_pentoxide(&self) -> Component {
        self.components[4]
        // Component::PhosphorPentoxide(self.phosphor_pentoxide)
    }

    pub fn potassium(&self) -> Component {
        self.components[5]
        // Component::Potassium(self.potassium)
    }

    pub fn potassium_oxide(&self) -> Component {
        self.components[6]
        // Component::PotassiumOxide(self.potassium_oxide)
    }

    pub fn calcium(&self) -> Component {
        self.components[7]
        // Component::Calcium(self.calcium)
    }

    pub fn calcium_oxide(&self) -> Component {
        self.components[8]
        // Component::CalciumOxide(self.calcium_oxide)
    }

    pub fn magnesium(&self) -> Component {
        self.components[9]
        // Component::Magnesium(self.magnesium)
    }

    pub fn magnesium_oxide(&self) -> Component {
        self.components[10]
        // Component::MagnesiumOxide(self.magnesium_oxide)
    }

    pub fn sulfur(&self) -> Component {
        self.components[11]
        // Component::Sulfur(self.sulfur)
    }

    pub fn sulfur_trioxide(&self) -> Component {
        self.components[12]
        // Component::SulfurTrioxide(self.sulfur_trioxide)
    }

    pub fn sulfur_tetroxide(&self) -> Component {
        self.components[13]
        // Component::SulfurTetroxide(self.sulfur_tetroxide)
    }

    pub fn iron(&self) -> Component {
        self.components[14]
        // Component::Iron(self.iron)
    }

    pub fn manganese(&self) -> Component {
        self.components[15]
        // Component::Manganese(self.manganese)
    }

    pub fn copper(&self) -> Component {
        self.components[16]
        // Component::Copper(self.copper)
    }

    pub fn zinc(&self) -> Component {
        self.components[17]
        // Component::Zinc(self.zinc)
    }

    pub fn boron(&self) -> Component {
        self.components[18]
        // Component::Boron(self.boron)
    }

    pub fn molybdenum(&self) -> Component {
        self.components[19]
        // Component::Molybdenum(self.molybdenum)
    }

    pub fn update_units(&mut self, units: Units) {
        self.units = units;
    }

    pub fn update_component(&mut self, component: Component) {
        match component {
            Component::Nitrogen(value) => {
                self.components[component.index()] = component;

                self.components[self.nitrogen_nitrate().index()] =
                    Component::NitrogenNitrate(value - self.nitrogen_ammonium().value());
            }

            Component::NitrogenNitrate(value) => {
                self.components[component.index()] = component;

                self.components[self.nitrogen().index()] =
                    Component::Nitrogen(value + self.nitrogen_ammonium().value());
            }

            Component::NitrogenAmmonium(value) => {
                self.components[component.index()] = component;

                self.components[self.nitrogen().index()] =
                    Component::Nitrogen(value + self.nitrogen_nitrate().value());
            }

            _ => {
                self.components[component.index()] = component;
            }
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
