use crate::storage::ProfilesStorage;
use crate::ui::components::profiles::ProfileListingItem;
use crate::ui::components::utils::{Block, Button, Card, Divider, Table, TableCell, Title};
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn ProfilesListingPage() -> Element {
    let profiles_storage = consume_context::<Signal<ProfilesStorage>>();

    let profiles = use_memo(move || profiles_storage.read().list());

    rsx! {
        div {
            class: "profiles-listing-page",

            section {
                class: "profiles-listing-page__listing",

                Card {
                    Block {
                        Title {
                            text: "Список профилей питания",
                        }
                    }

                    Divider {}

                    Block {
                        div {
                            class: "profiles-listing-page__controls",

                            Button {
                                style: "primary",
                                text: "Добавить профиль питания",
                                on_click: move |_| {
                                    navigator().push(Route::ProfileAddPage {});
                                },
                            }
                        }
                    }

                    Block {
                        exclude_padding: "top",

                        Table {
                            header: rsx! {
                                TableCell {
                                    width: "100%",
                                    "Название",
                                }
                            },
                            body: rsx! {
                                for profile in profiles.read().clone() {
                                    ProfileListingItem {
                                        profile,
                                        on_select: move |profile_id| {
                                            navigator().push(Route::ProfileEditPage {
                                                profile_id,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
