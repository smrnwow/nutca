use crate::ui::components::layout::{Column, Row};
use crate::ui::components::profiles::ProfileForm;
use crate::ui::components::utils::Title;
use dioxus::prelude::*;
use nutca::chemistry::NutrientAmount;
use nutca::profiles::Profile;

#[derive(Props, PartialEq, Clone)]
pub struct ProfileNutrientsProps {
    profile: Memo<Profile>,
    on_nutrient_update: EventHandler<NutrientAmount>,
}

#[component]
pub fn ProfileNutrients(props: ProfileNutrientsProps) -> Element {
    rsx! {
        Column {
            gap: "small",

            Row {
                Title {
                    size: "small",
                    "Питательные элементы",
                }
            }

            ProfileForm {
                profile: props.profile,
                on_nutrient_update: props.on_nutrient_update,
            }
        }
    }
}
