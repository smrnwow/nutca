use crate::model::fertilizers::{Fertilizer, FertilizerError, Source, SourceType};
use crate::model::formulas::Formula;
use crate::model::labels::{Component, Label, Units};
use uuid::Uuid;

pub struct FertilizerBuilder {
    id: String,
    name: String,
    vendor: String,
    source_type: SourceType,
    liquid: bool,
    label: Label,
    formula: Formula,
    saved: bool,
}

impl FertilizerBuilder {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            vendor: String::new(),
            source_type: SourceType::Label,
            liquid: false,
            label: Label::new(Units::Percent),
            formula: Formula::new(""),
            saved: false,
        }
    }

    pub fn from(fertilizer: Fertilizer) -> Self {
        Self {
            id: fertilizer.id(),
            name: fertilizer.name(),
            vendor: fertilizer.vendor(),
            source_type: fertilizer.source_type(),
            liquid: fertilizer.liquid(),
            label: {
                if let Source::Label(label) = fertilizer.source() {
                    label
                } else {
                    Label::new(Units::Percent)
                }
            },
            formula: {
                if let Source::Formula(formula) = fertilizer.source() {
                    formula
                } else {
                    Formula::new("")
                }
            },
            saved: false,
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

        if let Units::WeightVolume = units {
            self.liquid = true;
        }
    }

    pub fn update_label_component(&mut self, component: Component) {
        self.label.update_component(component);
    }

    pub fn update_formula(&mut self, formula: String) {
        self.formula = Formula::from(formula);
    }

    pub fn update_liquid(&mut self, liquid: bool) {
        self.liquid = liquid;
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

    pub fn save(&mut self) {
        self.saved = true;
    }

    pub fn validate(&self) -> FertilizerError {
        let mut fertilizer_error = FertilizerError::new();

        if self.saved {
            if self.name.len() == 0 {
                fertilizer_error.set_name("не заполнено");
            }
        }

        fertilizer_error
    }

    pub fn build(&self) -> Fertilizer {
        let fertilizer = Fertilizer::build()
            .with_id(self.id.clone())
            .with_name(self.name.clone())
            .with_vendor(self.vendor.clone())
            .with_liquid(self.liquid);

        match self.source_type {
            SourceType::Label => fertilizer.with_label(self.label),
            SourceType::Formula => fertilizer.with_formula(self.formula.clone()),
        }
    }
}
