use crate::repository::SolutionsListing;
use crate::ui::components::layout::Column;
use crate::ui::components::solutions::SolutionsListingItem;
use crate::ui::components::utils::{List, Pagination};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SolutionsListingTableProps {
    solutions_listing: Memo<SolutionsListing>,
    on_open: EventHandler<String>,
    on_stock: EventHandler<String>,
    on_delete: EventHandler<String>,
    on_paginate: EventHandler<usize>,
}

#[component]
pub fn SolutionsListingTable(props: SolutionsListingTableProps) -> Element {
    let solutions = props.solutions_listing.read().list();

    rsx! {
        Column {
            List {
                limit: 10,
                empty: solutions.len() == 0,
                stub_text: "Сохраненные растворы отсутствуют",

                for solution in solutions {
                    SolutionsListingItem {
                        key: "{solution.id()}",
                        solution,
                        on_open: props.on_open,
                        on_stock: props.on_stock,
                        on_delete: props.on_delete,
                    }
                }
            }

            Pagination {
                page_index: props.solutions_listing.read().page_index(),
                limit: props.solutions_listing.read().limit(),
                total: props.solutions_listing.read().total(),
                on_change: props.on_paginate,
            }
        }
    }
}
