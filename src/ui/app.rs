use super::router::Route;
use crate::storage::{FertilizersStorage, ProfilesStorage, SolutionsStorage};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub fn App() -> Element {
    use_context_provider(|| Signal::new(FertilizersStorage::new()));

    use_context_provider(|| Signal::new(ProfilesStorage::new()));

    use_context_provider(|| Signal::new(SolutionsStorage::new()));

    rsx! {
        Router::<Route> {}
    }
}
