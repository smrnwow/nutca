use crate::model::concentrates::parts::AutoPart;
use crate::model::fertilizers::Fertilizer;
use crate::model::solutions::FertilizerWeight;
use crate::repository::Storage;
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct FertilizersStack {
    storage: Signal<Storage>,
    amounts: Vec<FertilizerWeight>,
    fertilizers: HashMap<String, Fertilizer>,
    stack: Vec<(String, usize)>,
}

impl FertilizersStack {
    pub fn new(storage: Signal<Storage>) -> Self {
        Self {
            storage,
            amounts: Vec::new(),
            fertilizers: HashMap::new(),
            stack: Vec::new(),
        }
    }

    pub fn with_amounts(&mut self, amounts: Vec<FertilizerWeight>) -> &mut Self {
        self.amounts = amounts;

        self.load_fertilizers(self.amounts.iter().map(|amount| amount.id()).collect());

        self
    }

    pub fn with_stack(&mut self, stack: &HashMap<String, usize>) -> &mut Self {
        self.stack = stack
            .iter()
            .map(|(fertilizer_id, percent)| (fertilizer_id.clone(), *percent))
            .collect();

        self
    }

    pub fn stack(&self) -> Vec<(Fertilizer, usize)> {
        let mut stack: Vec<(Fertilizer, usize)> = Vec::new();

        self.stack.iter().for_each(|(fertilizer_id, percent)| {
            if let Some(fertilizer) = self.fertilizers.get(fertilizer_id) {
                stack.push((fertilizer.clone(), *percent));
            }
        });

        stack
    }

    pub fn list(&self, part: &AutoPart) -> Vec<Signal<(Fertilizer, f64)>> {
        let mut list = Vec::new();

        part.fertilizers().iter().for_each(|fertilizer_percent| {
            if let Some(fertilizer) = self.fertilizers.get(&fertilizer_percent.id()) {
                list.push(Signal::new((
                    fertilizer.clone(),
                    fertilizer_percent.amount(),
                )));
            }
        });

        list
    }

    fn load_fertilizers(&mut self, fertilizers_ids: Vec<String>) {
        let fertilizers = match self.storage.read().fertilizers().filter(&fertilizers_ids) {
            Ok(fertilizers) => fertilizers,
            Err(error) => {
                println!("storage error {:#?}", error);
                Vec::new()
            }
        };

        fertilizers.into_iter().for_each(|fertilizer| {
            self.fertilizers.insert(fertilizer.id(), fertilizer);
        });
    }
}
