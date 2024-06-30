use crate::storage::ProfilesStorage;
use crate::ui::components::layout::{Page, Section};
use crate::ui::components::profiles::ProfilesListing;
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn ProfilesListingPage() -> Element {
    let profiles_storage = consume_context::<Signal<ProfilesStorage>>();

    let mut profiles_listing = use_signal(|| profiles_storage.read().list());

    rsx! {
        Page {
            Section {
                ProfilesListing {
                    profiles_listing,
                    on_search: move |search_query| {
                        profiles_listing.write().search(search_query);
                    },
                    on_add: move |_| {
                        navigator().push(Route::ProfileAddPage {});
                    },
                    on_open: move |profile_id| {
                        navigator().push(Route::ProfileEditPage {
                            profile_id,
                        });
                    },
                    on_use: move |profile_id| {
                        navigator().push(Route::SolutionAddPage {
                            profile_id,
                        });
                    },
                    on_delete: move |profile_id| {
                        profiles_storage.read().delete(profile_id);

                        *profiles_listing.write() = profiles_storage.read().list();
                    },
                    on_paginate: move |page_index| {
                        profiles_listing.write().paginate(page_index);
                    },
                }
            }
        }
    }
}
