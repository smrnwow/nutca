use crate::model::chemistry::Nutrient;
use crate::model::fertilizers::{Source, SourceType};
use crate::model::{
    formulas::Formula,
    labels::{Label, Units},
};
use serde::{Deserialize, Serialize};
use std::ops::Index;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Fertilizer {
    id: String,
    name: String,
    vendor: String,
    source: Source,
    liquid: bool,
    nutrients: [Nutrient; 14],
}

impl Fertilizer {
    pub fn build() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            vendor: String::new(),
            source: Source::Label(Label::new(Units::Percent)),
            liquid: false,
            nutrients: [
                Nutrient::Nitrogen(0.0),
                Nutrient::NitrogenNitrate(0.0),
                Nutrient::NitrogenAmmonium(0.0),
                Nutrient::Phosphorus(0.0),
                Nutrient::Potassium(0.0),
                Nutrient::Calcium(0.0),
                Nutrient::Magnesium(0.0),
                Nutrient::Sulfur(0.0),
                Nutrient::Iron(0.0),
                Nutrient::Manganese(0.0),
                Nutrient::Copper(0.0),
                Nutrient::Zinc(0.0),
                Nutrient::Boron(0.0),
                Nutrient::Molybdenum(0.0),
            ],
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
            self.add_nutrient(component.nutrient());
        });

        self.source = Source::Label(label);

        self
    }

    pub fn with_formula(mut self, formula: Formula) -> Self {
        formula.nutrients().iter().for_each(|nutrient_amount| {
            self.add_nutrient(*nutrient_amount);
        });

        self.source = Source::Formula(formula);

        self
    }

    pub fn with_liquid(mut self, liquid: bool) -> Self {
        self.liquid = liquid;

        self
    }

    fn add_nutrient(&mut self, nutrient: Nutrient) {
        self.nutrients[nutrient.index()] = self.nutrients[nutrient.index()].add(nutrient.value());
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn vendor(&self) -> String {
        if self.vendor.len() > 0 {
            self.vendor.clone()
        } else {
            String::from("Не указан")
        }
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

    pub fn nutrients(&self) -> Vec<Nutrient> {
        self.nutrients
            .iter()
            .filter(|nutrient_amount| nutrient_amount.value() > 0.)
            .map(|nutrient_amount| *nutrient_amount)
            .collect()
    }

    pub fn is_complex(&self) -> bool {
        self.nutrients().len() > 3
    }
}

impl Index<Nutrient> for Fertilizer {
    type Output = Nutrient;

    fn index(&self, nutrient: Nutrient) -> &Self::Output {
        &self.nutrients[nutrient.index()]
    }
}
