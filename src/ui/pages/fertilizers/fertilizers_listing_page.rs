use crate::storage::FertilizersStorage;
use crate::ui::components::fertilizers::FertilizerListingItem;
use crate::ui::components::layout::{Column, Page, Row, Section};
use crate::ui::components::utils::icons::SearchIcon;
use crate::ui::components::utils::{Block, Button, Card, Divider, Pagination, TextField, Title};
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

    let fertilizers = use_memo(move || fertilizers_listing.read().list());

    rsx! {
        Page {
            Section {
                Card {
                    Block {
                        Title {
                            text: "Список удобрений",
                        }
                    }

                    Divider {}

                    Block {
                        Row {
                            TextField {
                                value: fertilizers_listing.read().search_query(),
                                placeholder: "найти удобрение",
                                on_input: move |query| {
                                    fertilizers_listing.write().search(query);
                                },
                                icon_left: rsx! {
                                    SearchIcon {}
                                },
                            }

                            Button {
                                style: "primary",
                                on_click: move |_| {
                                    navigator().push(Route::FertilizerAddPage {});
                                },

                                "Добавить удобрение",
                            }
                        }
                    }

                    Block {
                        exclude_padding: "top",

                        Column {
                            div {
                                class: "fertilizers-listing-table",

                                for fertilizer in fertilizers.read().clone() {
                                    FertilizerListingItem {
                                        key: "{fertilizer.id()}",
                                        fertilizer,
                                        on_open: move |fertilizer_id| {
                                            navigator().push(Route::FertilizerEditPage {
                                                fertilizer_id
                                            });
                                        },
                                        on_delete: move |fertilizer_id| {
                                            fertilizers_storage.read().delete(fertilizer_id);

                                            *fertilizers_listing.write() = fertilizers_storage.read().list();
                                        },
                                    }
                                }
                            }

                            Pagination {
                                page_index: fertilizers_listing.read().page_index(),
                                limit: fertilizers_listing.read().limit(),
                                total: fertilizers_listing.read().total(),
                                on_change: move |next_page| {
                                    fertilizers_listing.write().paginate(next_page);
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}
