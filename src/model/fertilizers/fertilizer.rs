use super::Composition;
use crate::model::{
    chemistry::{NitrogenForm, NutrientAmount},
    formulas::Formula,
    labels::{Label, Units},
};
use serde::{Deserialize, Serialize};
use std::ops::Index;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Fertilizer {
    id: String,
    name: String,
    vendor: String,
    composition: Composition,
    nutrients: [NutrientAmount; 12],
    nitrogen_forms: [NitrogenForm; 2],
}

impl Fertilizer {
    pub fn build() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            vendor: String::new(),
            composition: Composition::Label(Label::new(Units::Percent)),
            nutrients: [
                NutrientAmount::Nitrogen(0.0),
                NutrientAmount::Phosphorus(0.0),
                NutrientAmount::Potassium(0.0),
                NutrientAmount::Calcium(0.0),
                NutrientAmount::Magnesium(0.0),
                NutrientAmount::Sulfur(0.0),
                NutrientAmount::Iron(0.0),
                NutrientAmount::Manganese(0.0),
                NutrientAmount::Copper(0.0),
                NutrientAmount::Zinc(0.0),
                NutrientAmount::Boron(0.0),
                NutrientAmount::Molybdenum(0.0),
            ],
            nitrogen_forms: [NitrogenForm::Nitrate(0.0), NitrogenForm::Ammonium(0.0)],
        }
    }

    pub fn set_name(mut self, name: String) -> Self {
        self.name = name;

        self
    }

    pub fn with_name(&mut self, name: String) -> &mut Self {
        self.name = name;

        self
    }

    pub fn set_vendor(mut self, vendor: String) -> Self {
        self.vendor = vendor;

        self
    }

    pub fn with_vendor(&mut self, vendor: String) -> &mut Self {
        self.vendor = vendor;

        self
    }

    pub fn set_label(mut self, label: Label) -> Self {
        self.composition = Composition::Label(label);

        label.components().iter().for_each(|component| {
            self.add_nutrient_amount(component.nutrient_amount());
        });

        label.nitrogen_forms().iter().for_each(|nitrogen_form| {
            self.add_nitrogen_form(*nitrogen_form);
        });

        self
    }

    pub fn with_label(&mut self, label: Label) -> &mut Self {
        self.composition = Composition::Label(label);

        label.components().iter().for_each(|component| {
            self.add_nutrient_amount(component.nutrient_amount());
        });

        label.nitrogen_forms().iter().for_each(|nitrogen_form| {
            self.add_nitrogen_form(*nitrogen_form);
        });

        self
    }

    pub fn set_formula(mut self, formula: Formula) -> Self {
        self.composition = Composition::Formula(formula.clone());

        formula.nutrients().iter().for_each(|nutrient_amount| {
            self.add_nutrient_amount(*nutrient_amount);
        });

        formula.nitrogen_forms().iter().for_each(|nitrogen_form| {
            self.add_nitrogen_form(*nitrogen_form);
        });

        self
    }

    pub fn with_formula(&mut self, formula: Formula) -> &mut Self {
        self.composition = Composition::Formula(formula.clone());

        formula.nutrients().iter().for_each(|nutrient_amount| {
            self.add_nutrient_amount(*nutrient_amount);
        });

        formula.nitrogen_forms().iter().for_each(|nitrogen_form| {
            self.add_nitrogen_form(*nitrogen_form);
        });

        self
    }

    fn add_nutrient_amount(&mut self, nutrient_amount: NutrientAmount) {
        self.nutrients[nutrient_amount.index()] = nutrient_amount;
    }

    fn add_nitrogen_form(&mut self, nitrogen_form: NitrogenForm) {
        self.nitrogen_forms[nitrogen_form.index()] = nitrogen_form;
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

    pub fn composition(&self) -> Composition {
        self.composition.clone()
    }

    pub fn is_label_based(&self) -> bool {
        match self.composition {
            Composition::Label(_) => true,
            Composition::Formula(_) => false,
        }
    }

    pub fn is_formula_based(&self) -> bool {
        match self.composition {
            Composition::Label(_) => false,
            Composition::Formula(_) => true,
        }
    }

    pub fn nutrients(&self) -> Vec<NutrientAmount> {
        self.nutrients
            .iter()
            .filter(|nutrient_amount| nutrient_amount.value() > 0.)
            .map(|nutrient_amount| *nutrient_amount)
            .collect()
    }

    pub fn nitrogen_forms(&self) -> Vec<NitrogenForm> {
        self.nitrogen_forms
            .iter()
            .filter(|nitrogen_form| nitrogen_form.value() > 0.)
            .map(|nitrogen_form| *nitrogen_form)
            .collect()
    }
}

impl Index<NutrientAmount> for Fertilizer {
    type Output = NutrientAmount;

    fn index(&self, nutrient_amount: NutrientAmount) -> &Self::Output {
        &self.nutrients[nutrient_amount.index()]
    }
}

impl Index<NitrogenForm> for Fertilizer {
    type Output = NitrogenForm;

    fn index(&self, nitrogen_form: NitrogenForm) -> &Self::Output {
        &self.nitrogen_forms[nitrogen_form.index()]
    }
}
