use super::{SolutionsListingControls, SolutionsListingTable};
use crate::controller::reference::TopicId;
use crate::repository::SolutionsListing;
use crate::ui::components::layout::Row;
use crate::ui::components::reference::ReferenceBadge;
use crate::ui::components::utils::{Block, Card, Divider, Title};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SolutionsDashboardProps {
    solutions_listing: Memo<SolutionsListing>,
    on_search: EventHandler<String>,
    on_add: EventHandler<()>,
    on_open: EventHandler<String>,
    on_stock: EventHandler<String>,
    on_delete: EventHandler<String>,
    on_paginate: EventHandler<usize>,
}

#[component]
pub fn SolutionsDashboard(props: SolutionsDashboardProps) -> Element {
    rsx! {
        Card {
            Block {
                Row {
                    Title {
                        {TopicId::SolutionsDashboard.title()},
                        ReferenceBadge {
                            topic_id: TopicId::SolutionsDashboard,
                        },
                    }
                }
            }

            Divider {}

            Block {
                SolutionsListingControls {
                    search_query: props.solutions_listing.read().search_query(),
                    on_search: props.on_search,
                    on_add: props.on_add,
                }
            }

            Block {
                exclude_padding: "top",

                SolutionsListingTable {
                    solutions_listing: props.solutions_listing,
                    on_open: props.on_open,
                    on_stock: props.on_stock,
                    on_delete: props.on_delete,
                    on_paginate: props.on_paginate,
                }
            }
        }
    }
}
