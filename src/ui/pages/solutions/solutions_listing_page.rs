use crate::model::solutions::SolutionsListing;
use crate::storage::Storage;
use crate::ui::components::layout::{Page, Section};
use crate::ui::components::solutions::{SolutionsListingControls, SolutionsListingTable};
use crate::ui::components::utils::{Block, Card, Divider, Title};
use crate::ui::router::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn SolutionsListingPage() -> Element {
    let storage = consume_context::<Signal<Storage>>();

    let mut solutions_listing = use_signal(|| match storage.read().solutions().list() {
        Ok(listing) => listing,
        Err(_) => SolutionsListing::new(vec![]),
    });

    rsx! {
        Page {
            Section {
                Card {
                    Block {
                        Title {
                            "Список растворов",
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
                                storage.read().solutions().delete(solution_id).unwrap();

                                *solutions_listing.write() = match storage.read().solutions().list() {
                                    Ok(listing) => listing,
                                    Err(_) => SolutionsListing::new(vec![]),
                                };
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
