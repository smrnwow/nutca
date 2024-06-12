use crate::model::fertilizers::FertilizersListing;
use crate::storage::FertilizersStorage;
use crate::ui::components::fertilizers::FertilizerListingItem;
use crate::ui::components::utils::{Block, Button, Card, Divider, Search, Table, TableCell, Title};
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn FertilizersListingPage() -> Element {
    let fertilizers_storage = consume_context::<Signal<FertilizersStorage>>();

    let mut fertilizers_listing = use_signal(move || {
        let fertilizers = fertilizers_storage.read().list();

        FertilizersListing::new(fertilizers)
    });

    let fertilizers = use_memo(move || fertilizers_listing.read().list());

    rsx! {
        div {
            class: "fertilizers-index",

            section {
                class: "fertilizer-listing",

                Card {
                    Block {
                        Title {
                            text: "Список удобрений",
                        }
                    }

                    Divider {}

                    Block {
                        div {
                            class: "fertilizer-listing__header",

                            Search {
                                placeholder: "найти удобрение",
                                on_change: move |query| {
                                    fertilizers_listing.write().search(query);
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

                                TableCell {
                                    width: "1%",
                                }
                            },
                            body: rsx! {
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
                                            println!("delete fertilizer {}", fertilizer_id);
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
