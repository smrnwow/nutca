use super::StageNutrients;
use crate::model::chemistry::NutrientAmount;
use crate::model::profiles::Profile;
use crate::ui::components::layout::Column;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SingleStageProps {
    profile: Memo<Profile>,
    on_nutrient_update: EventHandler<NutrientAmount>,
}

#[component]
pub fn SingleStage(props: SingleStageProps) -> Element {
    let stage = use_memo(move || props.profile.read().stages().last().unwrap().clone());

    let nutrients = use_memo(move || stage.read().nutrients().clone());

    rsx! {
        Column {
            gap: "small",

            /*
            Row {
                Title {
                    size: "small",
                    "Питательные элементы",
                }
            }
            */

            StageNutrients {
                nutrients,
                on_nutrient_update: props.on_nutrient_update,
            }
        }
    }
}
