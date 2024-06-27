use crate::storage::SolutionsStorage;
use crate::ui::components::layout::{Page, Section};
use crate::ui::components::solutions::{SolutionsListingControls, SolutionsListingTable};
use crate::ui::components::utils::{Block, Card, Divider, Title};
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn SolutionsListingPage() -> Element {
    let solutions_storage = consume_context::<Signal<SolutionsStorage>>();

    let mut solutions_listing = use_signal(|| solutions_storage.read().list());

    rsx! {
        Page {
            Section {
                Card {
                    Block {
                        Title {
                            text: "Список растворов",
                        }
                    }

                    Divider {}

                    Block {
                        SolutionsListingControls {
                            search_query: solutions_listing.read().search_query(),
                            on_search: move |search_query| {
                                solutions_listing.write().search(search_query);
                            },
                            on_add: move |_| {
                                navigator().push(Route::SolutionAddPage {
                                    profile_id: String::new(),
                                });
                            },
                        }
                    }

                    Block {
                        exclude_padding: "top",

                        SolutionsListingTable {
                            solutions_listing,
                            on_open: move |solution_id| {
                                navigator().push(Route::SolutionEditPage {
                                    solution_id,
                                });
                            },
                            on_stock: move |solution_id| {
                                navigator().push(Route::StockSolutionPage {
                                    solution_id,
                                });
                            },
                            on_delete: move |solution_id| {
                                solutions_storage.read().delete(solution_id);

                                *solutions_listing.write() = solutions_storage.read().list();
                            },
                            on_paginate: move |page_index| {
                                solutions_listing.write().paginate(page_index);
                            },
                        }
                    }
                }
            }
        }
    }
}
