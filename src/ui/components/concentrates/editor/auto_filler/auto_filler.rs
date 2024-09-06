use super::{AutoPart, SolutionSelect};
use crate::controller::concentrates::{SolutionsBrowser, TanksSet};
use crate::model::chemistry::Volume;
use crate::ui::components::layout::Row;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct AutoFillerProps {
    tanks_set: Memo<TanksSet>,
    solutions_browser: Memo<SolutionsBrowser>,
    on_solution_search: EventHandler<String>,
    on_solution_change: EventHandler<String>,
    on_part_name_update: EventHandler<(String, String)>,
    on_part_concentration_update: EventHandler<(String, usize)>,
    on_part_volume_update: EventHandler<(String, Volume)>,
    on_part_delete: EventHandler<String>,
    on_part_fertilizer_add: EventHandler<(String, String, f64)>,
    on_part_fertilizer_delete: EventHandler<(String, String)>,
}

#[component]
pub fn AutoFiller(props: AutoFillerProps) -> Element {
    let fertilizers_stack = use_memo(move || props.tanks_set.read().fertilizers_stack().clone());

    rsx! {
        SolutionSelect {
            solutions_browser: props.solutions_browser,
            on_solution_search: props.on_solution_search,
            on_solution_change: props.on_solution_change,
        }

        div {
            class: "concentrate__parts",

            Row {
                for auto_part in props.tanks_set.read().auto_parts() {
                    AutoPart {
                        auto_part,
                        fertilizers_stack,
                        on_name_update: props.on_part_name_update,
                        on_concentration_update: props.on_part_concentration_update,
                        on_volume_update: props.on_part_volume_update,
                        on_delete: props.on_part_delete,
                        on_fertilizer_delete: props.on_part_fertilizer_delete,
                        on_fertilizer_add: props.on_part_fertilizer_add,
                    }
                }
            }
        }
    }
}
