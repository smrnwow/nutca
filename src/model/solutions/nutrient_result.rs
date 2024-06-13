#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NutrientResult {
    amount: f64,
    diff_percentage: f64,
}

impl NutrientResult {
    pub fn new(desired_amount: f64, result_amount: f64) -> Self {
        Self {
            amount: result_amount,
            diff_percentage: Self::diff_percentage(desired_amount, result_amount),
        }
    }

    fn diff_percentage(desired: f64, result: f64) -> f64 {
        let diff = (result - desired).abs();

        (diff / desired) * 100.
    }

    pub fn diff(&self) -> f64 {
        self.diff_percentage
    }
}
