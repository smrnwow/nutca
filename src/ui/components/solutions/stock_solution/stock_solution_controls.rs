use crate::repository::SolutionsListing;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::{NumberField, Select};
use crate::ui::components::VolumeField;
use dioxus::prelude::*;
use nutca::chemistry::Volume;
use nutca::solutions::Solution;

#[derive(Props, PartialEq, Clone)]
pub struct StockSolutionControlsProps {
    solution: Memo<Solution>,
    solutions_listing: Memo<SolutionsListing>,
    concentration: Memo<usize>,
    volume: Memo<Volume>,
    on_solution_search: EventHandler<String>,
    on_solution_change: EventHandler<String>,
    on_concentration_change: EventHandler<usize>,
    on_volume_change: EventHandler<Volume>,
}

#[component]
pub fn StockSolutionControls(props: StockSolutionControlsProps) -> Element {
    let value = use_memo(move || (props.solution.read().id(), props.solution.read().name()));

    let options = props
        .solutions_listing
        .read()
        .list()
        .iter()
        .map(|solution| (solution.id(), solution.name()))
        .collect();

    rsx! {
        Column {
            Select {
                label: "Питательный раствор",
                placeholder: "выбрать раствор",
                value,
                options,
                on_search: props.on_solution_search,
                on_change: props.on_solution_change,
            }

            Row {
                NumberField {
                    label: "Концентрация",
                    value: *props.concentration.read(),
                    on_change: props.on_concentration_change,
                }

                VolumeField {
                    label: "Объем",
                    volume: props.volume,
                    on_change: props.on_volume_change,
                }
            }
        }
    }
}
