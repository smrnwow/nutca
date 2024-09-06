use super::{FertilizersBrowser, FertilizersStack};
use crate::model::chemistry::Volume;
use crate::model::concentrates::fillers::{AutoFiller, Filler, FillerVariant, ManualFiller};
use crate::model::concentrates::parts::{AutoPart, ManualPart};
use crate::model::concentrates::Concentrate;
use crate::model::solutions::Solution;
use crate::repository::Storage;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct TanksSet {
    filler_variant: FillerVariant,
    auto_filler: AutoFiller,
    manual_filler: ManualFiller,
    fertilizers_browser: FertilizersBrowser,
    fertilizers_stack: FertilizersStack,
}

impl TanksSet {
    pub fn from_concentrate(
        storage: Signal<Storage>,
        concentrate: &Concentrate,
        solution: &Solution,
    ) -> Self {
        let auto_filler = match concentrate.filler() {
            Filler::Auto(auto_filler) => auto_filler.clone(),
            Filler::Manual(_) => AutoFiller::new(solution),
        };

        let manual_filler = match concentrate.filler() {
            Filler::Auto(_) => ManualFiller::new(),
            Filler::Manual(manual_filler) => manual_filler.clone(),
        };

        let filler_variant = match concentrate.filler() {
            Filler::Auto(_) => FillerVariant::Auto,
            Filler::Manual(_) => FillerVariant::Manual,
        };

        let mut fertilizers_stack = FertilizersStack::new(storage);

        fertilizers_stack
            .with_amounts(solution.fertilizers())
            .with_stack(auto_filler.stack());

        let mut fertilizers_browser = FertilizersBrowser::new(storage);

        fertilizers_browser.with_fertilizers(manual_filler.fertilizers_ids());

        Self {
            filler_variant,
            auto_filler,
            manual_filler,
            fertilizers_stack,
            fertilizers_browser,
        }
    }

    pub fn from_solution(storage: Signal<Storage>, solution: &Solution) -> Self {
        let auto_filler = AutoFiller::new(solution);

        let mut fertilizers_stack = FertilizersStack::new(storage);

        fertilizers_stack
            .with_amounts(solution.fertilizers())
            .with_stack(auto_filler.stack());

        Self {
            filler_variant: FillerVariant::Auto,
            auto_filler: AutoFiller::new(solution),
            manual_filler: ManualFiller::new(),
            fertilizers_stack,
            fertilizers_browser: FertilizersBrowser::new(storage),
        }
    }

    pub fn fertilizers_stack(&self) -> &FertilizersStack {
        &self.fertilizers_stack
    }

    pub fn fertilizers_browser(&self) -> &FertilizersBrowser {
        &self.fertilizers_browser
    }

    pub fn filler_variant(&self) -> FillerVariant {
        self.filler_variant
    }

    pub fn auto_parts(&self) -> Vec<Signal<AutoPart>> {
        self.auto_filler
            .parts()
            .iter()
            .cloned()
            .map(|part| Signal::new(part))
            .collect()
    }

    pub fn manual_parts(&self) -> Vec<Signal<ManualPart>> {
        self.manual_filler
            .parts()
            .iter()
            .cloned()
            .map(|part| Signal::new(part))
            .collect()
    }

    pub fn composition(&self) -> Filler {
        match self.filler_variant {
            FillerVariant::Auto => Filler::Auto(self.auto_filler.clone()),
            FillerVariant::Manual => Filler::Manual(self.manual_filler.clone()),
        }
    }

    pub fn change_solution(&mut self, solution: &Solution) {
        self.auto_filler = AutoFiller::new(solution);

        self.fertilizers_stack
            .with_amounts(solution.fertilizers())
            .with_stack(self.auto_filler.stack());
    }

    pub fn change_filler_variant(&mut self, filler_variant: FillerVariant) {
        self.filler_variant = filler_variant;
    }

    pub fn add_part(&mut self) {
        match self.filler_variant {
            FillerVariant::Auto => self.auto_filler.add_part(),
            FillerVariant::Manual => self.manual_filler.add_part(),
        };
    }

    pub fn delete_part(&mut self, part_id: String) {
        match self.filler_variant {
            FillerVariant::Auto => {
                self.auto_filler.delete_part(part_id);

                self.fertilizers_stack.with_stack(self.auto_filler.stack());
            }
            FillerVariant::Manual => self.manual_filler.delete_part(part_id),
        };
    }

    pub fn update_part_name(&mut self, part_id: String, name: String) -> Option<()> {
        match self.filler_variant {
            FillerVariant::Auto => {
                self.auto_filler.get_part(&part_id)?.update_name(name);
                Some(())
            }

            FillerVariant::Manual => self
                .manual_filler
                .get_part(&part_id)
                .and_then(|part| Some(part.update_name(name))),
        }
    }

    pub fn update_part_concentration(&mut self, part_id: String, concentration: usize) {
        match self.filler_variant {
            FillerVariant::Auto => self
                .auto_filler
                .update_part_concentration(part_id, concentration),
            FillerVariant::Manual => self
                .manual_filler
                .update_part_concentration(part_id, concentration),
        };
    }

    pub fn update_part_volume(&mut self, part_id: String, volume: Volume) {
        match self.filler_variant {
            FillerVariant::Auto => self.auto_filler.update_part_volume(part_id, volume),
            FillerVariant::Manual => self.manual_filler.update_part_volume(part_id, volume),
        };
    }

    pub fn add_part_fertilizer(&mut self, part_id: String, fertilizer_id: String, value: f64) {
        match self.filler_variant {
            FillerVariant::Auto => {
                self.auto_filler
                    .add_fertilizer(part_id, fertilizer_id, value as usize);

                self.fertilizers_stack.with_stack(self.auto_filler.stack());
            }

            FillerVariant::Manual => {
                let fertilizer = self.fertilizers_browser.get(&fertilizer_id);

                if let Some(fertilizer) = fertilizer {
                    self.manual_filler
                        .add_part_fertilizer(part_id, fertilizer, value);
                }
            }
        };
    }

    pub fn delete_part_fertilizer(&mut self, part_id: String, fertilizer_id: String) {
        match self.filler_variant {
            FillerVariant::Auto => {
                self.auto_filler.delete_fertilizer(part_id, fertilizer_id);

                self.fertilizers_stack.with_stack(self.auto_filler.stack());
            }
            FillerVariant::Manual => {
                self.manual_filler
                    .delete_part_fertilizer(part_id, fertilizer_id);
            }
        }
    }
}
