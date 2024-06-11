use crate::model::chemistry::NitrogenForm;
use crate::model::fertilizers::{Composition, Fertilizer, SourceType};
use crate::model::formulas::Formula;
use crate::model::labels::{Component, Label, Units};
use uuid::Uuid;

pub struct FertilizerBuilder {
    id: String,
    name: String,
    vendor: String,
    source_type: SourceType,
    label: Label,
    formula: Formula,
}

impl FertilizerBuilder {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            vendor: String::new(),
            source_type: SourceType::Label,
            label: Label::new(Units::Percent),
            formula: Formula::new(""),
        }
    }

    pub fn from(fertilizer: Fertilizer) -> Self {
        Self {
            id: fertilizer.id(),
            name: fertilizer.name(),
            vendor: fertilizer.vendor(),
            source_type: {
                if fertilizer.is_label_based() {
                    SourceType::Label
                } else {
                    SourceType::Formula
                }
            },
            label: {
                if let Composition::Label(label) = fertilizer.composition() {
                    label
                } else {
                    Label::new(Units::Percent)
                }
            },
            formula: {
                if let Composition::Formula(formula) = fertilizer.composition() {
                    formula
                } else {
                    Formula::new("")
                }
            },
        }
    }

    pub fn update_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn update_vendor(&mut self, vendor: String) {
        self.vendor = vendor;
    }

    pub fn update_source_type(&mut self, source_type: SourceType) {
        self.source_type = source_type;
    }

    pub fn update_label_units(&mut self, units: Units) {
        self.label.update_units(units);
    }

    pub fn update_label_component(&mut self, component: Component) {
        self.label.update_component(component);
    }

    pub fn update_label_nitrogen_form(&mut self, nitrogen_form: NitrogenForm) {
        self.label.update_nitrogen_form(nitrogen_form);
    }

    pub fn update_formula(&mut self, formula: String) {
        self.formula = Formula::from(formula);
    }

    pub fn source_type(&self) -> SourceType {
        self.source_type
    }

    pub fn label(&self) -> Label {
        self.label.clone()
    }

    pub fn formula(&self) -> Formula {
        self.formula.clone()
    }

    pub fn build(&self) -> Fertilizer {
        let mut fertilizer = Fertilizer::build();

        fertilizer.with_id(self.id.clone());

        fertilizer.with_name(self.name.clone());

        fertilizer.with_vendor(self.vendor.clone());

        match self.source_type {
            SourceType::Label => {
                fertilizer.with_label(self.label);
            }
            SourceType::Formula => {
                fertilizer.with_formula(self.formula.clone());
            }
        }

        fertilizer
    }
}
