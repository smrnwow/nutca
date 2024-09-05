use crate::controller::concentrates::Dashboard;
use crate::repository::Storage;
use crate::ui::components::concentrates::dashboard::Dashboard;
use dioxus::prelude::*;

#[component]
pub fn Main() -> Element {
    let storage = consume_context::<Signal<Storage>>();

    let mut dashboard = use_signal(|| Dashboard::new(storage));

    rsx! {
        Dashboard {
            listing: dashboard.read().listing(),
            on_search: move |search_query| {
                dashboard.write().search_solution(search_query);
            },
            on_add: move |_| {
                dashboard.read().create_concentrate();
            },
            on_open: move |concentrate_id| {
                dashboard.read().open_concentrate(concentrate_id);
            },
            on_delete: move |concentrate_id| {
                dashboard.write().delete_concentrate(concentrate_id);
            },
            on_paginate: move |page_index| {
                dashboard.write().paginate(page_index);
            },
        }
    }
}
