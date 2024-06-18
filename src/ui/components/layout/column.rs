use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ColumnProps {
    gap: Option<String>,
    children: Element,
}

#[component]
pub fn Column(props: ColumnProps) -> Element {
    let gap = props.gap.unwrap_or("large".to_string());

    rsx! {
        div {
            class: "column column_gap-{gap}",
            {props.children},
        }
    }
}
