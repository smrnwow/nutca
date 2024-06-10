use super::ProfileForm;
use crate::model::profiles::{Component, Profile};
use crate::ui::components::utils::{Block, Button, Card, Divider, TextField, Title};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProfileEditorWorkspaceProps {
    profile: Memo<Profile>,
    on_name_update: EventHandler<String>,
    on_component_update: EventHandler<Component>,
    on_save: EventHandler<()>,
    on_cancel: EventHandler<()>,
}

#[component]
pub fn ProfileEditorWorkspace(props: ProfileEditorWorkspaceProps) -> Element {
    rsx! {
        Card {
            Block {
                Title {
                    text: "Редактор профиля питания",
                }
            }

            Divider {}

            Block {
                div {
                    class: "profile-editor-page__name",

                    TextField {
                        value: props.profile.read().name(),
                        label: "Название",
                        on_input: props.on_name_update,
                    }
                }
            }

            Divider {}

            Block {
                ProfileForm {
                    profile: props.profile,
                    on_component_update: props.on_component_update,
                }
            }

            Divider {}

            Block {
                div {
                    class: "profile-editor-page__controls",

                    Button {
                        style: "stroke",
                        text: "Сбросить",
                        on_click: props.on_cancel,
                    }

                    Button {
                        style: "primary",
                        text: "Сохранить",
                        on_click: props.on_save,
                    }
                }
            }
        }
    }
}
