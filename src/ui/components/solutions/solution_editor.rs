use crate::model::chemistry::Nutrient;
use crate::model::fertilizers::FertilizersListing;
use crate::model::profiles::Profile;
use crate::model::solutions::Solution;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::profiles::ProfileForm;
use crate::ui::components::solutions::{FertilizersBrowser, FertilizersUsed, SolutionComposition};
use crate::ui::components::utils::{
    Badge, Block, Button, ButtonsGroup, Card, Divider, Select, Text, TextField, Title,
};
use dioxus::prelude::*;

fn round(value: f64) -> String {
    format!("{:.3}", value)
}

fn tab_class(tab_value: String, active_tab: String) -> String {
    if tab_value == active_tab {
        String::from("buttons-group__button buttons-group__button_size-small buttons-group__button buttons-group__button_size-small buttons-group__button_active")
    } else {
        String::from("buttons-group__button buttons-group__button_size-small ")
    }
}

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
    let mut profile_tab = use_signal(|| String::from("profile"));

    let mut solution_name = use_signal(|| props.solution.read().name());

    let profile_select_value =
        use_memo(move || (props.profile.read().id(), props.profile.read().name()));

    rsx! {
        Card {
            Block {
                Title {
                    text: "Редактор раствора",
                }
            }

            Divider {}

            Block {
                Column {
                    gap: "medium",

                    Row {
                        horizontal: "space-between",
                        vertical: "center",

                        Row {
                            Title {
                                size: "small",
                                text: "Профиль питания",
                            }
                        }

                        Row {
                            horizontal: "end",
                            vertical: "center",

                            Text {
                                size: "x-small",
                                "~EC {round(props.solution.read().ec())}",
                            }

                            ButtonsGroup {
                                button {
                                    class: tab_class(String::from("profile"), profile_tab.read().clone()),
                                    onclick: move |_| {
                                        *profile_tab.write() = String::from("profile");
                                    },
                                    "Желаемый",
                                }


                                button {
                                    class: tab_class(String::from("solution-composition"), profile_tab.read().clone()),
                                    onclick: move |_| {
                                        *profile_tab.write() = String::from("solution-composition");
                                    },
                                    "Рассчитанный",

                                    if !props.solution.read().is_empty() {
                                        Badge {
                                            size: "small",
                                            text: "!",
                                            state: "error",
                                        }
                                    }
                                }
                            }
                        }
                    }

                    Select {
                        placeholder: "выбрать готовый профиль",
                        value: profile_select_value,
                        options: props.profiles.read()
                            .iter()
                            .map(|profile| (profile.id(), profile.name()))
                            .collect(),
                        on_search: move |search_query| {
                            props.on_profile_search.call(search_query);
                        },
                        on_change: move |profile_id| {
                            props.on_profile_change.call(profile_id);
                        },
                    }

                    if profile_tab.read().clone() == String::from("profile") {
                        ProfileForm {
                            profile: props.profile,
                            on_nutrient_update: props.on_profile_nutrient_update,
                        }
                    }

                    if profile_tab.read().clone() == String::from("solution-composition") {
                        SolutionComposition {
                            solution: props.solution,
                        }
                    }
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

                    FertilizersUsed {
                        solution: props.solution,
                        on_exclude: props.on_fertilizer_exclude,
                        on_volume_update: props.on_volume_update,
                    }
                }
            }

            Divider {}

            Block {
                Row {
                    TextField {
                        placeholder: "название раствора",
                        value: solution_name.read(),
                        on_input: move |event| {
                            *solution_name.write() = event;
                        },
                    }

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
