use super::{MultiStage, SingleStage};
use crate::model::chemistry::NutrientAmount;
use crate::model::profiles::NutritionProgram;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct StagesListProps {
    nutrition_program: Memo<NutritionProgram>,
    on_stage_name_update: EventHandler<(String, String)>,
    on_nutrient_update: EventHandler<(String, NutrientAmount)>,
    on_stage_delete: EventHandler<String>,
}

#[component]
pub fn StagesList(props: StagesListProps) -> Element {
    let nutrition_program = props.nutrition_program.read();

    let stages = nutrition_program
        .stages()
        .iter()
        .map(|s| Signal::new(s.clone()));

    rsx! {
        match stages.len() {
            1 => rsx! {
                SingleStage {
                    nutrition_program: props.nutrition_program,
                    on_nutrient_update: move |nutrient_amount| {
                        props.on_nutrient_update.call((String::new(), nutrient_amount));
                    },
                }
            },
            _ => rsx! {
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
            },
        }
    }
}
