use crate::model::fertilizers::Fertilizer;
use crate::model::solutions::{FertilizerWeight, Solution};
use crate::repository::Storage;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct PickedSolution {
    storage: Signal<Storage>,
    solution: Solution,
    fertilizers_amounts: Vec<FertilizerWeight>,
}

impl PickedSolution {
    pub fn new(storage: Signal<Storage>) -> Self {
        Self {
            storage,
            solution: Solution::default(),
            fertilizers_amounts: Vec::new(),
        }
    }

    pub fn change(&mut self, solution: Solution) {
        self.solution = solution;

        self.fetch_fertilizers().into_iter().for_each(|fertilizer| {
            if let Some(calculation_result) = self.solution.fertilizer(&fertilizer.id()) {
                self.fertilizers_amounts.push(FertilizerWeight::new(
                    fertilizer,
                    calculation_result.amount(),
                ));
            }
        });
    }

    pub fn fertilizers(&self) -> &Vec<FertilizerWeight> {
        &self.fertilizers_amounts
    }

    pub fn id(&self) -> String {
        self.solution.id()
    }

    pub fn name(&self) -> String {
        self.solution.name()
    }

    fn fetch_fertilizers(&self) -> Vec<Fertilizer> {
        let fertilizers_amounts = self.solution.fertilizers();

        let fertilizers_ids = fertilizers_amounts
            .iter()
            .map(|fertilizer_amount| fertilizer_amount.0.clone())
            .collect();

        match self.storage.read().fertilizers().filter(&fertilizers_ids) {
            Ok(fertilizers) => fertilizers,
            Err(error) => {
                println!("storage error {:#?}", error);
                Vec::new()
            }
        }
    }
}
