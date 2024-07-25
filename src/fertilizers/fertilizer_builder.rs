use crate::chemistry::Nutrients;
use crate::fertilizers::labels::{Component, Label, Units};
use crate::fertilizers::{Fertilizer, Formula, Source, SourceType};
use crate::Error;
use uuid::Uuid;

pub struct FertilizerBuilder {
    id: String,
    name: String,
    vendor: String,
    source_type: SourceType,
    liquid: bool,
    label: Label,
    formula: Formula,
}

impl FertilizerBuilder {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            vendor: String::new(),
            source_type: SourceType::default(),
            liquid: false,
            label: Label::default(),
            formula: Formula::default(),
        }
    }

    pub fn name(&mut self, name: impl ToString) -> &mut Self {
        self.name = name.to_string();
        self
    }

    pub fn vendor(&mut self, vendor: impl ToString) -> &mut Self {
        self.vendor = vendor.to_string();
        self
    }

    pub fn source_type(&mut self, source_type: SourceType) -> &mut Self {
        self.source_type = source_type;
        self
    }

    pub fn label_units(&mut self, units: Units) -> &mut Self {
        self.label.update_units(units);

        if let Units::WeightVolume = units {
            self.liquid = true;
        }

        self.source_type = SourceType::Label;

        self
    }

    pub fn label_component(&mut self, component: Component) -> &mut Self {
        self.label.update_component(component);

        self.source_type = SourceType::Label;

        self
    }

    pub fn formula(&mut self, formula: impl ToString) -> &mut Self {
        self.formula = Formula::from(formula.to_string());

        self.source_type = SourceType::Formula;

        self
    }

    pub fn liquid(&mut self, liquid: bool) -> &mut Self {
        self.liquid = liquid;

        self
    }

    pub fn validate(&self) -> Vec<Error> {
        let mut errors = Vec::new();

        if self.name.len() == 0 {
            errors.push(Error::FertilizerNameEmpty);
        }

        if self.name.len() > 100 {
            errors.push(Error::FertilizerNameTooLong);
        }

        errors
    }

    pub fn build(&self) -> Fertilizer {
        Fertilizer {
            id: self.id.clone(),
            name: self.name.clone(),
            vendor: self.vendor.clone(),
            liquid: self.liquid,
            source: match self.source_type {
                SourceType::Label => Source::Label(self.label),
                SourceType::Formula => Source::Formula(self.formula.clone()),
            },
            nutrients: match self.source_type {
                SourceType::Label => {
                    let mut nutrients = Nutrients::new();

                    self.label.components().iter().for_each(|component| {
                        nutrients.add(component.nutrient());
                    });

                    nutrients
                }

                SourceType::Formula => {
                    let mut nutrients = Nutrients::new();

                    self.formula
                        .nutrients()
                        .list()
                        .iter()
                        .for_each(|nutrient_amount| {
                            nutrients.add(*nutrient_amount);
                        });

                    nutrients
                }
            },
        }
    }
}

impl From<Fertilizer> for FertilizerBuilder {
    fn from(fertilizer: Fertilizer) -> Self {
        Self {
            id: fertilizer.id(),
            name: fertilizer.name(),
            vendor: fertilizer.vendor(),
            source_type: fertilizer.source_type(),
            liquid: fertilizer.liquid(),
            label: match fertilizer.source() {
                Source::Label(label) => label,
                _ => Label::default(),
            },
            formula: match fertilizer.source() {
                Source::Formula(formula) => formula,
                _ => Formula::default(),
            },
        }
    }
}
