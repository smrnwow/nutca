use crate::model::concentrates::parts::AutoPart;
use crate::model::fertilizers::Fertilizer;
use crate::model::solutions::FertilizerWeight;
use crate::repository::Storage;
use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct FertilizersStack {
    storage: Signal<Storage>,
    amounts: HashMap<String, f64>,
    fertilizers: HashMap<String, Fertilizer>,
    stack: Vec<(String, usize)>,
}

impl FertilizersStack {
    pub fn new(storage: Signal<Storage>) -> Self {
        Self {
            storage,
            amounts: HashMap::new(),
            fertilizers: HashMap::new(),
            stack: Vec::new(),
        }
    }

    pub fn with_amounts(&mut self, amounts: &Vec<FertilizerWeight>) -> &mut Self {
        amounts.iter().for_each(|fertilizer_weight| {
            self.amounts
                .insert(fertilizer_weight.id(), fertilizer_weight.weight());
        });

        self.load_fertilizers(amounts.iter().map(|amount| amount.id()).collect());

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
                if let Some(amount) = self.amounts.get(&fertilizer_percent.id()) {
                    let value = fertilizer_percent.amount(*amount) * part.coefficient();

                    list.push(Signal::new((
                        fertilizer.clone(),
                        format!("{:.3}", value).parse().unwrap_or(0.0),
                    )));
                }
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
