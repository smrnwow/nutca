use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TitleProps {
    size: Option<String>,
    text: String,
}

#[component]
pub fn Title(props: TitleProps) -> Element {
    let size = props.size.unwrap_or(String::from("medium"));

    rsx! {
        h3 {
            class: "title title_size-{size}",
            "{props.text}"
        }
    }
}
