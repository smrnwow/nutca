use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TooltipProps {
    target: Element,
    body: Element,
    #[props(default = String::from("top-center"))]
    position: String,
}

#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    rsx! {
        div {
            class: "tooltip tooltip_{props.position}",

            div {
                class: "tooltip__target",
                {props.target},
            }

            div {
                class: "tooltip__body",
                {props.body},
            }
        }
    }
}
