use crate::model::solutions::Solution;
use crate::ui::components::utils::{TableCell, TableRow};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SolutionListingItemProps {
    solution: Solution,
    on_select: EventHandler<String>,
}

#[component]
pub fn SolutionListingItem(props: SolutionListingItemProps) -> Element {
    let solution_id = props.solution.id();

    let solution_name = props.solution.name();

    rsx! {
        TableRow {
            on_click: move |_| {
                props.on_select.call(solution_id.clone())
            },

            TableCell {
                p {
                    class: "solutions-listing__name",
                    "{solution_name}",
                }
            }
        }
    }
}
