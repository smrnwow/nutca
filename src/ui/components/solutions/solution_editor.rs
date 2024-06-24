use crate::model::chemistry::Nutrient;
use crate::model::fertilizers::Fertilizer;
use crate::model::profiles::Profile;
use crate::model::solutions::Solution;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::profiles::ProfileForm;
use crate::ui::components::solutions::{
    FertilizersBrowser, FertilizersUsed, SolutionComposition, SolutionFertilizers,
};
use crate::ui::components::utils::{
    Block, Button, Card, Divider, NumberField, Select, Text, TextField, Title,
};
use dioxus::prelude::*;

fn round(value: f64) -> String {
    format!("{:.3}", value)
}

fn tab_class(tab_value: String, active_tab: String) -> String {
    if tab_value == active_tab {
        String::from("fertilizers-source__tab fertilizers-source__tab_active")
    } else {
        String::from("fertilizers-source__tab")
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct SolutionEditorProps {
    solution: Memo<Solution>,
    profile: Memo<Profile>,
    profiles: Memo<Vec<Profile>>,
    fertilizers: Memo<Vec<Fertilizer>>,
    selected_fertilizers: Memo<Vec<String>>,
    on_profile_change: EventHandler<String>,
    on_profile_search: EventHandler<String>,
    on_profile_nutrient_update: EventHandler<Nutrient>,
    on_fertilizer_select: EventHandler<String>,
    on_fertilizer_remove: EventHandler<String>,
    on_fertilizer_search: EventHandler<String>,
    on_volume_update: EventHandler<usize>,
    on_save: EventHandler<String>,
}

#[component]
pub fn SolutionEditor(props: SolutionEditorProps) -> Element {
    let mut profile_tab = use_signal(|| String::from("profile"));

    let mut solution_name = use_signal(|| props.solution.read().name());

    let selected_fertilizers = use_memo(move || props.solution.read().fertilizers());

    rsx! {
        Card {
            Block {
                Title {
                    text: "Редактор раствора",
                }
            }

            Divider {}

            /*
            Block {
                exclude_padding: "vertical",

                div {
                    class: "tabs",

                    div {
                        class: tab_class(String::from("profile"), active_tab.read().clone()),
                        onclick: move |_| {
                            *active_tab.write() = String::from("profile");
                        },
                        "Заполните профиль питания",
                    }

                    div {
                        class: tab_class(String::from("fertilizers"), active_tab.read().clone()),
                        onclick: move |_| {
                            *active_tab.write() = String::from("fertilizers");
                        },
                        "Выберите удобрения",
                    }
                }
            }
            */

            Block {
                exclude_padding: "bottom",

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
                        vertical: "center",

                        Text {
                            size: "x-small",
                            "~EC {round(props.solution.read().ec())}",
                        }

                        div {
                            class: "fertilizers-source__tabs",

                            button {
                                class: {tab_class(String::from("profile"), profile_tab.read().clone())},
                                onclick: move |_| {
                                    *profile_tab.write() = String::from("profile");
                                },
                                "Желаемый"
                            }

                            button {
                                class: {tab_class(String::from("solution-composition"), profile_tab.read().clone())},
                                onclick: move |_| {
                                    *profile_tab.write() = String::from("solution-composition");
                                },
                                "Рассчитанный"
                            }
                        }
                    }
                }
            }

            Block {
                Column {
                    Select {
                        placeholder: "выбрать готовый профиль",
                        value: (props.profile.read().id(), props.profile.read().name()),
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
                        fertilizers: props.fertilizers,
                        on_select: props.on_fertilizer_select,
                        on_search: props.on_fertilizer_search,
                    }

                    FertilizersUsed {
                        fertilizers_used: selected_fertilizers,
                        solution: props.solution,
                        on_remove: props.on_fertilizer_remove,
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
