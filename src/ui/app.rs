use super::router::Route;
use crate::storage::{ArticlesStorage, FertilizersStorage, ProfilesStorage, SolutionsStorage};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn App() -> Element {
    use_context_provider(|| Signal::new(FertilizersStorage::new()));

    use_context_provider(|| Signal::new(ProfilesStorage::new()));

    use_context_provider(|| Signal::new(SolutionsStorage::new()));

    use_context_provider(|| Signal::new(ArticlesStorage::new()));

    let articles_storage = use_context::<Signal<ArticlesStorage>>();

    use_context_provider(|| Signal::new(articles_storage.read().browse()));

    rsx! {
        Router::<Route> {}
    }
}
