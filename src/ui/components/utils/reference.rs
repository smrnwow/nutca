use crate::ui::components::utils::Tooltip;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ReferenceProps {
    display: Signal<bool>,
    tooltip: Element,
    style: Option<String>,
    tooltip_position: Option<String>,
}

#[component]
pub fn Reference(props: ReferenceProps) -> Element {
    let tooltip_position = props.tooltip_position.unwrap_or(String::from("top-right"));

    let style = props.style.unwrap_or(String::from("badge"));

    rsx! {
        div {
            class: "reference reference_display-{*props.display.read()} reference_style-{style}",

            Tooltip {
                position: tooltip_position,
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
