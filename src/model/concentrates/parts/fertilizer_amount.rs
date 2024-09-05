use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FertilizerAmount {
    fertilizer_id: String,
    amount: f64,
}

impl FertilizerAmount {
    pub fn new(fertilizer_id: String, amount: f64) -> Self {
        Self {
            fertilizer_id,
            amount,
        }
    }

    pub fn id(&self) -> String {
        self.fertilizer_id.clone()
    }

    pub fn amount(&self) -> f64 {
        format!("{:.3}", self.amount).parse().unwrap_or(0.0)
    }
}
