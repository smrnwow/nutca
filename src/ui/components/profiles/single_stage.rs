use super::StageNutrients;
use crate::model::chemistry::NutrientAmount;
use crate::model::profiles::NutritionProgram;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::Title;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SingleStageProps {
    nutrition_program: Memo<NutritionProgram>,
    on_nutrient_update: EventHandler<NutrientAmount>,
}

#[component]
pub fn SingleStage(props: SingleStageProps) -> Element {
    let stage = use_memo(move || {
        props
            .nutrition_program
            .read()
            .stages()
            .last()
            .unwrap()
            .clone()
    });

    let nutrients = use_memo(move || stage.read().nutrients().clone());

    rsx! {
        Column {
            gap: "small",

            Row {
                Title {
                    size: "small",
                    "Питательные элементы",
                }
            }

            StageNutrients {
                nutrients,
                on_nutrient_update: props.on_nutrient_update,
            }
        }
    }
}
