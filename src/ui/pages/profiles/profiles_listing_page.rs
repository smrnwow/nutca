use crate::storage::ProfilesStorage;
use crate::ui::components::layout::{Column, Page, Section};
use crate::ui::components::profiles::{ProfileListingItem, ProfilesListingControls};
use crate::ui::components::utils::{Block, Card, Divider, Pagination, Title};
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn ProfilesListingPage() -> Element {
    let profiles_storage = consume_context::<Signal<ProfilesStorage>>();

    let mut profiles_listing = use_signal(|| profiles_storage.read().list());

    let profiles = use_memo(move || profiles_listing.read().list());

    rsx! {
        Page {
            Section {
                Card {
                    Block {
                        Title {
                            text: "Список профилей питания",
                        }
                    }

                    Divider {}

                    Block {
                        ProfilesListingControls {
                            search_query: profiles_listing.read().search_query(),
                            on_search: move |search_query| {
                                profiles_listing.write().search(search_query);
                            },
                            on_add: move |_| {
                                navigator().push(Route::ProfileAddPage {});
                            },
                        }
                    }

                    Block {
                        exclude_padding: "top",

                        Column {
                            div {
                                class: "profiles-listing-table",

                                for profile in profiles.read().clone() {
                                    ProfileListingItem {
                                        key: "{profile.id()}",
                                        profile,
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
                                    }
                                }
                            }

                            Pagination {
                                page_index: profiles_listing.read().page_index(),
                                limit: profiles_listing.read().limit(),
                                total: profiles_listing.read().total(),
                                on_change: move |next_page| {
                                    profiles_listing.write().paginate(next_page);
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}
