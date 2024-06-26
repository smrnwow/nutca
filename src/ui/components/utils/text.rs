use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TextProps {
    size: Option<String>,
    nowrap: Option<bool>,
    bold: Option<bool>,
    children: Element,
}

#[component]
pub fn Text(props: TextProps) -> Element {
    let size = props.size.unwrap_or("small".to_string());

    let nowrap = props.nowrap.unwrap_or(false);

    let bold = props.bold.unwrap_or(false);

    rsx! {
        p {
            class: "text text_{size} text_nowrap-{nowrap} text_bold-{bold}",

            {props.children},
        }
    }
}
