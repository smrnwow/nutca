use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct BadgeProps {
    size: Option<String>,
    state: Option<String>,
    text: String,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let size = props.size.unwrap_or(String::from("medium"));

    let state = props.state.unwrap_or(String::from("default"));

    rsx! {
        div {
            class: "badge badge_size-{size} badge_state-{state}",
            {props.text},
        }
    }
}
