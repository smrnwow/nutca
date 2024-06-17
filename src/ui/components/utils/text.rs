use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TextProps {
    text: String,
}

#[component]
pub fn Text(props: TextProps) -> Element {
    rsx! {
        p {
            class: "text",

            "{props.text}",
        }
    }
}
