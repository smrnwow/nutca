use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct BadgeProps {
    text: String,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    rsx! {
        div {
            class: "badge",

            span {
                class: "badge__text",
                "{props.text}",
            }
        }
    }
}
