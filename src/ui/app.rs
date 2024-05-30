use super::router::Route;
use crate::storage::FertilizersStorage;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub fn App() -> Element {
    use_context_provider(|| Signal::new(FertilizersStorage::new()));

    rsx! {
        Router::<Route> {}
    }
}
