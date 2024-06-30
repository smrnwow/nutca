use crate::storage::FertilizersStorage;
use crate::ui::components::fertilizers::FertilizersListing;
use crate::ui::components::layout::{Page, Section};
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn FertilizersListingPage() -> Element {
    let fertilizers_storage = consume_context::<Signal<FertilizersStorage>>();

    let mut fertilizers_listing = use_signal(move || {
        let mut listing = fertilizers_storage.read().list();

        listing.update_limit(10);

        listing
    });

    rsx! {
        Page {
            Section {
                FertilizersListing {
                    fertilizers_listing,
                    on_search: move |query| {
                        fertilizers_listing.write().search(query);
                    },
                    on_add: move |_| {
                        navigator().push(Route::FertilizerAddPage {});
                    },
                    on_open: move |fertilizer_id| {
                        navigator().push(Route::FertilizerEditPage {
                            fertilizer_id
                        });
                    },
                    on_delete: move |fertilizer_id| {
                        fertilizers_storage.read().delete(fertilizer_id);

                        *fertilizers_listing.write() = fertilizers_storage.read().list();
                    },
                    on_paginate: move |next_page| {
                        fertilizers_listing.write().paginate(next_page);
                    },
                }
            }
        }
    }
}
