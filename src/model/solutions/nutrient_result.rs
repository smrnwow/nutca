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
        println!(
            "result {} desired {}",
            self.result_amount, self.desired_amount
        );

        let diff = (self.result_amount - self.desired_amount).abs();

        println!("diff {}", diff);

        if diff == 0. {
            println!("0 result {} ", diff);

            diff
        } else {
            println!("1 result {} ", (diff / self.desired_amount) * 100.);

            (diff / self.desired_amount) * 100.
        }
    }
}
