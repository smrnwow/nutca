use super::{Component, Units};
use crate::model::chemistry::NitrogenForm;
use serde::{Deserialize, Serialize};
use std::ops::Index;

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Label {
    units: Units,
    components: [Component; 18],
    nitrogen_forms: [NitrogenForm; 2],
}

impl Label {
    pub fn new(units: Units) -> Self {
        Self {
            units,
            components: [
                Component::Nitrogen(0.0),
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
            nitrogen_forms: [NitrogenForm::Nitrate(0.0), NitrogenForm::Ammonium(0.0)],
        }
    }

    pub fn from(units: Units, components: Vec<Component>) -> Self {
        let mut label = Self::new(units);

        components.iter().for_each(|component| {
            label.update_component(*component);
        });

        label
    }

    pub fn components(&self) -> Vec<Component> {
        let mut components: Vec<Component> = vec![];

        for component in &self.components {
            components.push(self.apply_units(*component));
        }

        println!("components {:#?}", components);

        components
    }

    pub fn nitrogen_forms(&self) -> Vec<NitrogenForm> {
        Vec::from(self.nitrogen_forms)
    }

    pub fn units(&self) -> Units {
        self.units
    }

    pub fn update_units(&mut self, units: Units) {
        self.units = units;
    }

    pub fn update_nitrogen_form(&mut self, nitrogen_form: NitrogenForm) {
        match nitrogen_form {
            NitrogenForm::Nitrate(value) => {
                self.nitrogen_forms[nitrogen_form.index()] = nitrogen_form;

                let ammonium_value =
                    self.components[Component::Nitrogen(0.0).index()].value() - value;

                self.nitrogen_forms[NitrogenForm::Ammonium(0.0).index()] =
                    NitrogenForm::Ammonium(ammonium_value);
            }

            NitrogenForm::Ammonium(value) => {
                self.nitrogen_forms[nitrogen_form.index()] = nitrogen_form;

                let nitrogen_value =
                    self.components[Component::Nitrogen(0.0).index()].value() - value;

                self.nitrogen_forms[NitrogenForm::Nitrate(0.0).index()] =
                    NitrogenForm::Nitrate(nitrogen_value);
            }
        }
    }

    pub fn update_component(&mut self, component: Component) {
        self.components[component.index()] = component;

        if let Component::Nitrogen(value) = component {
            let nitrate_value =
                value - self.nitrogen_forms[NitrogenForm::Ammonium(0.0).index()].value();

            self.nitrogen_forms[NitrogenForm::Nitrate(0.0).index()] =
                NitrogenForm::Nitrate(nitrate_value);
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

impl Index<NitrogenForm> for Label {
    type Output = NitrogenForm;

    fn index(&self, nitrogen_form: NitrogenForm) -> &Self::Output {
        &self.nitrogen_forms[nitrogen_form.index()]
    }
}
