use super::router::Route;
use crate::storage::{FertilizersStorage, ProfilesStorage};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub fn App() -> Element {
    use_context_provider(|| Signal::new(FertilizersStorage::new()));

    use_context_provider(|| Signal::new(ProfilesStorage::new()));

    rsx! {
        Router::<Route> {}
    }
}
