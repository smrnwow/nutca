use dioxus::prelude::*;

fn row_class(align: Option<String>) -> String {
    match align {
        Some(align) => format!("row row_align-{}", align),
        None => String::from("row"),
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct RowProps {
    align: Option<String>,
    children: Element,
}

#[component]
pub fn Row(props: RowProps) -> Element {
    rsx! {
        div {
            class: row_class(props.align),
            {props.children},
        }
    }
}
