use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TooltipProps {
    position: Option<String>,
    target: Element,
    body: Element,
}

#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let position = props.position.unwrap_or(String::from("top-center"));

    rsx! {
        div {
            class: "tooltip tooltip_{position}",

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
