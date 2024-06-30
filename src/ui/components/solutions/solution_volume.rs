use crate::ui::components::layout::Row;
use crate::ui::components::utils::{Label, NumberField};
use crate::ui::components::UnitsSelect;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SolutionVolumeProps {
    volume: Memo<usize>,
    on_volume_update: EventHandler<usize>,
    on_units_change: EventHandler<String>,
}

#[component]
pub fn SolutionVolume(props: SolutionVolumeProps) -> Element {
    rsx! {
        Row {
            gap: "small",
            vertical: "end",

            Label {
                text: "Объем раствора",

                NumberField {
                    value: *props.volume.read(),
                    units: "литр",
                    on_change: props.on_volume_update,
                }
            }

            UnitsSelect {
                on_change: props.on_units_change,
            }
        }
    }
}
