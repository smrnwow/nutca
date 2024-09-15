use crate::model::fertilizers::FertilizerAmount;
use crate::ui::components::layout::Row;
use crate::ui::components::utils::icons::Delete;
use crate::ui::components::utils::{Button, QuickAction, Text};
use dioxus::prelude::*;

fn round(value: f64) -> String {
    format!("{:.3}", value)
}

#[derive(Props, PartialEq, Clone)]
pub struct FertilizerItemProps {
    fertilizer_amount: Signal<FertilizerAmount>,
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
                    {props.fertilizer_amount.read().fertilizer().name()},
                }

                Text {
                    size: "x-small",
                    "{round(props.fertilizer_amount.read().amount())}",
                }
            }

            Button {
                style: "compact",
                on_click: move |_| {
                    let fertilizer_id = props.fertilizer_amount.read().fertilizer().id();
                    props.on_delete.call(fertilizer_id);
                },
                Delete {},
            }
        }
    }
}
