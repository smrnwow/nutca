use crate::controller::{Error, Validation};
use crate::model::fertilizers::{Fertilizer, FertilizerBuilder};
use crate::repository::Storage;
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub struct FertilizerEditor {
    is_draft: Signal<bool>,
    storage: Signal<Storage>,
    storage_error: Signal<Option<Error>>,
    builder: Signal<FertilizerBuilder>,
    fertilizer: Memo<Fertilizer>,
    validation: Memo<Validation>,
}

impl FertilizerEditor {
    pub fn new(storage: Signal<Storage>) -> Self {
        let builder = Signal::new(FertilizerBuilder::new());

        let is_draft = Signal::new(true);

        let storage_error = Signal::new(None);

        Self {
            is_draft,
            storage,
            storage_error,
            builder,
            fertilizer: Memo::new(move || builder.read().build()),
            validation: Memo::new(move || {
                return Validation::new(
                    !*is_draft.read(),
                    builder.read().validate(),
                    storage_error.read().clone(),
                );
            }),
        }
    }

    pub fn edit(storage: Signal<Storage>, fertilizer_id: String) -> Self {
        let fertilizer = storage.read().fertilizers().get(&fertilizer_id);

        let builder = match fertilizer {
            Ok(fertilizer) => Signal::new(FertilizerBuilder::from(fertilizer)),
            Err(_) => Signal::new(FertilizerBuilder::new()),
        };

        let is_draft = Signal::new(true);

        let storage_error = Signal::new(None);

        Self {
            is_draft,
            storage,
            storage_error,
            builder,
            fertilizer: Memo::new(move || builder.read().build()),
            validation: Memo::new(move || {
                return Validation::new(
                    !*is_draft.read(),
                    builder.read().validate(),
                    storage_error.read().clone(),
                );
            }),
        }
    }

    pub fn fertilizer_builder(&self) -> Signal<FertilizerBuilder> {
        self.builder
    }

    pub fn fertilizer(&self) -> Memo<Fertilizer> {
        self.fertilizer
    }

    pub fn validation(&self) -> Memo<Validation> {
        self.validation
    }

    pub fn create(&mut self) {
        self.is_draft.set(false);

        if self.validation.read().is_empty() {
            let result = self
                .storage
                .read()
                .fertilizers()
                .add(self.fertilizer.read().clone());

            match result {
                Ok(_) => {
                    navigator().push(Route::FertilizersMainPage {});
                }
                Err(error) => {
                    self.storage_error
                        .set(Some(Error::RepositoryError(error.to_string())));
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.is_draft.set(false);

        if self.validation.read().is_empty() {
            let result = self
                .storage
                .read()
                .fertilizers()
                .update(self.fertilizer.read().clone());

            match result {
                Ok(_) => {
                    navigator().push(Route::FertilizersMainPage {});
                }
                Err(error) => {
                    self.storage_error
                        .set(Some(Error::RepositoryError(error.to_string())));
                }
            }
        }
    }

    pub fn back(&mut self) {
        navigator().push(Route::FertilizersMainPage {});
    }
}
