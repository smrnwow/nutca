use crate::model::chemistry::Nutrients;
use crate::model::fertilizers::{Source, SourceType};
use crate::model::formulas::Formula;
use crate::model::labels::{Label, Units};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Fertilizer {
    pub nutrients: Nutrients,
    id: String,
    name: String,
    vendor: String,
    source: Source,
    liquid: bool,
}

impl Fertilizer {
    pub fn build() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            vendor: String::new(),
            source: Source::Label(Label::new(Units::Percent)),
            liquid: false,
            nutrients: Nutrients::new(),
        }
    }

    pub fn with_id(mut self, id: String) -> Self {
        self.id = id;

        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;

        self
    }

    pub fn with_vendor(mut self, vendor: String) -> Self {
        self.vendor = vendor;

        self
    }

    pub fn with_label(mut self, label: Label) -> Self {
        label.components().iter().for_each(|component| {
            self.nutrients.add(component.nutrient());
        });

        self.source = Source::Label(label);

        self
    }

    pub fn with_formula(mut self, formula: Formula) -> Self {
        formula.nutrients.list().iter().for_each(|nutrient_amount| {
            self.nutrients.add(*nutrient_amount);
        });

        self.source = Source::Formula(formula);

        self
    }

    pub fn with_liquid(mut self, liquid: bool) -> Self {
        self.liquid = liquid;

        self
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn vendor(&self) -> String {
        self.vendor.clone()
    }

    pub fn source(&self) -> Source {
        self.source.clone()
    }

    pub fn source_type(&self) -> SourceType {
        match self.source {
            Source::Label(_) => SourceType::Label,
            Source::Formula(_) => SourceType::Formula,
        }
    }

    pub fn liquid(&self) -> bool {
        self.liquid
    }
}
