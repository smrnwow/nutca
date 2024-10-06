use super::SolutionComposition;
use crate::controller::solutions::ProfilesBrowser;
use crate::model::chemistry::NutrientAmount;
use crate::model::solutions::Solution;
use crate::ui::components::chemistry::Nutrients;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::{
    Badge, ButtonsGroup, ButtonsGroupButton, SearchableSelect, Select, Text, Title,
};
use dioxus::prelude::*;

fn round(value: f64) -> String {
    format!("{:.3}", value)
}

#[derive(Props, PartialEq, Clone)]
pub struct SolutionProfileProps {
    solution: Memo<Solution>,
    profiles_browser: Memo<ProfilesBrowser>,
    on_profile_change: EventHandler<String>,
    on_profile_stage_change: EventHandler<String>,
    on_profile_search: EventHandler<String>,
    on_profile_nutrient_update: EventHandler<NutrientAmount>,
}

#[component]
pub fn SolutionProfile(props: SolutionProfileProps) -> Element {
    let mut profile_tab = use_signal(|| String::from("profile"));

    let profile_requirement = use_memo(move || props.solution.read().profile_requirement().clone());

    let nutrition_content = use_memo(move || props.solution.read().nutrition_content().clone());

    let diff = use_memo(move || props.solution.read().diff().clone());

    let nutrients = use_memo(move || profile_requirement.read().nutrients().clone());

    let profile = use_memo(move || match profile_requirement.read().profile() {
        Some((profile, _)) => (profile.id().to_string(), profile.name().to_string()),
        None => (String::new(), String::new()),
    });

    let stage = use_memo(move || match profile_requirement.read().profile() {
        Some((profile, stage_id)) => (
            stage_id.clone(),
            profile.stage(stage_id).unwrap().name().to_string(),
        ),
        None => (String::new(), String::new()),
    });

    let stages = match profile_requirement.read().profile() {
        Some((profile, _)) => profile
            .stages()
            .iter()
            .map(|stage| (stage.id().to_string(), stage.name().to_string()))
            .collect(),
        None => Vec::new(),
    };

    rsx! {
        Column {
            gap: "medium",

            Row {
                horizontal: "space-between",
                vertical: "center",

                Row {
                    Title {
                        size: "small",
                        "Программа питания",
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
                        value: profile_tab.read().clone(),
                        buttons: vec![
                            ButtonsGroupButton {
                                label: String::from("Желаемый"),
                                value: String::from("profile"),
                                badge: None
                            },
                            ButtonsGroupButton {
                                label: String::from("Рассчитанный"),
                                value: String::from("solution-composition"),
                                badge: rsx! {
                                    if !props.solution.read().is_empty() {
                                        Badge {
                                            size: "small",
                                            text: "!",
                                            state: "error",
                                        }
                                    }
                                }
                            },
                        ],
                        on_change: move |value| profile_tab.set(value),
                    }
                }
            }

            div {
                class: "profile-picker",

                SearchableSelect {
                    placeholder: "выбрать готовый профиль",
                    value: profile,
                    options: props.profiles_browser.read()
                        .fetch()
                        .iter()
                        .map(|profile| (profile.id().to_string(), profile.name().to_string()))
                        .collect(),
                    on_search: move |search_query| {
                        props.on_profile_search.call(search_query);
                    },
                    on_change: move |profile_id| {
                        props.on_profile_change.call(profile_id);
                    },
                }

                if stages.len() > 1 {
                    Select {
                        value: stage,
                        options: stages,
                        on_change: props.on_profile_stage_change,
                    }
                }
            }

            match profile_tab.read().as_str() {
                "profile" => rsx! {
                    Nutrients {
                        nutrients,
                        on_nutrient_update: props.on_profile_nutrient_update,
                    },
                },

                "solution-composition" => rsx! {
                    SolutionComposition {
                        nutrition_content,
                        diff,
                    },
                },

                _ => None,
            }
        }
    }
}
