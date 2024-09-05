use crate::model::chemistry::Volume;
use crate::ui::components::layout::{Column, Row};
use crate::ui::components::utils::icons::Close;
use crate::ui::components::utils::{Button, NumberField, TextField};
use crate::ui::components::VolumeField;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct PartSettingsProps {
    name: String,
    concentration: usize,
    volume: Volume,
    on_name_update: EventHandler<String>,
    on_concentration_update: EventHandler<usize>,
    on_volume_update: EventHandler<Volume>,
    on_delete: EventHandler<()>,
}

#[component]
pub fn PartSettings(props: PartSettingsProps) -> Element {
    let volume = use_memo(move || props.volume);

    rsx! {
        Column {
            gap: "medium",

            Row {
                TextField {
                    label: "Название",
                    value: props.name,
                    on_input: move |value| props.on_name_update.call(value),
                }

                Button {
                    style: "compact",
                    on_click: move |_| props.on_delete.call(()),
                    Close {},
                }
            }

            Row {
                NumberField {
                    label: "Концентрация",
                    value: props.concentration,
                    on_change: move |value| props.on_concentration_update.call(value),
                }

                VolumeField {
                    label: "Объем",
                    volume,
                    on_change: move |value| props.on_volume_update.call(value),
                }
            }
        }
    }
}
