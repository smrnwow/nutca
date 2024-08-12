use super::router::Route;
use crate::controller::Toaster;
use crate::repository::Storage;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn App() -> Element {
    use_context_provider(|| Signal::new(Toaster::new()));

    use_context_provider(|| Signal::new(Storage::new().unwrap()));

    rsx! {
        Router::<Route> {}
    }
}
