use super::{AutoPart, SolutionSelect};
use crate::controller::concentrates::FertilizersStack;
use crate::controller::solutions::SolutionsListing;
use crate::model::chemistry::Volume;
use crate::model::concentrates::fillers::AutoFiller;
use crate::model::solutions::Solution;
use crate::ui::components::layout::Row;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct AutoFillerProps {
    auto_filler: Memo<AutoFiller>,
    solution: Memo<Solution>,
    solutions_listing: Memo<SolutionsListing>,
    fertilizers_stack: Memo<FertilizersStack>,
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
    rsx! {
        SolutionSelect {
            solution: props.solution,
            solutions_listing: props.solutions_listing,
            on_solution_search: props.on_solution_search,
            on_solution_change: props.on_solution_search,
        }

        div {
            class: "concentrate__parts",

            Row {
                for auto_part in props.auto_filler.read().parts().iter().cloned().map(|part| Signal::new(part)) {
                    AutoPart {
                        auto_part,
                        fertilizers_stack: props.fertilizers_stack,
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
