use crate::ui::components::layout::Row;
use crate::ui::components::utils::{FloatField, Label, Select};
use dioxus::prelude::*;
use crate::model::chemistry::{Volume, VolumeUnits};

fn round(value: f64) -> f64 {
    format!("{:.2}", value).parse().unwrap()
}

#[derive(Props, PartialEq, Clone)]
pub struct VolumeFieldProps {
    volume: Memo<Volume>,
    label: String,
    on_change: EventHandler<Volume>,
}

#[component]
pub fn VolumeField(props: VolumeFieldProps) -> Element {
    let volume = *props.volume.read();

    let options = use_signal(|| {
        vec![
            (VolumeUnits::Litres.into(), VolumeUnits::Litres.label()),
            (VolumeUnits::Gallons.into(), VolumeUnits::Gallons.label()),
        ]
    });

    let value = use_memo(move || {
        (
            props.volume.read().units().into(),
            props.volume.read().units().label(),
        )
    });

    rsx! {
        Row {
            gap: "small",
            vertical: "end",

            Label {
                text: props.label,

                FloatField {
                    value: round(volume.value()),
                    on_change: move |value| {
                        props.on_change.call(Volume::new(value, volume.units()));
                    },
                }
            }

            Select {
                value,
                options: options.read().clone(),
                on_change: move |units: String| {
                    props.on_change.call(volume.convert(VolumeUnits::from(units)));
                },
            }
        }
    }
}
