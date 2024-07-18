use super::router::Route;
use crate::controller::Toaster;
use crate::repository::{ReferenceBrowser, Storage};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn App() -> Element {
    use_context_provider(|| Signal::new(Toaster::new()));

    use_context_provider(|| Signal::new(Storage::new().unwrap()));

    let storage = use_context::<Signal<Storage>>();

    use_context_provider(|| match storage.read().reference().browse() {
        Ok(browser) => Signal::new(browser),
        Err(_) => Signal::new(ReferenceBrowser::empty()),
    });

    rsx! {
        Router::<Route> {}
    }
}
