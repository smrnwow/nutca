use crate::model::chemistry::Nutrient;
use crate::model::fertilizers::FertilizersListing;
use crate::model::profiles::Profile;
use crate::model::solutions::Solution;
use crate::ui::components::layout::Row;
use crate::ui::components::solutions::{FertilizersBrowser, FertilizersSet, SolutionProfile};
use crate::ui::components::utils::{
    Block, Button, Card, Divider, Reference, Text, TextField, Title,
};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SolutionEditorProps {
    solution: Memo<Solution>,
    profile: Memo<Profile>,
    profiles: Memo<Vec<Profile>>,
    fertilizers_listing: Signal<FertilizersListing>,
    on_profile_change: EventHandler<String>,
    on_profile_search: EventHandler<String>,
    on_profile_nutrient_update: EventHandler<Nutrient>,
    on_fertilizer_select: EventHandler<String>,
    on_fertilizer_exclude: EventHandler<String>,
    on_fertilizer_search: EventHandler<String>,
    on_fertilizers_paginate: EventHandler<usize>,
    on_volume_update: EventHandler<usize>,
    on_save: EventHandler<String>,
}

#[component]
pub fn SolutionEditor(props: SolutionEditorProps) -> Element {
    let mut solution_name = use_signal(|| props.solution.read().name());

    let mut show_reference = use_signal(|| false);

    rsx! {
        Card {
            on_hover: move |hovered| show_reference.set(hovered),
            Block {
                Row {
                    gap: "x-small",
                    vertical: "center",

                    Title {
                        "Редактор раствора",

                        Reference {
                            display: show_reference,
                            style: "badge",
                            tooltip: rsx! {
                                Title {
                                    size: "x-small",
                                    "Пока не придуманый заголовок",
                                }

                                Text {
                                    size: "x-small",
                                    "Еще не придуманный текст. Еще не придуманный текст. Еще не придуманный текст.",
                                }
                            },
                            tooltip_position: "top-center",
                        },
                    }
                }
            }

            Divider {}

            Block {
                TextField {
                    label: "Название",
                    value: solution_name.read(),
                    on_input: move |event| {
                        *solution_name.write() = event;
                    },
                }
            }

            Divider {}

            Block {
                SolutionProfile {
                    solution: props.solution,
                    profile: props.profile,
                    profiles: props.profiles,
                    on_profile_change: props.on_profile_change,
                    on_profile_search: props.on_profile_search,
                    on_profile_nutrient_update: props.on_profile_nutrient_update,
                }
            }

            Divider {}

            Block {
                Row {
                    FertilizersBrowser {
                        fertilizers_listing: props.fertilizers_listing,
                        on_select: props.on_fertilizer_select,
                        on_search: props.on_fertilizer_search,
                        on_paginate: props.on_fertilizers_paginate,
                    }

                    FertilizersSet {
                        solution: props.solution,
                        on_exclude: props.on_fertilizer_exclude,
                        on_volume_update: props.on_volume_update,
                    }
                }
            }

            Divider {}

            Block {
                Row {
                    horizontal: "end",

                    Button {
                        style: "primary",
                        on_click: move |_| {
                            props.on_save.call(solution_name.read().clone());

                            *solution_name.write() = String::new();
                        },
                        "Сохранить",
                    }
                }
            }
        }
    }
}
