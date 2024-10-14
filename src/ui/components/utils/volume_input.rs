use super::{Input, Select};
use crate::model::chemistry::{Volume, VolumeUnits};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct VolumeInputProps {
    volume: Memo<Volume>,
    on_change: EventHandler<Volume>,
}

#[component]
pub fn VolumeInput(props: VolumeInputProps) -> Element {
    let volume = *props.volume.read();

    let value = use_memo(move || {
        (
            props.volume.read().units().into(),
            props.volume.read().units().label(),
        )
    });

    let options = use_signal(|| {
        vec![
            (VolumeUnits::Litres.into(), VolumeUnits::Litres.label()),
            (VolumeUnits::Gallons.into(), VolumeUnits::Gallons.label()),
        ]
    });

    rsx! {
        div {
            class: "volume-input",

            Input {
                value: volume.value().to_string(),
                on_change: move |value: String| {
                    let value = value.parse::<usize>().unwrap_or(0);

                    props.on_change.call(Volume::new(value, volume.units()));
                },
            }

            Select {
                value,
                options: options.read().clone(),
                on_change: move |units: String| {
                    props.on_change.call(Volume::new(volume.value(), VolumeUnits::from(units)));
                },
            }
        }
    }
}
