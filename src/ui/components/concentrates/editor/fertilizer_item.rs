use crate::model::solutions::FertilizerWeight;
use crate::ui::components::layout::Row;
use crate::ui::components::utils::icons::Delete;
use crate::ui::components::utils::{Button, QuickAction, Text};
use dioxus::prelude::*;

fn round(value: f64) -> String {
    format!("{:.3}", value)
}

#[derive(Props, PartialEq, Clone)]
pub struct FertilizerItemProps {
    fertilizer: Signal<FertilizerWeight>,
    on_delete: EventHandler<String>,
}

#[component]
pub fn FertilizerItem(props: FertilizerItemProps) -> Element {
    rsx! {
        Row {
            gap: "x-small",
            vertical: "center",

            QuickAction {
                Text {
                    size: "x-small",
                    {props.fertilizer.read().name()},
                }

                Text {
                    size: "x-small",
                    "{round(props.fertilizer.read().weight())}",
                }
            }

            Button {
                style: "compact",
                on_click: move |_| {
                    let fertilizer_id = props.fertilizer.read().id();
                    props.on_delete.call(fertilizer_id);
                },
                Delete {},
            }
        }
    }
}
