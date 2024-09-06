use super::{auto_filler::AutoFiller, manual_filler::ManualFiller};
use crate::controller::concentrates::{SolutionsBrowser, TanksSet};
use crate::model::chemistry::Volume;
use crate::model::concentrates::fillers::FillerVariant;
use crate::model::concentrates::Concentrate;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::icons::Plus;
use crate::ui::components::utils::{
    Block, Button, ButtonsGroup, ButtonsGroupButton, Card, Divider, TextField, Title,
};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ConcentrateEditorProps {
    solutions_browser: Memo<SolutionsBrowser>,
    tanks_set: Memo<TanksSet>,
    concentrate: Memo<Concentrate>,
    on_solution_search: EventHandler<String>,
    on_solution_change: EventHandler<String>,
    on_filler_variant_change: EventHandler<FillerVariant>,
    on_name_update: EventHandler<String>,
    on_part_add: EventHandler<()>,
    on_part_delete: EventHandler<String>,
    on_part_name_update: EventHandler<(String, String)>,
    on_part_concentration_update: EventHandler<(String, usize)>,
    on_part_volume_update: EventHandler<(String, Volume)>,
    on_part_fertilizer_add: EventHandler<(String, String, f64)>,
    on_part_fertilizer_delete: EventHandler<(String, String)>,
    on_save: EventHandler<()>,
}

#[component]
pub fn ConcentrateEditor(props: ConcentrateEditorProps) -> Element {
    rsx! {
        Card {
            Block {
                Row {
                    Title {
                        "Редактор концентрата",
                    }
                }
            }

            Divider {}

            Block {
                TextField {
                    label: "Название",
                    value: props.concentrate.read().name(),
                    on_input: props.on_name_update,
                }
            }

            Divider {}

            Block {
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
                            value: props.tanks_set.read().filler_variant().to_string(),
                            buttons: vec![
                                ButtonsGroupButton {
                                    label: String::from("Из раствора"),
                                    value: FillerVariant::Auto.to_string(),
                                    badge: None,
                                },
                                ButtonsGroupButton {
                                    label: String::from("Из удобрений"),
                                    value: FillerVariant::Manual.to_string(),
                                    badge: None,
                                },
                            ],
                            on_change: move |tab_value| props.on_filler_variant_change.call(FillerVariant::from(tab_value)),
                        }
                    }

                    match props.tanks_set.read().filler_variant() {
                        FillerVariant::Auto => rsx! {
                            AutoFiller {
                                tanks_set: props.tanks_set,
                                solutions_browser: props.solutions_browser,
                                on_solution_search: props.on_solution_search,
                                on_solution_change: props.on_solution_change,
                                on_part_name_update: props.on_part_name_update,
                                on_part_concentration_update: props.on_part_concentration_update,
                                on_part_volume_update: props.on_part_volume_update,
                                on_part_delete: props.on_part_delete,
                                on_part_fertilizer_add: props.on_part_fertilizer_add,
                                on_part_fertilizer_delete: props.on_part_fertilizer_delete,
                            }
                        },
                        FillerVariant::Manual => rsx! {
                            ManualFiller {
                                tanks_set: props.tanks_set,
                                on_part_name_update: props.on_part_name_update,
                                on_part_concentration_update: props.on_part_concentration_update,
                                on_part_volume_update: props.on_part_volume_update,
                                on_part_delete: props.on_part_delete,
                                on_part_fertilizer_add: props.on_part_fertilizer_add,
                                on_part_fertilizer_delete: props.on_part_fertilizer_delete,
                            }
                        },
                    }
                }
            }

            Divider {}

            Block {
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
