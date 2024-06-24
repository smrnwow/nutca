use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ColumnProps {
    gap: Option<String>,
    horizontal: Option<String>,
    vertical: Option<String>,
    children: Element,
}

#[component]
pub fn Column(props: ColumnProps) -> Element {
    let gap = props.gap.unwrap_or("large".to_string());

    let horizontal = props.horizontal.unwrap_or("stretch".to_string());

    let vertical = props.vertical.unwrap_or("start".to_string());

    rsx! {
        div {
            class: "column column_gap-{gap} column_horizontal-{horizontal} column_vertical-{vertical}",
            {props.children},
        }
    }
}
