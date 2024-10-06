use super::{MultiStage, SingleStage};
use crate::model::chemistry::NutrientAmount;
use crate::model::profiles::Profile;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::icons::Plus;
use crate::ui::components::utils::{Button, Title};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct StagesListProps {
    profile: Memo<Profile>,
    on_stage_add: EventHandler<()>,
    on_stage_name_update: EventHandler<(String, String)>,
    on_nutrient_update: EventHandler<(String, NutrientAmount)>,
    on_stage_delete: EventHandler<String>,
}

#[component]
pub fn StagesList(props: StagesListProps) -> Element {
    let profile = props.profile.read();

    let stages = profile.stages().iter().map(|s| Signal::new(s.clone()));

    rsx! {
        Row {
            Title {
                size: "small",
                "Стадии жизненного цикла растения",
            }

            Button {
                style: "compact",
                on_click: props.on_stage_add,
                Plus {},
            }
        }

        match stages.len() {
            1 => rsx! {
                SingleStage {
                    profile: props.profile,
                    on_nutrient_update: props.on_nutrient_update,
                }
            },
            _ => rsx! {
                Column {
                    gap: "x-large",

                    for stage in stages {
                        MultiStage {
                            stage: Memo::new(move || stage.read().clone()),
                            on_name_update: move |name| {
                                props.on_stage_name_update.call((stage.read().id().to_string(), name));
                            },
                            on_nutrient_update: move |nutrient_amount| {
                                props.on_nutrient_update.call((stage.read().id().to_string(), nutrient_amount));
                            },
                            on_delete: move |_| {
                                props.on_stage_delete.call(stage.read().id().to_string());
                            },
                        }
                    }
                }
            },
        }
    }
}
