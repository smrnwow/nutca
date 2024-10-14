use super::{PartsList, SolutionSelect};
use crate::controller::concentrates::{FertilizersBrowser, SolutionsBrowser};
use crate::model::chemistry::Volume;
use crate::model::concentrates::{CompositionFromSolution, Concentrate};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FromSolutionProps {
    fertilizers_browser: Memo<FertilizersBrowser>,
    solutions_browser: Memo<SolutionsBrowser>,
    composition: Signal<CompositionFromSolution>,
    concentrate: Memo<Concentrate>,
    on_solution_search: EventHandler<String>,
    on_solution_change: EventHandler<String>,
    on_part_name_update: EventHandler<(String, String)>,
    on_part_concentration_update: EventHandler<(String, usize)>,
    on_part_volume_update: EventHandler<(String, Volume)>,
    on_part_delete: EventHandler<String>,
    on_fertilizer_delete: EventHandler<(String, String)>,
    on_fertilizer_percent_distribute: EventHandler<(String, String, usize)>,
}

#[component]
pub fn FromSolution(props: FromSolutionProps) -> Element {
    let solution = use_memo(move || props.composition.read().solution().clone());

    rsx! {
        SolutionSelect {
            solution,
            solutions_browser: props.solutions_browser,
            on_solution_search: props.on_solution_search,
            on_solution_change: props.on_solution_change,
        }

        PartsList {
            fertilizers_browser: props.fertilizers_browser,
            composition_from_solution: props.composition,
            concentrate: props.concentrate,
            on_part_name_update: props.on_part_name_update,
            on_part_concentration_update: props.on_part_concentration_update,
            on_part_volume_update: props.on_part_volume_update,
            on_part_delete: props.on_part_delete,
            on_fertilizer_delete: props.on_fertilizer_delete,
            on_fertilizer_percent_distribute: props.on_fertilizer_percent_distribute,
        }
    }
}
