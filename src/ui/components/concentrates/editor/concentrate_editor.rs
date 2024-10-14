use super::{FromFertilizers, FromSolution};
use crate::controller::concentrates::{
    CompositionType, ConcentrateComposition, FertilizersBrowser, SolutionsBrowser,
};
use crate::model::chemistry::Volume;
use crate::model::concentrates::{Composition, Concentrate};
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::icons::Plus;
use crate::ui::components::utils::{
    Block, Button, ButtonsGroup, ButtonsGroupButton, Divider, TextField, Title,
};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ConcentrateEditorProps {
    concentrate: Memo<Concentrate>,
    concentrate_composition: Memo<ConcentrateComposition>,
    solutions_browser: Memo<SolutionsBrowser>,
    fertilizers_browser: Memo<FertilizersBrowser>,
    on_solution_search: EventHandler<String>,
    on_solution_change: EventHandler<String>,
    on_composition_type_change: EventHandler<CompositionType>,
    on_name_update: EventHandler<String>,
    on_part_add: EventHandler<()>,
    on_part_delete: EventHandler<String>,
    on_part_name_update: EventHandler<(String, String)>,
    on_part_concentration_update: EventHandler<(String, usize)>,
    on_part_volume_update: EventHandler<(String, Volume)>,
    on_fertilizer_amount_add: EventHandler<(String, String, f64)>,
    on_fertilizer_percent_distribute: EventHandler<(String, String, usize)>,
    on_fertilizer_delete: EventHandler<(String, String)>,
    on_save: EventHandler<()>,
}

#[component]
pub fn ConcentrateEditor(props: ConcentrateEditorProps) -> Element {
    rsx! {
        Block {
            Row {
                Title {
                    "Редактор концентрата",
                }
            }
        }

        Block {
            exclude_padding: "top",

            Column {
                Divider {}

                TextField {
                    label: "Название",
                    value: props.concentrate.read().name().clone(),
                    on_input: props.on_name_update,
                }

                Divider {}

                Column {
                    Row {
                        horizontal: "space-between",

                        Row {
                            Title {
                                size: "small",
                                "Резервуары",
                            }

                            Button {
                                style: "compact",
                                on_click: props.on_part_add,
                                Plus {},
                            }
                        }

                        ButtonsGroup {
                            value: props.concentrate_composition.read().composition_type().to_string(),
                            buttons: vec![
                                ButtonsGroupButton {
                                    label: CompositionType::FromFertilizers.label(),
                                    value: CompositionType::FromFertilizers.to_string(),
                                    badge: None,
                                },
                                ButtonsGroupButton {
                                    label: CompositionType::FromSolution.label(),
                                    value: CompositionType::FromSolution.to_string(),
                                    badge: None,
                                },
                            ],
                            on_change: move |tab_value| {
                                props.on_composition_type_change.call(CompositionType::from(tab_value));
                            },
                        }
                    }

                    match props.concentrate.read().composition().clone() {
                        Composition::FromFertilizers(composition) => rsx! {
                            FromFertilizers {
                                fertilizers_browser: props.fertilizers_browser,
                                composition: Signal::new(composition),
                                concentrate: props.concentrate,
                                on_part_name_update: props.on_part_name_update,
                                on_part_concentration_update: props.on_part_concentration_update,
                                on_part_volume_update: props.on_part_volume_update,
                                on_part_delete: props.on_part_delete,
                                on_fertilizer_amount_add: props.on_fertilizer_amount_add,
                                on_fertilizer_delete: props.on_fertilizer_delete,
                            }
                        },

                        Composition::FromSolution(composition) => rsx! {
                            FromSolution {
                                fertilizers_browser: props.fertilizers_browser,
                                composition: Signal::new(composition),
                                concentrate: props.concentrate,
                                solutions_browser: props.solutions_browser,
                                on_solution_search: props.on_solution_search,
                                on_solution_change: props.on_solution_change,
                                on_part_name_update: props.on_part_name_update,
                                on_part_concentration_update: props.on_part_concentration_update,
                                on_part_volume_update: props.on_part_volume_update,
                                on_part_delete: props.on_part_delete,
                                on_fertilizer_percent_distribute: props.on_fertilizer_percent_distribute,
                                on_fertilizer_delete: props.on_fertilizer_delete,
                            }
                        },
                    }
                }

                Divider {}

                Row {
                    horizontal: "end",

                    Button {
                        style: "primary",
                        on_click: props.on_save,
                        "Сохранить",
                    }
                }
            }
        }
    }
}
