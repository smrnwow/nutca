use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TextProps {
    size: Option<String>,
    children: Element,
}

#[component]
pub fn Text(props: TextProps) -> Element {
    let size = props.size.unwrap_or("small".to_string());

    rsx! {
        p {
            class: "text text_{size}",

            {props.children},
        }
    }
}
