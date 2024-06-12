use crate::storage::SolutionsStorage;
use crate::ui::components::layout::Row;
use crate::ui::components::solutions::SolutionListingItem;
use crate::ui::components::utils::{Block, Button, Card, Divider, Search, Table, TableCell, Title};
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn SolutionsListingPage() -> Element {
    let solutions_storage = consume_context::<Signal<SolutionsStorage>>();

    let mut solutions_listing = use_signal(|| solutions_storage.read().list());

    let solutions = use_memo(move || solutions_listing.read().list());

    rsx! {
        div {
            class: "solutions-listing-page",

            section {
                class: "solutions-listing-page__listing",

                Card {
                    Block {
                        Title {
                            text: "Список растворов",
                        }
                    }

                    Divider {}

                    Block {
                        Row {
                            Search {
                                placeholder: "найти раствор",
                                on_change: move |search_query| {
                                    solutions_listing.write().search(search_query);
                                },
                            }

                            Button {
                                style: "primary",
                                on_click: move |_| {
                                    navigator().push(Route::SolutionAddPage {
                                        profile_id: String::new(),
                                    });
                                },

                                "Добавить раствор",
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

                                TableCell {
                                    width: "1%",
                                }
                            },
                            body: rsx! {
                                for solution in solutions.read().clone() {
                                    SolutionListingItem {
                                        key: "{solution.id()}",
                                        solution,
                                        on_open: move |solution_id| {
                                            navigator().push(Route::SolutionEditPage {
                                                solution_id,
                                            });
                                        },
                                        on_delete: move |solution_id| {
                                            solutions_storage.read().delete(solution_id);

                                            *solutions_listing.write() = solutions_storage.read().list();
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
