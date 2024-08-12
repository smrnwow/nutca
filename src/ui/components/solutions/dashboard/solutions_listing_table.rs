use super::SolutionsListingItem;
use crate::controller::solutions::SolutionsListing;
use crate::ui::components::layout::Column;
use crate::ui::components::utils::{List, Pagination};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SolutionsListingTableProps {
    solutions_listing: Signal<SolutionsListing>,
    on_open: EventHandler<String>,
    on_stock: EventHandler<String>,
    on_delete: EventHandler<String>,
    on_paginate: EventHandler<usize>,
}

#[component]
pub fn SolutionsListingTable(props: SolutionsListingTableProps) -> Element {
    let solutions = use_memo(move || props.solutions_listing.read().fetch());

    rsx! {
        Column {
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
