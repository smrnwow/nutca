use crate::model::fertilizers::Fertilizer;
use crate::ui::components::utils::icons::Close;
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
        div {
            key: "{props.fertilizer_item.read().0.id()}",
            class: "concentrate-part__fertilizer",

            QuickAction {
                Text {
                    size: "x-small",
                    {props.fertilizer_item.read().0.name()},
                }

                Text {
                    size: "x-small",
                    {props.fertilizer_item.read().1.to_string()},
                }
            }

            div {
                class: "concentrate-part__delete-fertilizer",

                Button {
                    style: "compact",
                    on_click: move |_| {
                        let fertilizer_id = props.fertilizer_item.read().0.id();
                        props.on_delete.call(fertilizer_id);
                    },
                    Close {},
                }
            }
        }
    }
}
