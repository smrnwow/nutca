use super::{auto_filler::AutoFiller, manual_filler::ManualFiller};
use crate::controller::concentrates::{FertilizersBrowser, FertilizersStack};
use crate::controller::solutions::SolutionsListing;
use crate::model::chemistry::Volume;
use crate::model::concentrates::fillers::{AutoFiller, FillerVariant, ManualFiller};
use crate::model::concentrates::Concentrate;
use crate::model::solutions::Solution;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::icons::Plus;
use crate::ui::components::utils::{
    Block, Button, ButtonsGroup, ButtonsGroupButton, Card, Divider, TextField, Title,
};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ConcentrateEditorProps {
    fertilizers_stack: Memo<FertilizersStack>,
    fertilizers_browser: Memo<FertilizersBrowser>,
    concentrate: Memo<Concentrate>,
    filler_variant: Memo<FillerVariant>,
    auto_filler: Memo<AutoFiller>,
    manual_filler: Memo<ManualFiller>,
    solution: Memo<Solution>,
    solutions_listing: Memo<SolutionsListing>,
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
                        "Редактор рабочего раствора",
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
                                "Части",
                            }

                            Button {
                                style: "compact",
                                on_click: props.on_part_add,
                                Plus {},
                            }
                        }

                        ButtonsGroup {
                            value: props.filler_variant.read().to_string(),
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

                    match *props.filler_variant.read() {
                        FillerVariant::Auto => rsx! {
                            AutoFiller {
                                solution: props.solution,
                                solutions_listing: props.solutions_listing,
                                fertilizers_stack: props.fertilizers_stack,
                                on_solution_search: props.on_solution_search,
                                on_solution_change: props.on_solution_search,
                                auto_filler: props.auto_filler,
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
                                manual_filler: props.manual_filler,
                                solution: props.solution,
                                solutions_listing: props.solutions_listing,
                                on_solution_search: props.on_solution_search,
                                on_solution_change: props.on_solution_search,
                                fertilizers_browser: props.fertilizers_browser,
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
