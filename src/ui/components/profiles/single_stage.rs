use super::StageNutrients;
use crate::model::chemistry::NutrientAmount;
use crate::model::profiles::Profile;
use crate::ui::components::layout::Column;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SingleStageProps {
    profile: Memo<Profile>,
    on_nutrient_update: EventHandler<(String, NutrientAmount)>,
}

#[component]
pub fn SingleStage(props: SingleStageProps) -> Element {
    let stage = use_memo(move || props.profile.read().stages().last().unwrap().clone());

    let nutrients = stage.read().nutrients().clone();

    rsx! {
        Column {
            gap: "small",

            StageNutrients {
                nutrients,
                on_nutrient_update: move |nutrient_amount| {
                    props.on_nutrient_update.call((stage.read().id().to_string(), nutrient_amount));
                },
            }
        }
    }
}
