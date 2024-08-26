use super::SolutionsListingItem;
use crate::controller::solutions::SolutionsListing;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::icons::SearchIcon;
use crate::ui::components::utils::{
    Block, Button, Card, Divider, List, Pagination, TextField, Title,
};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SolutionsDashboardProps {
    solutions_listing: Signal<SolutionsListing>,
    on_search: EventHandler<String>,
    on_add: EventHandler<()>,
    on_open: EventHandler<String>,
    on_stock: EventHandler<String>,
    on_delete: EventHandler<String>,
    on_paginate: EventHandler<usize>,
}

#[component]
pub fn SolutionsDashboard(props: SolutionsDashboardProps) -> Element {
    let solutions = use_memo(move || props.solutions_listing.read().fetch());

    rsx! {
        Card {
            Block {
                Row {
                    Title {
                        "Растворы",
                    }
                }
            }

            Divider {}

            Block {
                Column {
                    gap: "medium",

                    Row {
                        gap: "medium",

                        TextField {
                            value: props.solutions_listing.read().search_query(),
                            placeholder: "найти раствор",
                            on_input: props.on_search,
                            icon_left: rsx! {
                                SearchIcon {}
                            },
                        }

                        Button {
                            style: "primary",
                            on_click: props.on_add,
                            "Добавить раствор",
                        }
                    }

                    List {
                        limit: 10,
                        empty: solutions.len() == 0,
                        stub_text: "Сохраненные растворы отсутствуют",

                        for solution in solutions.read().iter() {
                            SolutionsListingItem {
                                key: "{solution.id()}",
                                solution: solution.clone(),
                                on_open: props.on_open,
                                on_stock: props.on_stock,
                                on_delete: props.on_delete,
                            }
                        }
                    }

                    Pagination {
                        page_index: props.solutions_listing.read().page_index(),
                        limit: props.solutions_listing.read().limit(),
                        items_count: solutions.read().len(),
                        on_change: props.on_paginate,
                    }
                }
            }
        }
    }
}
