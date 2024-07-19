use crate::controller::fertilizers::Dashboard;
use crate::repository::Storage;
use crate::ui::components::fertilizers::FertilizersListing;
use dioxus::prelude::*;

#[component]
pub fn FertilizersMainPage() -> Element {
    let storage = consume_context::<Signal<Storage>>();

    let mut dashboard = use_signal(|| Dashboard::new(storage));

    let fertilizers_listing = use_memo(move || dashboard.read().listing());

    rsx! {
        FertilizersListing {
            fertilizers_listing,
            on_search: move |query| {
                dashboard.write().search(query);
            },
            on_add: move |_| {
                dashboard.write().add();
            },
            on_open: move |fertilizer_id| {
                dashboard.write().edit(fertilizer_id);
            },
            on_delete: move |fertilizer_id| {
                dashboard.write().delete(fertilizer_id);
            },
            on_paginate: move |next_page| {
                dashboard.write().paginate(next_page);
            },
        }
    }
}
