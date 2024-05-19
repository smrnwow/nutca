use crate::chemistry::Nutrient;
use ellp::problem::VariableId;

#[derive(Clone)]
pub struct Requirement {
    symbol: Nutrient,
    amount: f32,
    coefficients: Vec<(VariableId, f64)>,
}

impl Requirement {
    pub fn new(symbol: Nutrient, amount: f32) -> Self {
        Self {
            symbol,
            amount,
            coefficients: Vec::new(),
        }
    }

    pub fn add_coefficient(&mut self, coefficient: (VariableId, f64)) {
        self.coefficients.push(coefficient);
    }

    pub fn symbol(&self) -> Nutrient {
        self.symbol
    }

    pub fn amount(&self) -> f64 {
        self.amount.into()
    }

    pub fn coefficients(&self) -> Vec<(VariableId, f64)> {
        self.coefficients.clone()
    }
}
