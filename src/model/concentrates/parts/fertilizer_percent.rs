use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FertilizerPercent {
    fertilizer_id: String,
    percent: usize,
}

impl FertilizerPercent {
    pub fn new(fertilizer_id: String, percent: usize) -> Self {
        Self {
            fertilizer_id,
            percent,
        }
    }

    pub fn id(&self) -> String {
        self.fertilizer_id.clone()
    }

    pub fn percent(&self) -> usize {
        self.percent
    }

    pub fn amount(&self, amount: f64) -> f64 {
        amount * (self.percent / 100) as f64
    }
}
