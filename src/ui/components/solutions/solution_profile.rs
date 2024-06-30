use crate::model::chemistry::Nutrient;
use crate::model::profiles::Profile;
use crate::model::solutions::Solution;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::profiles::ProfileForm;
use crate::ui::components::solutions::SolutionComposition;
use crate::ui::components::utils::{Badge, ButtonsGroup, Reference, Select, Text, Title};
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
pub struct SolutionProfileProps {
    solution: Memo<Solution>,
    profile: Memo<Profile>,
    profiles: Memo<Vec<Profile>>,
    on_profile_change: EventHandler<String>,
    on_profile_search: EventHandler<String>,
    on_profile_nutrient_update: EventHandler<Nutrient>,
}

#[component]
pub fn SolutionProfile(props: SolutionProfileProps) -> Element {
    let mut profile_tab = use_signal(|| String::from("profile"));

    let profile_select_value =
        use_memo(move || (props.profile.read().id(), props.profile.read().name()));

    let mut show_reference = use_signal(|| false);

    rsx! {
        Column {
            gap: "medium",
            on_hover: move |hovered| show_reference.set(hovered),

            Row {
                horizontal: "space-between",
                vertical: "center",

                Row {
                    Title {
                        size: "small",

                        "Профиль питания",

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
}
