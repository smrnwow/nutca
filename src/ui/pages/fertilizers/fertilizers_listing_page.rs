use crate::storage::FertilizersStorage;
use crate::ui::components::fertilizers::Listing;
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn FertilizersListingPage() -> Element {
    let fertilizers_storage = consume_context::<Signal<FertilizersStorage>>();

    let fertilizers_list = use_memo(move || fertilizers_storage.read().list());

    rsx! {
        div {
            class: "fertilizers-index",

            section {
                class: "fertilizer-listing",

                Listing {
                    fertilizers: fertilizers_list,
                    on_editor_open: move |_| {
                        navigator().push(Route::FertilizerEditorPage {});
                    },
                    on_search: move |search_query| {
                        println!("on_search {}", search_query);
                    },
                }
            }
        }
    }
}
