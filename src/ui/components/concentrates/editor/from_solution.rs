use super::{FertilizerItem, FertilizerPercentButton, PartSettings, SolutionSelect};
use crate::controller::concentrates::SolutionsBrowser;
use crate::model::chemistry::Volume;
use crate::model::concentrates::{CompositionFromSolution, Concentrate};
use crate::ui::components::layout::{Column, Row};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FromSolutionProps {
    concentrate: Memo<Concentrate>,
    composition: Signal<CompositionFromSolution>,
    solutions_browser: Memo<SolutionsBrowser>,
    on_solution_search: EventHandler<String>,
    on_solution_change: EventHandler<String>,
    on_part_name_update: EventHandler<(String, String)>,
    on_part_concentration_update: EventHandler<(String, usize)>,
    on_part_volume_update: EventHandler<(String, Volume)>,
    on_part_remove: EventHandler<String>,
    on_fertilizer_percent_update: EventHandler<(String, String, usize)>,
    on_fertilizer_remove: EventHandler<(String, String)>,
}

#[component]
pub fn FromSolution(props: FromSolutionProps) -> Element {
    rsx! {
        SolutionSelect {
            composition: props.composition,
            solutions_browser: props.solutions_browser,
            on_solution_search: props.on_solution_search,
            on_solution_change: props.on_solution_change,
        }

        div {
            class: "concentrate__parts",

            Row {
                for part in props.concentrate.read().parts().into_iter().cloned().map(|p| Signal::new(p)) {
                    div {
                        class: "part-editor",

                        Column {
                            gap: "x-small",

                            PartSettings {
                                name: part.read().name().clone(),
                                concentration: part.read().concentration().clone(),
                                volume: part.read().volume().clone(),
                                on_name_update: move |name| {
                                    props.on_part_name_update.call((part.read().id().clone(), name));
                                },
                                on_concentration_update: move |concentration| {
                                    props.on_part_concentration_update.call((part.read().id().clone(), concentration));
                                },
                                on_volume_update: move |volume| {
                                    props.on_part_volume_update.call((part.read().id().clone(), volume));
                                },
                                on_delete: move |_| props.on_part_remove.call(part.read().id().clone()),
                            }
                        }

                        Column {
                            gap: "xx-small",

                            for fertilizer in props.composition.read().fertilizers_by_part(&part.read()).into_iter().map(|f| Signal::new(f)) {
                                FertilizerItem {
                                    key: "{fertilizer.read().id()}",
                                    fertilizer,
                                    on_delete: move |fertilizer_id| {
                                        let part_id = part.read().id().clone();

                                        props.on_fertilizer_remove.call((part_id, fertilizer_id));
                                    },
                                }
                            }
                        }

                        FertilizerPercentButton {
                            composition: props.composition,
                            on_fertilizer_percent_update: move |(fertilizer_id, percent)| {
                                let part_id = part.read().id().clone();

                                props.on_fertilizer_percent_update.call((part_id, fertilizer_id, percent));
                            },
                        }
                    }
                }
            }
        }
    }
}
