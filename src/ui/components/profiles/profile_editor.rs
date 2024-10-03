use super::StagesList;
use crate::controller::Validation;
use crate::model::chemistry::NutrientAmount;
use crate::model::profiles::{NutritionProgram, Profile};
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::{Block, Button, Divider, TextField, Title};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProfileEditorProps {
    profile: Memo<Profile>,
    nutrition_program: Memo<NutritionProgram>,
    validation: Memo<Validation>,
    on_name_update: EventHandler<String>,
    on_nutrient_update: EventHandler<(String, NutrientAmount)>,
    on_stage_name_update: EventHandler<(String, String)>,
    on_stage_delete: EventHandler<String>,
    on_save: EventHandler<()>,
    on_cancel: EventHandler<()>,
}

#[component]
pub fn ProfileEditor(props: ProfileEditorProps) -> Element {
    rsx! {
        Block {
            Row {
                Title {
                    "Редактор программы питания",
                }
            }
        }

        Block {
            exclude_padding: "top",

            Column {
                Divider {}

                TextField {
                    label: "Название",
                    value: props.nutrition_program.read().name(),
                    error: props.validation.read().get("profile-name"),
                    on_input: props.on_name_update,
                }

                Divider {}

                StagesList {
                    nutrition_program: props.nutrition_program,
                    on_stage_name_update: props.on_stage_name_update,
                    on_nutrient_update: props.on_nutrient_update,
                    on_stage_delete: props.on_stage_delete,
                }

                /*
                ProfileNutrients {
                    profile: props.profile,
                    on_nutrient_update: props.on_nutrient_update,
                }
                */

                Divider {}

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
}
