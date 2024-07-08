use crate::model::fertilizers::FertilizersListing;
use crate::storage::Storage;
use crate::ui::components::fertilizers::FertilizersListing;
use crate::ui::components::layout::{Page, Section};
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn FertilizersListingPage() -> Element {
    let storage = consume_context::<Signal<Storage>>();

    let mut fertilizers_listing = use_signal(move || {
        let listing = storage.read().fertilizers().list();

        match listing {
            Ok(mut listing) => {
                listing.update_limit(10);

                listing
            }

            Err(_) => FertilizersListing::new(vec![]),
        }
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
                        storage.read().fertilizers().delete(fertilizer_id).unwrap();

                        if let Ok(listing) = storage.read().fertilizers().list() {
                            fertilizers_listing.set(listing);
                        }
                    },
                    on_paginate: move |next_page| {
                        fertilizers_listing.write().paginate(next_page);
                    },
                }
            }
        }
    }
}
