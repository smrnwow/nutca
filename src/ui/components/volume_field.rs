use crate::model::chemistry::{Volume, VolumeUnits};
use crate::ui::components::layout::Row;
use crate::ui::components::utils::{Label, NumberField, Select};
use dioxus::prelude::*;

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
            (
                VolumeUnits::Millilitres.into(),
                VolumeUnits::Millilitres.label(),
            ),
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

                NumberField {
                    value: volume.value(),
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
