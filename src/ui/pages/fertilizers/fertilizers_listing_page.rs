use crate::storage::FertilizersStorage;
use crate::ui::components::fertilizers::FertilizerListingItem;
use crate::ui::components::utils::{Block, Button, Card, Divider, Search, Table, TableCell, Title};
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn FertilizersListingPage() -> Element {
    let fertilizers_storage = consume_context::<Signal<FertilizersStorage>>();

    let fertilizers = use_memo(move || fertilizers_storage.read().list());

    rsx! {
        div {
            class: "fertilizers-index",

            section {
                class: "fertilizer-listing",

                Card {
                    Block {
                        Title {
                            text: "Список удобрений ({ fertilizers.len() })",
                        }
                    }

                    Divider {}

                    Block {
                        div {
                            class: "fertilizer-listing__header",

                            Search {
                                placeholder: "найти удобрение",
                                on_change: move |search_query| {
                                    println!("on_search {}", search_query);
                                },
                            }

                            Button {
                                style: "primary",
                                text: "Добавить удобрение",
                                on_click: move |_| {
                                    navigator().push(Route::FertilizerAddPage {});
                                },
                            }
                        }
                    }

                    Block {
                        exclude_padding: "top",

                        Table {
                            header: rsx! {
                                TableCell {
                                    width: "50%",
                                    "Название",
                                }

                                TableCell {
                                    width: "50%",
                                    "Состав",
                                }
                            },
                            body: rsx! {
                                for fertilizer in fertilizers.read().clone() {
                                    FertilizerListingItem {
                                        fertilizer,
                                        on_select: move |fertilizer_id| {
                                            navigator().push(Route::FertilizerEditPage {
                                                fertilizer_id
                                            });
                                        },
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
