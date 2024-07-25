#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NutrientResult {
    requirement: f64,
    value: f64,
}

impl NutrientResult {
    pub fn new(requirement: f64, value: f64) -> Self {
        Self { requirement, value }
    }

    pub fn diff(&self) -> f64 {
        self.requirement - self.value
    }

    pub fn diff_percent(&self) -> f64 {
        let diff = self.diff().abs();

        if diff == 0. {
            diff
        } else {
            (diff / self.requirement) * 100.
        }
    }

    pub fn diff_state(&self) -> String {
        let diff_percent = self.diff_percent();

        if diff_percent < 1. {
            return String::from("success");
        }

        if diff_percent < 50. {
            return String::from("warn");
        }

        String::from("error")
    }
}
