use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct RowProps {
    horizontal: Option<String>,
    vertical: Option<String>,
    gap: Option<String>,
    children: Element,
}

#[component]
pub fn Row(props: RowProps) -> Element {
    let horizontal = props.horizontal.unwrap_or("start".to_string());

    let vertical = props.vertical.unwrap_or("start".to_string());

    let gap = props.gap.unwrap_or("large".to_string());

    rsx! {
        div {
            class: "row row_gap-{gap} row_horizontal-{horizontal} row_vertical-{vertical}",
            {props.children},
        }
    }
}
