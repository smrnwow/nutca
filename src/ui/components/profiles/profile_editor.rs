use crate::controller::Validation;
use crate::model::chemistry::NutrientAmount;
use crate::model::profiles::Profile;
use crate::ui::components::layout::Row;
use crate::ui::components::profiles::ProfileNutrients;
use crate::ui::components::utils::{Block, Button, Divider, TextField, Title};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProfileEditorProps {
    profile: Memo<Profile>,
    validation: Memo<Validation>,
    on_name_update: EventHandler<String>,
    on_nutrient_update: EventHandler<NutrientAmount>,
    on_save: EventHandler<()>,
    on_cancel: EventHandler<()>,
}

#[component]
pub fn ProfileEditor(props: ProfileEditorProps) -> Element {
    rsx! {
        Block {
            Row {
                Title {
                    "Редактор питательного состава",
                }
            }
        }

        Divider {}

        Block {
            TextField {
                label: "Название",
                value: props.profile.read().name(),
                error: props.validation.read().get("profile-name"),
                on_input: props.on_name_update,
            }
        }

        Divider {}

        Block {
            ProfileNutrients {
                profile: props.profile,
                on_nutrient_update: props.on_nutrient_update,
            }
        }

        Divider {}

        Block {
            Row {
                horizontal: "end",

                Button {
                    style: "stroke",
                    on_click: props.on_cancel,
                    "Сбросить",
                }

                Button {
                    style: "primary",
                    on_click: props.on_save,
                    "Сохранить",
                }
            }
        }
    }
}
