use super::StagesList;
use crate::controller::profiles::ProfileValidator;
use crate::model::chemistry::NutrientAmount;
use crate::model::profiles::Profile;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::{Block, Button, Divider, TextField, Title};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProfileEditorProps {
    profile: Memo<Profile>,
    profile_validator: Memo<ProfileValidator>,
    on_name_update: EventHandler<String>,
    on_stage_add: EventHandler<()>,
    on_stage_name_update: EventHandler<(String, String)>,
    on_nutrient_update: EventHandler<(String, NutrientAmount)>,
    on_stage_delete: EventHandler<String>,
    on_save: EventHandler<()>,
}

#[component]
pub fn ProfileEditor(props: ProfileEditorProps) -> Element {
    rsx! {
        Block {
            exclude_padding: "bottom",

            Column {
                Row {
                    Title {
                        "Редактор программы питания",
                    }
                }

                Divider {}
            }
        }

        Block {
            Column {
                Row {
                    Title {
                        size: "small",
                        "О программе",
                    }
                }

                TextField {
                    label: "Название",
                    value: props.profile.read().name(),
                    error: props.profile_validator.read().name(),
                    on_input: props.on_name_update,
                }

                Divider {}

                StagesList {
                    profile: props.profile,
                    on_stage_add: props.on_stage_add,
                    on_stage_name_update: props.on_stage_name_update,
                    on_nutrient_update: props.on_nutrient_update,
                    on_stage_delete: props.on_stage_delete,
                }

                Divider {}

                Row {
                    horizontal: "end",

                    Button {
                        style: "primary",
                        on_click: props.on_save,
                        "Сохранить",
                    }
                }
            }
        }
    }
}
