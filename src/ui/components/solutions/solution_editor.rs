use crate::model::chemistry::Nutrient;
use crate::model::fertilizers::FertilizersListing;
use crate::model::profiles::Profile;
use crate::model::solutions::{Solution, SolutionError};
use crate::ui::components::layout::Row;
use crate::ui::components::solutions::{FertilizersBrowser, FertilizersSet, SolutionProfile};
use crate::ui::components::utils::{Block, Button, Card, Divider, TextField, Title};
use crate::ui::components::ReferenceSubject;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SolutionEditorProps {
    solution: Memo<Solution>,
    solution_error: Memo<SolutionError>,
    profile: Memo<Profile>,
    profiles: Memo<Vec<Profile>>,
    fertilizers_listing: Signal<FertilizersListing>,
    on_name_update: EventHandler<String>,
    on_volume_update: EventHandler<usize>,
    on_profile_change: EventHandler<String>,
    on_profile_search: EventHandler<String>,
    on_profile_nutrient_update: EventHandler<Nutrient>,
    on_fertilizer_select: EventHandler<String>,
    on_fertilizer_exclude: EventHandler<String>,
    on_fertilizer_search: EventHandler<String>,
    on_fertilizers_paginate: EventHandler<usize>,
    on_save: EventHandler<()>,
}

#[component]
pub fn SolutionEditor(props: SolutionEditorProps) -> Element {
    rsx! {
        Card {
            Block {
                Row {
                    gap: "x-small",
                    vertical: "center",

                    Title {
                        "Редактор раствора",
                    }
                }
            }

            Divider {}

            Block {
                TextField {
                    label: "Название",
                    value: props.solution.read().name(),
                    error: props.solution_error.read().name(),
                    on_input: props.on_name_update,
                }
            }

            Divider {}

            ReferenceSubject {
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
            }

            Divider {}

            Block {
                Row {
                    ReferenceSubject {
                        FertilizersBrowser {
                            fertilizers_listing: props.fertilizers_listing,
                            on_select: props.on_fertilizer_select,
                            on_search: props.on_fertilizer_search,
                            on_paginate: props.on_fertilizers_paginate,
                        }
                    }

                    ReferenceSubject {
                        FertilizersSet {
                            solution: props.solution,
                            on_exclude: props.on_fertilizer_exclude,
                            on_volume_update: props.on_volume_update,
                        }
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
