use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TooltipProps {
    text: String,
    children: Element,
}

#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    rsx! {
        div {
            class: "tooltip tooltip_top",

            div {
                class: "tooltip__target",
                {props.children},
            }

            div {
                class: "tooltip__body",
                {props.text},
            }
        }
    }
}
