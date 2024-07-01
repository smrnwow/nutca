use crate::model::chemistry::Nutrient;
use crate::model::profiles::Profile;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::profiles::ProfileForm;
use crate::ui::components::utils::Title;
use crate::ui::components::ReferencePreview;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProfileNutrientsProps {
    profile: Memo<Profile>,
    on_nutrient_update: EventHandler<Nutrient>,
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
                    ReferencePreview {
                        show_reference: Signal::new(false),
                        article_id: "profile-editor-nutrients",
                    }
                }
            }

            ProfileForm {
                profile: props.profile,
                on_nutrient_update: props.on_nutrient_update,
            }
        }
    }
}
