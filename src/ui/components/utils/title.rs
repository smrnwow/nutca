use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct TitleProps {
    id: Option<String>,
    size: Option<String>,
    text: String,
}

#[component]
pub fn Title(props: TitleProps) -> Element {
    let size = props.size.unwrap_or(String::from("medium"));

    let id = props.id.unwrap_or(String::new());

    rsx! {
        h3 {
            id: "{id}",
            class: "title title_size-{size}",
            "{props.text}"
        }
    }
}
