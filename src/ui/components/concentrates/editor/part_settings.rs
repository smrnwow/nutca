use crate::model::chemistry::Volume;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::icons::Delete;
use crate::ui::components::utils::{Button, NumberField, TextField};
use crate::ui::components::VolumeField;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct PartSettingsProps {
    id: String,
    name: String,
    concentration: usize,
    volume: Volume,
    on_name_update: EventHandler<(String, String)>,
    on_concentration_update: EventHandler<(String, usize)>,
    on_volume_update: EventHandler<(String, Volume)>,
    on_delete: EventHandler<String>,
}

#[component]
pub fn PartSettings(props: PartSettingsProps) -> Element {
    let id = use_signal(|| props.id);

    let volume = use_memo(move || props.volume);

    rsx! {
        Column {
            gap: "medium",

            Row {
                vertical: "end",

                TextField {
                    label: "Название",
                    value: props.name,
                    on_input: move |value| {
                        props.on_name_update.call((id.read().clone(), value))
                    },
                }

                div {
                    class: "part-settings__delete",

                    Button {
                        style: "compact",
                        on_click: move |_| props.on_delete.call(id.read().clone()),
                        Delete {},
                    }
                }
            }

            Row {
                NumberField {
                    label: "Концентрация",
                    value: props.concentration,
                    on_change: move |value| {
                        props.on_concentration_update.call((id.read().clone(), value));
                    },
                }

                VolumeField {
                    label: "Объем",
                    volume,
                    on_change: move |value| {
                        props.on_volume_update.call((id.read().clone(), value));
                    },
                }
            }
        }
    }
}
