use crate::ui::components::utils::Tooltip;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ReferenceTipProps {
    tooltip: Element,
    #[props(default = String::from("bottom-center"))]
    tooltip_position: String,
}

#[component]
pub fn ReferenceTip(props: ReferenceTipProps) -> Element {
    rsx! {
        div {
            class: "reference",

            Tooltip {
                position: props.tooltip_position,
                target: rsx! {
                    span {
                        class: "reference__icon",
                        "i"
                    }
                },
                body: props.tooltip,
            }
        }
    }
}
