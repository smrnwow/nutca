use crate::model::fertilizers::Fertilizer;

#[derive(Clone, Debug)]
pub struct Amount {
    fertilizer: Fertilizer,
    fertilizer_index: usize,
    amount: f64,
}

impl Amount {
    pub fn new(fertilizer: Fertilizer, fertilizer_index: usize, amount: f64) -> Self {
        Self {
            fertilizer,
            fertilizer_index,
            amount,
        }
    }

    pub fn fertilizer_index(&self) -> usize {
        self.fertilizer_index
    }

    pub fn amount(&self) -> f64 {
        format!("{:.6}", self.amount).parse().unwrap()
    }

    pub fn fertilizer(&self) -> &Fertilizer {
        &self.fertilizer
    }
}
