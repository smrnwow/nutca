use ellp::problem::VariableId;

#[derive(Clone)]
pub struct Nutrient<'a> {
    symbol: &'a str,
    amount: f32,
    coefficients: Vec<(VariableId, f64)>,
}

impl<'a> Nutrient<'a> {
    pub fn new(symbol: &'a str, amount: f32) -> Self {
        Self {
            symbol,
            amount,
            coefficients: Vec::new(),
        }
    }

    pub fn add_coefficient(&mut self, coefficient: (VariableId, f64)) {
        self.coefficients.push(coefficient);
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn amount(&self) -> f64 {
        self.amount.into()
    }

    pub fn coefficients(&self) -> Vec<(VariableId, f64)> {
        self.coefficients.clone()
    }
}

/*
pub enum Nutrient {
    N,
    NO3,
    NH4,
    P,
    P2O5,
    K,
    K2O,
    Ca,
    CaO,
    Mg,
    MgO,
    S,
    SO2,
    SO3,
    SO4,
    Fe,
    Na,
    Mn,
    B,
    Zn,
    Cu,
    Mo,
    Co,
    I,
    Cr,
    Ni,
    Se,
    Br,
    Al,
}
 */
