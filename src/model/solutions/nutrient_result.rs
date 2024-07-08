#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NutrientResult {
    desired_amount: f64,
    result_amount: f64,
}

impl NutrientResult {
    pub fn new(desired_amount: f64, result_amount: f64) -> Self {
        Self {
            desired_amount,
            result_amount,
        }
    }

    pub fn diff(&self) -> f64 {
        self.desired_amount - self.result_amount
    }

    pub fn diff_percent(&self) -> f64 {
        let diff = self.diff().abs();

        if diff == 0. {
            diff
        } else {
            (diff / self.desired_amount) * 100.
        }
    }

    pub fn diff_state(&self) -> String {
        let diff_percent = self.diff_percent();

        if diff_percent < 2. {
            return String::from("success");
        }

        if diff_percent < 10. {
            return String::from("warn");
        }

        String::from("error")
    }
}
