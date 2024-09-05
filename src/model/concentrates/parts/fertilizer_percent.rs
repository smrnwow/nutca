use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct FertilizerPercent {
    fertilizer_id: String,
    weight: f64,
    percent: usize,
}

impl FertilizerPercent {
    pub fn new(fertilizer_id: String, weight: f64, percent: usize) -> Self {
        Self {
            fertilizer_id,
            weight,
            percent,
        }
    }

    pub fn increment_percent(&mut self, percent: usize) {
        self.percent += percent;
    }

    pub fn decrement_percent(&mut self, percent: usize) {
        self.percent -= percent;
    }

    pub fn coefficient(&self, coefficient: f64) -> Self {
        Self {
            fertilizer_id: self.fertilizer_id.clone(),
            weight: self.weight * coefficient,
            percent: self.percent,
        }
    }

    pub fn id(&self) -> String {
        self.fertilizer_id.clone()
    }

    pub fn percent(&self) -> usize {
        self.percent
    }

    pub fn amount(&self) -> f64 {
        format!("{:.3}", self.weight * (self.percent / 100) as f64)
            .parse()
            .unwrap_or(0.0)
    }
}
