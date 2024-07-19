use crate::controller::profiles::Dashboard;
use crate::repository::Storage;
use crate::ui::components::profiles::ProfilesListing;
use dioxus::prelude::*;

#[component]
pub fn ProfilesMainPage() -> Element {
    let storage = consume_context::<Signal<Storage>>();

    let mut dashboard = use_signal(|| Dashboard::new(storage));

    let profiles_listing = use_memo(move || dashboard.read().listing());

    rsx! {
        ProfilesListing {
            profiles_listing,
            on_search: move |search_query| {
                dashboard.write().search(search_query);
            },
            on_add: move |_| {
                dashboard.write().add();
            },
            on_open: move |profile_id| {
                dashboard.write().edit(profile_id);
            },
            on_use: move |profile_id| {
                dashboard.write().add_solution(profile_id);
            },
            on_delete: move |profile_id| {
                dashboard.write().delete(profile_id);
            },
            on_paginate: move |page_index| {
                dashboard.write().paginate(page_index);
            },
        }
    }
}
