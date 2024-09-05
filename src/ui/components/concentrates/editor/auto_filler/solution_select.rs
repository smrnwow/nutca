use crate::controller::solutions::SolutionsListing;
use crate::model::solutions::Solution;
use crate::ui::components::layout::Row;
use crate::ui::components::utils::Select;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SolutionSelectProps {
    solution: Memo<Solution>,
    solutions_listing: Memo<SolutionsListing>,
    on_solution_search: EventHandler<String>,
    on_solution_change: EventHandler<String>,
}

#[component]
pub fn SolutionSelect(props: SolutionSelectProps) -> Element {
    let value = use_memo(move || (props.solution.read().id(), props.solution.read().name()));

    let options = props
        .solutions_listing
        .read()
        .fetch()
        .iter()
        .map(|solution| (solution.id(), solution.name()))
        .collect();

    rsx! {
        Row {
            Select {
                label: "Питательный раствор",
                placeholder: "выбрать раствор",
                value,
                options,
                on_search: props.on_solution_search,
                on_change: props.on_solution_change,
            }
        }
    }
}
