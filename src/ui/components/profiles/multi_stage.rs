use super::StageNutrients;
use crate::model::chemistry::NutrientAmount;
use crate::model::profiles::Stage;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::icons::Delete;
use crate::ui::components::utils::{Button, TextField};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct MultiStageProps {
    stage: Memo<Stage>,
    on_name_update: EventHandler<String>,
    on_nutrient_update: EventHandler<NutrientAmount>,
    on_delete: EventHandler<()>,
}

#[component]
pub fn MultiStage(props: MultiStageProps) -> Element {
    let nutrients = props.stage.read().nutrients().clone();

    rsx! {
        Column {
            gap: "small",

            Row {
                vertical: "end",

                TextField {
                    label: "Название стадии",
                    value: props.stage.read().name(),
                    on_input: move |value| props.on_name_update.call(value),
                }

                div {
                    class: "part-settings__delete",

                    Button {
                        style: "compact",
                        on_click: move |_| props.on_delete.call(()),
                        Delete {},
                    }
                }
            }

            StageNutrients {
                nutrients,
                on_nutrient_update: props.on_nutrient_update,
            }
        }
    }
}
