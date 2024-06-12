use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ColumnProps {
    children: Element,
}

#[component]
pub fn Column(props: ColumnProps) -> Element {
    rsx! {
        div {
            class: "column",
            {props.children},
        }
    }
}
