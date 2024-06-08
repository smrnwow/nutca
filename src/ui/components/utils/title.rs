use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TitleProps {
    text: String,
}

#[component]
pub fn Title(props: TitleProps) -> Element {
    rsx! {
        h3 {
            class: "title",
            "{props.text}"
        }
    }
}
