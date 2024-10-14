use super::{FertilizerAmountButton, FertilizerItem, FertilizerPercentButton, PartSettings};
use crate::controller::concentrates::FertilizersBrowser;
use crate::model::chemistry::Volume;
use crate::model::concentrates::{CompositionFromFertilizers, CompositionFromSolution, Part};
use crate::model::fertilizers::FertilizerAmount;
use crate::ui::components::layout::Column;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct PartEditorProps {
    fertilizers_browser: Memo<FertilizersBrowser>,
    composition_from_solution: Option<Signal<CompositionFromSolution>>,
    composition_from_fertilizers: Option<Signal<CompositionFromFertilizers>>,
    part: Signal<Part>,
    on_part_name_update: EventHandler<(String, String)>,
    on_part_concentration_update: EventHandler<(String, usize)>,
    on_part_volume_update: EventHandler<(String, Volume)>,
    on_part_delete: EventHandler<String>,
    on_fertilizer_delete: EventHandler<(String, String)>,
    on_fertilizer_amount_add: Option<EventHandler<(String, String, f64)>>,
    on_fertilizer_percent_distribute: Option<EventHandler<(String, String, usize)>>,
}

#[component]
pub fn PartEditor(props: PartEditorProps) -> Element {
    let part = props.part;

    let fertilizers: Vec<Signal<FertilizerAmount>> = match props.on_fertilizer_percent_distribute {
        Some(_) => {
            if let Some(composition_from_solution) = props.composition_from_solution {
                composition_from_solution
                    .read()
                    .fertilizers_by_part(&part.read())
                    .into_iter()
                    .map(|f| Signal::new(f))
                    .collect()
            } else {
                Vec::new()
            }
        }

        None => {
            if let Some(composition_from_fertilizers) = props.composition_from_fertilizers {
                composition_from_fertilizers
                    .read()
                    .fertilizers_by_part(&part.read())
                    .into_iter()
                    .cloned()
                    .map(|f| Signal::new(f))
                    .collect()
            } else {
                Vec::new()
            }
        }
    };

    rsx! {
        div {
            class: "part-editor",

            Column {
                gap: "x-small",

                PartSettings {
                    id: part.read().id().clone(),
                    name: part.read().name().clone(),
                    concentration: part.read().concentration().clone(),
                    volume: part.read().volume().clone(),
                    on_name_update: props.on_part_name_update,
                    on_concentration_update: props.on_part_concentration_update,
                    on_volume_update: props.on_part_volume_update,
                    on_delete: props.on_part_delete,
                }
            }

            Column {
                gap: "xx-small",

                for fertilizer_amount in fertilizers {
                    FertilizerItem {
                        key: "{fertilizer_amount.read().fertilizer().id()}",
                        fertilizer_amount,
                        on_delete: move |fertilizer_id| {
                            props.on_fertilizer_delete.call((part.read().id().clone(), fertilizer_id));
                        },
                    }
                }
            }

            if let Some(on_fertilizer_amount_add) = props.on_fertilizer_amount_add {
                FertilizerAmountButton {
                    fertilizers_browser: props.fertilizers_browser,
                    on_fertilizer_add: move |(fertilizer_id, amount)| {
                        let part_id = part.read().id().clone();

                        on_fertilizer_amount_add.call((part_id, fertilizer_id, amount));
                    },
                }
            }

            if let Some(on_fertilizer_percent_distribute) = props.on_fertilizer_percent_distribute {
                FertilizerPercentButton {
                    composition: props.composition_from_solution.unwrap(),
                    on_fertilizer_percent_update: move |(fertilizer_id, percent)| {
                        let part_id = part.read().id().clone();

                        on_fertilizer_percent_distribute.call((part_id, fertilizer_id, percent));
                    },
                }
            }
        }
    }
}
