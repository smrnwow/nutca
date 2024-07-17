use crate::controller::solutions::Dashboard;
use crate::repository::Storage;
use crate::ui::components::layout::{Page, Section};
use crate::ui::components::solutions::{SolutionsListingControls, SolutionsListingTable};
use crate::ui::components::utils::{Block, Card, Divider, Title};
use dioxus::prelude::*;

#[component]
pub fn SolutionsListingPage() -> Element {
    let storage = consume_context::<Signal<Storage>>();

    let mut dashboard = use_signal(|| Dashboard::new(storage));

    let solutions_listing = use_memo(move || dashboard.read().list_solutions());

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
                                dashboard.write().search_solution(search_query);
                            },
                            on_add: move |_| {
                                dashboard.read().add_solution();
                            },
                        }
                    }

                    Block {
                        exclude_padding: "top",

                        SolutionsListingTable {
                            solutions_listing,
                            on_open: move |solution_id| {
                                dashboard.read().open_solution(solution_id);
                            },
                            on_stock: move |solution_id| {
                                dashboard.read().open_stock_solution(solution_id);
                            },
                            on_delete: move |solution_id| {
                                dashboard.write().delete_solution(solution_id);
                            },
                            on_paginate: move |page_index| {
                                dashboard.write().paginate(page_index);
                            },
                        }
                    }
                }
            }
        }
    }
}
