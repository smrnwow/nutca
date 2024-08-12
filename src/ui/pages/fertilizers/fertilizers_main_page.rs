use crate::controller::fertilizers::Dashboard;
use crate::repository::Storage;
use crate::ui::components::fertilizers::FertilizersListing;
use dioxus::prelude::*;

#[component]
pub fn FertilizersMainPage() -> Element {
    let storage = consume_context::<Signal<Storage>>();

    let mut dashboard = use_signal(|| Dashboard::new(storage));

    rsx! {
        FertilizersListing {
            fertilizers_listing: dashboard.read().listing(),
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
