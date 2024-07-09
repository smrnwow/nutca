use super::router::Route;
use crate::model::reference::Browser;
use crate::model::NotificationContainer;
use crate::storage::Storage;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn App() -> Element {
    use_context_provider(|| Signal::new(NotificationContainer::new()));

    use_context_provider(|| Signal::new(Storage::new().unwrap()));

    let storage = use_context::<Signal<Storage>>();

    use_context_provider(|| match storage.read().articles().browse() {
        Ok(browser) => Signal::new(browser),
        Err(_) => Signal::new(Browser::empty()),
    });

    rsx! {
        Router::<Route> {}
    }
}
