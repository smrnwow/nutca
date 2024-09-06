use crate::model::fertilizers::Fertilizer;
use crate::ui::components::layout::Row;
use crate::ui::components::utils::icons::Delete;
use crate::ui::components::utils::{Button, QuickAction, Text};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FertilizerItemProps {
    fertilizer_item: Signal<(Fertilizer, f64)>,
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
                    {props.fertilizer_item.read().0.name()},
                }

                Text {
                    size: "x-small",
                    "{props.fertilizer_item.read().1.to_string()} {props.fertilizer_item.read().0.units()}",
                }
            }

            Button {
                style: "compact",
                on_click: move |_| {
                    let fertilizer_id = props.fertilizer_item.read().0.id();
                    props.on_delete.call(fertilizer_id);
                },
                Delete {},
            }
        }
    }
}
